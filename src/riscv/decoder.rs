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

    /// Test ADDI instruction with a positive immediate
    #[test]
    fn addi_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(15),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 0, false),
        };

        // ADDI R4 <= R3 + 15
        let parsed_instr = Instruction::new(0x00f1_8213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test ADDI instruction with a negative immediate
    #[test]
    fn addi_neg() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(-15),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 0, false),
        };

        // ADDI R4 <= R3 + (-15)
        let parsed_instr = Instruction::new(0xff11_8213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SLTI instruction with a positive immediate
    #[test]
    fn slti_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(2047),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 2, false),
        };

        // SLTI R4, R3, 2047
        let parsed_instr = Instruction::new(0x7ff1_a213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SLTI instruction with a negative immediate
    #[test]
    fn slti_neg() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(-1),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 2, false),
        };

        // SLTI R4, R3, -1
        let parsed_instr = Instruction::new(0xfff1_a213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SLTIU instruction with a positive immediate
    #[test]
    fn sltiu_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(2047),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 3, false),
        };

        // SLTI R4, R3, 2047
        let parsed_instr = Instruction::new(0x7ff1_b213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SLTIU instruction with a negative immediate
    #[test]
    fn sltiu_neg() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(-1),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 3, false),
        };

        // SLTI R4, R3, -1
        let parsed_instr = Instruction::new(0xfff1_b213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test XORI instruction with a positive immediate
    #[test]
    fn xori_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(2047),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 4, false),
        };

        // XORI R4, R3, 2047
        let parsed_instr = Instruction::new(0x7ff1_c213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test XORI instruction with a negative immediate
    #[test]
    fn xori_neg() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(-1),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 4, false),
        };

        // XORI R4, R3, -1
        let parsed_instr = Instruction::new(0xfff1_c213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test ORI instruction with a positive immediate
    #[test]
    fn ori_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(2047),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 6, false),
        };

        // ORI R4, R3, 2047
        let parsed_instr = Instruction::new(0x7ff1_e213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test ORI instruction with a negative immediate
    #[test]
    fn ori_neg() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(-1),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 6, false),
        };

        // ORI R4, R3, -1
        let parsed_instr = Instruction::new(0xfff1_e213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test ANDI instruction with a positive immediate
    #[test]
    fn andi_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(2047),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 7, false),
        };

        // ORI R4, R3, 2047
        let parsed_instr = Instruction::new(0x7ff1_f213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test ANDI instruction with a negative immediate
    #[test]
    fn andi_neg() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(-1),
            shamt: None,
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 7, false),
        };

        // ORI R4, R3, -1
        let parsed_instr = Instruction::new(0xfff1_f213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SLLI instruction with a positive immediate
    #[test]
    fn slli() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: None,
            shamt: Some(4),
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 1, false),
        };

        // SLLI R4, R3, 4
        let parsed_instr = Instruction::new(0x0041_9213);
        assert_eq!(parsed_instr, final_instr);
        // Remaining 7 bits should be don't care
        let parsed_instr = Instruction::new(0x6a41_9213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SRLI instruction with a positive immediate
    #[test]
    fn srli() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: None,
            shamt: Some(5),
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 5, false),
        };

        // ORI R4, R3, 2047
        let parsed_instr = Instruction::new(0x0051_d213);
        assert_eq!(parsed_instr, final_instr);
        // Remaining 7 bits should not be don't care
        let parsed_instr = Instruction::new(0x0051_d213);
        assert_eq!(parsed_instr, final_instr);
    }

    /// Test SRAI instruction with a positive immediate
    #[test]
    fn srai() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: None,
            shamt: Some(6),
            instr: InstrType::new(RV32_OP_CODES_ARITH_IMM, 5, true),
        };

        // ORI R4, R3, 2047
        let parsed_instr = Instruction::new(0x4061_d213);
        assert_eq!(parsed_instr, final_instr);
    }
}
