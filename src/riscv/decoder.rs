use super::isa::{InstrType, RVT};

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    /// Instruction Type
    instr: InstrType,

    // Registers
    /// Destination Registers
    rd: Option<u8>,
    /// Register 1
    rs1: Option<u8>,
    /// Register 2
    rs2: Option<u8>,

    /// Shift Amount
    shamt: Option<u8>,

    /// Immediate
    imm: Option<i32>,
}

impl Instruction {
    /// Decode RV32I Instruction
    pub fn new(raw_instr: u32) -> Self {
        let op_code = (raw_instr & 0x0000_007f) as u8;
        let funct3 = ((raw_instr & 0x0000_7000) >> 12) as u8;
        let option_op = ((raw_instr & 0x4000_0000) >> 30) != 0;

        let instr = InstrType::new(op_code, funct3, option_op);

        // Get registers IDs
        let rd = if instr.has_rd() {
            Some(((raw_instr & 0x0000_0f80) >> 7) as u8)
        } else {
            None
        };
        let rs1 = if instr.has_rs1() {
            Some(((raw_instr & 0x000f_8000) >> 15) as u8)
        } else {
            None
        };
        let rs2 = if instr.has_rs2() {
            Some(((raw_instr & 0x01f0_0000) >> 20) as u8)
        } else {
            None
        };

        // Get shift amount
        let shamt = if instr.has_option() {
            Some(((raw_instr & 0x01f0_0000) >> 20) as u8)
        } else {
            None
        };

        // Get immediate
        let imm: Option<i32> = if shamt.is_none() {
            match instr.instr_type {
                RVT::I => Some((raw_instr & 0xfff0_0000) as i32 >> 20),
                RVT::S => Some(
                    (((raw_instr & 0xfe00_0000) >> 20) | ((raw_instr & 0x0000_0780) >> 7)) as i32,
                ),
                RVT::B => Some(
                    (((raw_instr & 0x8000_0000) >> 20) | ((raw_instr & 0x7e00_0000) >> 15)
                        | ((raw_instr & 0x0000_0700) >> 7)
                        | ((raw_instr & 0x0000_0080) << 3)) as i32,
                ),
                RVT::U => Some((raw_instr & 0xffff_0000) as i32),
                RVT::J => Some(
                    (((raw_instr & 0x7fe0_0000) >> 19) | ((raw_instr & 0x0010_0000) >> 8)
                        | (raw_instr & 0x000f_f000)
                        | ((raw_instr & 0x8000_0000) >> 11)) as i32,
                ),
                _ => None,
            }
        } else {
            None
        };

        Instruction {
            instr,
            rd,
            rs1,
            rs2,
            shamt,
            imm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use riscv::*;

    /// Generate the standard test for immediate instructions. Create correct object
    /// and then compare with generated object.
    macro_rules! generate_test {
        (imm, $rd:expr, $rs1:expr, $imm:expr, $op:expr, $option_op:expr) => {
            Instruction {
                rd: Some($rd),
                rs2: None,
                rs1: Some($rs1),
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, $op, $option_op),
            }
        };

        (shift_imm, $rd:expr, $rs1:expr, $shift:expr, $op:expr, $option_op:expr) => {
            Instruction {
                rd: Some($rd),
                rs2: None,
                rs1: Some($rs1),
                imm: None,
                shamt: Some($shift),
                instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, $op, $option_op),
            }
        };

        ($type:tt, $rd:expr, $rs1:expr, $imm:expr, $op:expr, $instr:expr, $option_op:expr) => {
            let final_instr = generate_test!($type, $rd, $rs1, $imm, $op, $option_op);

            let parsed_instr = Instruction::new($instr);
            assert_eq!(parsed_instr, final_instr);
        };
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Immediate Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test ADDI instruction with a positive immediate
    #[test]
    fn addi_pos() {
        // ADDI R4 <= R3 + 15
        generate_test!(imm, 4, 3, 15, 0, 0x00f1_8213, false);
    }

    /// Test ADDI instruction with a negative immediate
    #[test]
    fn addi_neg() {
        // ADDI R4 <= R3 + (-15)
        generate_test!(imm, 4, 3, -15, 0, 0xff11_8213, false);
    }

    /// Test SLTI instruction with a positive immediate
    #[test]
    fn slti_pos() {
        // SLTI R4, R3, 2047
        generate_test!(imm, 4, 3, 2047, 2, 0x7ff1_a213, false);
    }

    /// Test SLTI instruction with a negative immediate
    #[test]
    fn slti_neg() {
        // SLTI R4, R3, -1
        generate_test!(imm, 4, 3, -1, 2, 0xfff1_a213, false);
    }

    /// Test SLTIU instruction with a positive immediate
    #[test]
    fn sltiu_pos() {
        // SLTI R4, R3, 2047
        generate_test!(imm, 4, 3, 2047, 3, 0x7ff1_b213, false);
    }

    /// Test SLTIU instruction with a negative immediate
    #[test]
    fn sltiu_neg() {
        // SLTI R4, R3, -1
        generate_test!(imm, 4, 3, -1, 3, 0xfff1_b213, false);
    }

    /// Test XORI instruction with a positive immediate
    #[test]
    fn xori_pos() {
        // XORI R4, R3, 2047
        generate_test!(imm, 4, 3, 2047, 4, 0x7ff1_c213, false);
    }

    /// Test XORI instruction with a negative immediate
    #[test]
    fn xori_neg() {
        // XORI R4, R3, -1
        generate_test!(imm, 4, 3, -1, 4, 0xfff1_c213, false);
    }

    /// Test ORI instruction with a positive immediate
    #[test]
    fn ori_pos() {
        // ORI R4, R3, 2047
        generate_test!(imm, 4, 3, 2047, 6, 0x7ff1_e213, false);
    }

    /// Test ORI instruction with a negative immediate
    #[test]
    fn ori_neg() {
        // ORI R4, R3, -1
        generate_test!(imm, 4, 3, -1, 6, 0xfff1_e213, false);
    }

    /// Test ANDI instruction with a positive immediate
    #[test]
    fn andi_pos() {
        // ANDI R4, R3, 2047
        generate_test!(imm, 4, 3, 2047, 7, 0x7ff1_f213, false);
    }

    /// Test ANDI instruction with a negative immediate
    #[test]
    fn andi_neg() {
        // ANDI R4, R3, -1
        generate_test!(imm, 4, 3, -1, 7, 0xfff1_f213, false);
    }

    /// Test SLLI instruction with a positive immediate
    #[test]
    fn slli() {
        generate_test!(shift_imm, 4, 3, 4, 1, 0x0041_9213, false);

        // TODO: Remaining 7 bits shouldn't be don't care
        // let parsed_instr = Instruction::new(0x6a41_9213);
        // assert_eq!(parsed_instr, final_instr);
    }

    /// Test SRLI instruction with a positive immediate
    #[test]
    fn srli() {
        // SRLI R4, R3, 5
        generate_test!(shift_imm, 4, 3, 5, 5, 0x0051_d213, false);
    }

    /// Test SRAI instruction with a positive immediate
    #[test]
    fn srai() {
        // SRAI R4, R3, 6
        generate_test!(shift_imm, 4, 3, 6, 5, 0x4061_d213, true);
    }
}
