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
                    ((raw_instr & 0xfe00_0000) as i32 >> 20)
                        | ((raw_instr & 0x0000_0f80) as i32 >> 7),
                ),
                RVT::B => Some(
                    ((raw_instr & 0x8000_0000) as i32 >> 19)
                        | ((raw_instr & 0x7e00_0000) as i32 >> 20)
                        | ((raw_instr & 0x0000_0f00) as i32 >> 7)
                        | (((raw_instr & 0x0000_0080) as i32) << 4),
                ),
                RVT::U => Some((raw_instr & 0xffff_0000) as i32),
                RVT::J => Some(
                    ((raw_instr & 0x7fe0_0000) as i32 >> 20)
                        | ((raw_instr & 0x0010_0000) as i32 >> 9)
                        | (raw_instr & 0x000f_f000) as i32
                        | ((raw_instr & 0x8000_0000) as i32 >> 11),
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

        (load, $rd:expr, $rs1:expr, $imm:expr, $op:expr) => {
            Instruction {
                rd: Some($rd),
                rs2: None,
                rs1: Some($rs1),
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_MEM_LD, $op, false),
            }
        };

        (store, $rs2:expr, $rs1:expr, $imm:expr, $op:expr) => {
            Instruction {
                rd: None,
                rs2: Some($rs2),
                rs1: Some($rs1),
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_MEM_ST, $op, false),
            }
        };

        (branch, $rs2:expr, $rs1:expr, $imm:expr, $op:expr) => {
            Instruction {
                rd: None,
                rs2: Some($rs2),
                rs1: Some($rs1),
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_BR, $op, false),
            }
        };

        (jal, $rsd:expr, $imm:expr) => {
            Instruction {
                rd: Some($rsd),
                rs2: None,
                rs1: None,
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_JAL, 0, false),
            }
        };

        (jalr, $rsd:expr, $rs1:expr, $imm:expr) => {
            Instruction {
                rd: Some($rsd),
                rs2: None,
                rs1: Some($rs1),
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_JALR, 0, false),
            }
        };

        (lui, $rsd:expr, $imm:expr) => {
            Instruction {
                rd: Some($rsd),
                rs2: None,
                rs1: None,
                imm: Some($imm),
                shamt: None,
                instr: InstrType::new(RV32_OP_CODES_LUI, 0, false),
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

        ($type:tt, $rd:expr, $rs1:expr, $imm_or_rs2:expr, $op:expr, $instr:expr) => {
            let final_instr = __create_instruction!($type, $rd, $rs1, $imm_or_rs2, $op);

            let parsed_instr = Instruction::new($instr);
            assert_eq!(parsed_instr, final_instr);
        };

        (jal, $rd:expr, $imm:expr, $instr:expr) => {
            let final_instr = __create_instruction!(jal, $rd, $imm);

            let parsed_instr = Instruction::new($instr);
            assert_eq!(parsed_instr, final_instr);
        };

        (jalr, $rd:expr, $rs1:expr, $imm:expr, $instr:expr) => {
            let final_instr = __create_instruction!(jalr, $rd, $rs1, $imm);

            let parsed_instr = Instruction::new($instr);
            assert_eq!(parsed_instr, final_instr);
        };

        (lui, $rd:expr, $imm:expr, $instr:expr) => {
            let final_instr = __create_instruction!(lui, $rd, $imm);

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
        generate_test!(load, 4, 6, 2, 0, 0x0023_0203);
    }

    /// Test LH detection
    #[test]
    fn lh() {
        // LH R4, 2(R6)
        generate_test!(load, 4, 6, 2, 1, 0x0023_1203);
    }

    /// Test LW detection
    #[test]
    fn lw() {
        // LW R4, 2(R6)
        generate_test!(load, 4, 6, 2, 2, 0x0023_2203);
    }

    /// Test LBU detection
    #[test]
    fn lbu() {
        // LBU R4, 2(R6)
        generate_test!(load, 4, 6, 2, 4, 0x0023_4203);
    }

    /// Test LHU detection
    #[test]
    fn lhu() {
        // LHU R4, 2(R6)
        generate_test!(load, 4, 6, 2, 5, 0x0023_5203);
    }

    // TODO: Make tests for invalid op codes, and valid op codes and invalid
    // functions

    ////////////////////////////////////////////////////////////////////////////////
    // Store Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test SB detection with positive immediate
    #[test]
    fn sb_pos() {
        // SB R4, 2(R6)
        generate_test!(store, 4, 6, 2, 0, 0x0043_0123);
        // SB R4, 17(R6)
        generate_test!(store, 4, 6, 17, 0, 0x0043_08a3);
        // SB R4, 1073(R6)
        generate_test!(store, 4, 6, 1073, 0, 0x4243_08a3);
    }

    #[test]
    fn sb_neg() {
        // SB R4, -2(R6)
        generate_test!(store, 4, 6, -2, 0, 0xfe43_0f23);
        // SB R4, -17(R6)
        generate_test!(store, 4, 6, -17, 0, 0xfe43_07a3);
        // SB R4, -1073(R6)
        generate_test!(store, 4, 6, -1073, 0, 0xbc43_07a3);
    }

    /// Test SH detection with positive immediate
    #[test]
    fn sh() {
        // SH R4, 2(R6)
        generate_test!(store, 4, 6, 2, 1, 0x0043_1123);
    }

    /// Test SW detection with positive immediate
    #[test]
    fn sw() {
        // SW R4, 2(R6)
        generate_test!(store, 4, 6, 2, 2, 0x0043_2123);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Branch Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test BEQ detection
    #[test]
    fn beq_pos() {
        // beqz	a3,8
        generate_test!(branch, 0, 13, 8, 0, 0x0006_8463);
    }

    #[test]
    fn beq_neg() {
        // beq	r8,r23,-28
        generate_test!(branch, 9, 15, -28, 0, 0xfe97_82e3);
    }

    /// Test BNE detection
    #[test]
    fn bne() {
        // bnez	a1,-20
        generate_test!(branch, 0, 11, -20, 1, 0xfe05_96e3);
    }

    /// Test BLT detection
    #[test]
    fn blt() {
        // bltz	a1,20
        generate_test!(branch, 0, 11, 20, 4, 0x0005_ca63);
    }

    /// Test BGE detection
    #[test]
    fn bge() {
        // bgez	a0,100006d0
        generate_test!(branch, 0, 10, -16, 5, 0xfe05_58e3);
    }

    /// Test BLTU detection
    #[test]
    fn bltu() {
        // bltu	a0,a1,-8
        generate_test!(branch, 11, 10, -8, 6, 0xfeb5_6ce3);
    }

    /// Test BGEU detection
    #[test]
    fn bgeu() {
        // bleu	a1,a0,10000028
        generate_test!(branch, 11, 10, 16, 7, 0x00b5_7863);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Jump Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test JALR detection
    #[test]
    fn jalr() {
        // jr	t0
        generate_test!(jalr, 0, 5, 0, 0x0002_8067);
        // ret
        generate_test!(jalr, 0, 1, 0, 0x0000_8067);
    }

    /// Test JAL detection
    #[test]
    fn jal() {
        // jal	ra,10000648
        generate_test!(jal, 1, -136, 0xf79f_f0ef);
        // jal	ra,10000648
        generate_test!(jal, 1, -160, 0xf61f_f0ef);
        // jal	ra,10000648
        generate_test!(jal, 1, -112, 0xf91f_f0ef);
        // jal	ra,10000648
        generate_test!(jal, 1, -76, 0xfb5f_f0ef);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // LUI Instruction Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test LUI detection
    #[test]
    fn lui() {
        // lui	a4,0xdead0
        generate_test!(lui, 14, 0xdead << 16, 0xdead_0737);
        // lui	a5,0x40000
        generate_test!(lui, 15, 0x4000 << 16, 0x4000_07b7);
        // lui	a5,0x10000
        generate_test!(lui, 15, 0x1000 << 16, 0x1000_07b7);
    }
}
