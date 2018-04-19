use super::isa::{InstrType, RVT};
use std::cmp::PartialEq;

#[derive(Debug, Eq)]
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

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        // Invalid instructions are always the equal regardless of the remaining
        // parameters
        if self.instr.instr_type == RVT::Invalid && other.instr.instr_type == RVT::Invalid {
            return true;
        }

        if self.instr == other.instr {
            if self.rd != other.rd {
                return false;
            }
            if self.rs1 != other.rs1 {
                return false;
            }
            if self.rs2 != other.rs2 {
                return false;
            }
            if self.imm != other.imm {
                return false;
            }
            if self.shamt != other.shamt {
                return false;
            }

            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use riscv::*;

    /// Create Instruction object with specific instruction type
    macro_rules! __create_instruction {
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

        (register, $rd:expr, $rs1:expr, $rs2:expr, $op:expr, $option_op:expr) => {
            Instruction {
                rd: Some($rd),
                rs2: Some($rs2),
                rs1: Some($rs1),
                imm: None,
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_ARITH_REG, $op, $option_op),
            }
        };

        (load, $rd:expr, $rs1:expr, $imm:expr, $op:expr, $option_op:expr) => {
            Instruction {
                rd: Some($rd),
                rs2: None,
                rs1: Some($rs1),
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_MEM_LD, $op, false),
            }
        };
    }

    /// Generate the standard test every instruction. Create correct object and
    /// then compare with generated object.
    macro_rules! generate_test {
        (
            $type:tt, $rd:expr, $rs1:expr, $imm_or_rs2:expr, $op:expr, $instr:expr, $option_op:expr
        ) => {
            let final_instr =
                __create_instruction!($type, $rd, $rs1, $imm_or_rs2, $op, $option_op);

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

    ////////////////////////////////////////////////////////////////////////////////
    // Register Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test ADD decode
    #[test]
    fn add() {
        // ADD R4, R6, R2
        generate_test!(register, 4, 6, 2, 0, 0x0023_0233, false);
    }

    /// Test SUB decode
    #[test]
    fn sub() {
        // SUB R4, R6, R2
        generate_test!(register, 4, 6, 2, 0, 0x4023_0233, true);
    }

    /// Test SLL decode
    #[test]
    fn sll() {
        // SLL R4, R6, R2
        generate_test!(register, 4, 6, 2, 1, 0x0023_1233, false);
    }

    /// Test SLT decode
    #[test]
    fn slt() {
        // SLT R4, R6, R2
        generate_test!(register, 4, 6, 2, 2, 0x0023_2233, false);
    }

    /// Test SLTU decode
    #[test]
    fn sltu() {
        // SLTU R4, R6, R2
        generate_test!(register, 4, 6, 2, 3, 0x0023_3233, false);
    }

    /// Test XOR decode
    #[test]
    fn xor() {
        // XOR R4, R6, R2
        generate_test!(register, 4, 6, 2, 4, 0x0023_4233, false);
    }

    /// Test SRL decode
    #[test]
    fn srl() {
        // SRL R4, R6, R2
        generate_test!(register, 4, 6, 2, 5, 0x0023_5233, false);
    }

    /// Test SRA decode
    #[test]
    fn sra() {
        // SRA R4, R6, R2
        generate_test!(register, 4, 6, 2, 5, 0x4023_5233, true);
    }

    /// Test OR decode
    #[test]
    fn or() {
        // OR R4, R6, R2
        generate_test!(register, 4, 6, 2, 6, 0x0023_6233, false);
    }

    /// Test AND decode
    #[test]
    fn and() {
        // AND R4, R6, R2
        generate_test!(register, 4, 6, 2, 7, 0x0023_7233, false);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Load Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test LB detection
    #[test]
    fn lb() {
        // LB R4, 2(R6)
        generate_test!(load, 4, 6, 2, 0, 0x0023_0203, false);
    }

    /// Test LH detection
    #[test]
    fn lh() {
        // LH R4, 2(R6)
        generate_test!(load, 4, 6, 2, 1, 0x0023_1203, false);
    }

    /// Test LW detection
    #[test]
    fn lw() {
        // LW R4, 2(R6)
        generate_test!(load, 4, 6, 2, 2, 0x0023_2203, false);
    }

    /// Test LBU detection
    #[test]
    fn lbu() {
        // LBU R4, 2(R6)
        generate_test!(load, 4, 6, 2, 4, 0x0023_4203, false);
    }

    /// Test LHU detection
    #[test]
    fn lhu() {
        // LHU R4, 2(R6)
        generate_test!(load, 4, 6, 2, 5, 0x0023_5203, false);
    }

    // TODO: Make tests for invalid op codes, and valid op codes and invalid
    // functions
}
