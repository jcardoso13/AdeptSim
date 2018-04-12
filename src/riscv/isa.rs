//! The RISC-V Instruction Set

#[derive(Debug, Eq, PartialEq)]
pub struct InstrType {
    pub instr_type: RVT,
    instr_op: RV32I,
}

impl InstrType {
    pub fn new(op_code: u8, funct3: u8, option_op: bool) -> Self {
        InstrType {
            instr_type: RVT::new(op_code),
            instr_op: RV32I::new(op_code, funct3, option_op),
        }
    }

    /// Check if instruction has an option type
    pub fn has_option(&self) -> bool {
        self.instr_op == RV32I::SLLI || self.instr_op == RV32I::SRLI || self.instr_op == RV32I::SRAI
    }

    /// Check if instruction has a destination register
    pub fn has_rd(&self) -> bool {
        self.instr_type == RVT::R || self.instr_type == RVT::I || self.instr_type == RVT::U
            || self.instr_type == RVT::J
    }

    /// Check if instruction has a register source 1
    pub fn has_rs1(&self) -> bool {
        self.instr_type == RVT::R || self.instr_type == RVT::I || self.instr_type == RVT::S
            || self.instr_type == RVT::B
    }

    /// Check if instruction has a register source 2
    pub fn has_rs2(&self) -> bool {
        self.instr_type == RVT::R || self.instr_type == RVT::S || self.instr_type == RVT::B
    }
}

// Instruction OP codes
const RV32_ARITH_IMM_OP_CODE: u8 = 0x13;
const RV32_ARITH_REG_OP_CODE: u8 = 0x33;
const RV32_MEM_LD_OP_CODE: u8 = 0x03;
const RV32_MEM_ST_OP_CODE: u8 = 0x43;
const RV32_BR_OP_CODE: u8 = 0x63;
const RV32_JALR_OP_CODE: u8 = 0x67;
const RV32_JAL_OP_CODE: u8 = 0x6f;
const RV32_AUIPC_OP_CODE: u8 = 0x1f;
const RV32_LUI_OP_CODE: u8 = 0x3f;

/// Instruction Register Types
#[derive(Debug, Eq, PartialEq)]
pub enum RVT {
    /// Register Type
    R,
    /// Immediate Type
    I,
    /// Story Type
    S,
    /// Branch Type
    B,
    /// Upper Type
    U,
    /// Jump Type
    J,
    /// Invalide Type
    Invalid,
}

impl RVT {
    /// Translate the OP code and its function into an Instruction type enum
    fn new(op_code: u8) -> Self {
        match op_code {
            // LUI
            RV32_LUI_OP_CODE => RVT::U,
            // AUIPC
            RV32_AUIPC_OP_CODE => RVT::U,
            // Jumps
            RV32_JAL_OP_CODE => RVT::J,
            RV32_JALR_OP_CODE => RVT::I,
            // Branches
            RV32_BR_OP_CODE => RVT::B,
            // Loads
            RV32_MEM_LD_OP_CODE => RVT::I,
            // Stores
            RV32_MEM_ST_OP_CODE => RVT::S,
            // Register operations
            RV32_ARITH_REG_OP_CODE => RVT::R,
            // Immediate operations
            RV32_ARITH_IMM_OP_CODE => RVT::I,
            _ => RVT::Invalid,
        }
    }
}

/// RISC-V 32-bit ISA
#[derive(Debug, Eq, PartialEq)]
pub enum RV32I {
    //////////////
    // Arithmetic
    //////////////
    // Immediate
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
    SLLI,
    SRLI,
    SRAI,
    // Register 2 Register
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND,

    //////////////
    // Memory
    //////////////
    // Load
    LB,
    LH,
    LW,
    LBU,
    LHU,
    // Store
    SB,
    SH,
    SW,

    //////////////
    // Control
    //////////////
    // Jumps
    JAL,
    JALR,
    // Branches
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,

    LUI,
    AUIPC,

    Invalid,
}

impl RV32I {
    /// Translate the OP code and its function into an Instruction enum
    fn new(op_code: u8, funct3: u8, option_op: bool) -> Self {
        match op_code {
            // LUI
            RV32_LUI_OP_CODE => RV32I::LUI,
            // AUIPC
            RV32_AUIPC_OP_CODE => RV32I::AUIPC,
            // Jumps
            RV32_JAL_OP_CODE => RV32I::JAL,
            RV32_JALR_OP_CODE => match funct3 {
                0 => RV32I::JALR,
                _ => RV32I::Invalid,
            },
            // Branches
            RV32_BR_OP_CODE => match funct3 {
                0 => RV32I::BEQ,
                1 => RV32I::BNE,
                4 => RV32I::BLT,
                5 => RV32I::BGE,
                6 => RV32I::BLTU,
                7 => RV32I::BGEU,
                _ => RV32I::Invalid,
            },
            // Loads
            RV32_MEM_LD_OP_CODE => match funct3 {
                0 => RV32I::LB,
                1 => RV32I::LH,
                2 => RV32I::LW,
                4 => RV32I::LBU,
                5 => RV32I::LHU,
                _ => RV32I::Invalid,
            },
            // Stores
            RV32_MEM_ST_OP_CODE => match funct3 {
                0 => RV32I::SB,
                1 => RV32I::SH,
                2 => RV32I::SW,
                _ => RV32I::Invalid,
            },
            // Register operations
            RV32_ARITH_REG_OP_CODE => match funct3 {
                0 => {
                    if !option_op {
                        RV32I::ADD
                    } else {
                        RV32I::SUB
                    }
                }
                1 => RV32I::SLL,
                2 => RV32I::SLT,
                3 => RV32I::SLTU,
                4 => RV32I::XOR,
                5 => {
                    if option_op {
                        RV32I::SRA
                    } else {
                        RV32I::SRL
                    }
                }
                6 => RV32I::OR,
                7 => RV32I::AND,
                _ => RV32I::Invalid,
            },
            // Immediate operations
            RV32_ARITH_IMM_OP_CODE => match funct3 {
                0 => RV32I::ADDI,
                1 => RV32I::SLLI,
                2 => RV32I::SLTI,
                3 => RV32I::SLTIU,
                4 => RV32I::XORI,
                5 => {
                    if option_op {
                        RV32I::SRAI
                    } else {
                        RV32I::SRLI
                    }
                }
                6 => RV32I::ORI,
                7 => RV32I::ANDI,
                _ => RV32I::Invalid,
            },
            _ => RV32I::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test ADDI detection
    #[test]
    fn addi() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::ADDI,
        };

        let parsed_instr_type = InstrType::new(0x13, 0, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 0, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SLTI detection
    #[test]
    fn slti() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SLTI,
        };

        let parsed_instr_type = InstrType::new(0x13, 2, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 2, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SLTIU detection
    #[test]
    fn sltiu() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SLTIU,
        };

        let parsed_instr_type = InstrType::new(0x13, 3, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 3, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test XORI detection
    #[test]
    fn xori() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::XORI,
        };

        let parsed_instr_type = InstrType::new(0x13, 4, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 4, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test ORI detection
    #[test]
    fn ori() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::ORI,
        };

        let parsed_instr_type = InstrType::new(0x13, 6, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 6, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test ANDI detection
    #[test]
    fn andi() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::ANDI,
        };

        let parsed_instr_type = InstrType::new(0x13, 7, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 7, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SLLI detection
    #[test]
    fn slli() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SLLI,
        };

        let parsed_instr_type = InstrType::new(0x13, 1, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(0x13, 1, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SRLI detection
    #[test]
    fn srli() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SRLI,
        };

        let parsed_instr_type = InstrType::new(0x13, 5, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SRAI detection
    #[test]
    fn srai() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SRAI,
        };

        let parsed_instr_type = InstrType::new(0x13, 5, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }
}
