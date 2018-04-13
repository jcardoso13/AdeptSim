//! The RISC-V Instruction Set
use super::*;

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
            RV32_OP_CODES_LUI => RVT::U,
            // AUIPC
            RV32_OP_CODES_AUIPC => RVT::U,
            // Jumps
            RV32_OP_CODES_JAL => RVT::J,
            RV32_OP_CODES_JALR => RVT::I,
            // Branches
            RV32_OP_CODES_BR => RVT::B,
            // Loads
            RV32_OP_CODES_MEM_LD => RVT::I,
            // Stores
            RV32_OP_CODES_MEM_ST => RVT::S,
            // Register operations
            RV32_OP_CODES_ARITH_REG => RVT::R,
            // Immediate operations
            RV32_OP_CODES_ARITH_IMM => RVT::I,
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
            RV32_OP_CODES_LUI => RV32I::LUI,
            // AUIPC
            RV32_OP_CODES_AUIPC => RV32I::AUIPC,
            // Jumps
            RV32_OP_CODES_JAL => RV32I::JAL,
            RV32_OP_CODES_JALR => match funct3 {
                0 => RV32I::JALR,
                _ => RV32I::Invalid,
            },
            // Branches
            RV32_OP_CODES_BR => match funct3 {
                0 => RV32I::BEQ,
                1 => RV32I::BNE,
                4 => RV32I::BLT,
                5 => RV32I::BGE,
                6 => RV32I::BLTU,
                7 => RV32I::BGEU,
                _ => RV32I::Invalid,
            },
            // Loads
            RV32_OP_CODES_MEM_LD => match funct3 {
                0 => RV32I::LB,
                1 => RV32I::LH,
                2 => RV32I::LW,
                4 => RV32I::LBU,
                5 => RV32I::LHU,
                _ => RV32I::Invalid,
            },
            // Stores
            RV32_OP_CODES_MEM_ST => match funct3 {
                0 => RV32I::SB,
                1 => RV32I::SH,
                2 => RV32I::SW,
                _ => RV32I::Invalid,
            },
            // Register operations
            RV32_OP_CODES_ARITH_REG => match funct3 {
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
            RV32_OP_CODES_ARITH_IMM => match funct3 {
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

    macro_rules! generate_test {
        ($type:expr, $op:expr, $op_code:expr, $funct:expr, $option_op:expr) => {{
            let final_instr_type = InstrType {
                instr_type: $type,
                instr_op: $op,
            };

            let parsed_instr_type = InstrType::new($op_code, $funct, $option_op);
            assert_eq!(parsed_instr_type, final_instr_type);
            let parsed_instr_type = InstrType::new($op_code, $funct, !$option_op);
            assert_eq!(parsed_instr_type, final_instr_type);
        }};
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Immediate Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test ADDI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn addi() {
        generate_test!(RVT::I, RV32I::ADDI, RV32_OP_CODES_ARITH_IMM, 0, false);
    }

    /// Test SLTI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn slti() {
        generate_test!(RVT::I, RV32I::SLTI, RV32_OP_CODES_ARITH_IMM, 2, false);
    }

    /// Test SLTIU detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn sltiu() {
        generate_test!(RVT::I, RV32I::SLTIU, RV32_OP_CODES_ARITH_IMM, 3, false);
    }

    /// Test XORI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn xori() {
        generate_test!(RVT::I, RV32I::XORI, RV32_OP_CODES_ARITH_IMM, 4, false);
    }

    /// Test ORI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn ori() {
        generate_test!(RVT::I, RV32I::ORI, RV32_OP_CODES_ARITH_IMM, 6, false);
    }

    /// Test ANDI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn andi() {
        generate_test!(RVT::I, RV32I::ANDI, RV32_OP_CODES_ARITH_IMM, 7, false);
    }

    /// Test SLLI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn slli() {
        generate_test!(RVT::I, RV32I::SLLI, RV32_OP_CODES_ARITH_IMM, 1, false);
    }

    /// Test SRLI detection
    /// When creating the InstrType instance the third argument selects between
    /// the logical or arithmetic right shift.
    #[test]
    fn srli() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SRLI,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_ARITH_IMM, 5, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SRAI detection
    /// When creating the InstrType instance the third argument selects between
    /// the logical or arithmetic right shift.
    #[test]
    fn srai() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::SRAI,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_ARITH_IMM, 5, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Register Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test ADD detection
    #[test]
    fn add() {
        let final_instr_type = InstrType {
            instr_type: RVT::R,
            instr_op: RV32I::ADD,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_ARITH_REG, 0, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SUB detection
    #[test]
    fn sub() {
        let final_instr_type = InstrType {
            instr_type: RVT::R,
            instr_op: RV32I::SUB,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_ARITH_REG, 0, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SLL detection
    #[test]
    fn sll() {
        generate_test!(RVT::R, RV32I::SLL, RV32_OP_CODES_ARITH_REG, 1, false);
    }

    /// Test SLT detection
    #[test]
    fn slt() {
        generate_test!(RVT::R, RV32I::SLT, RV32_OP_CODES_ARITH_REG, 2, false);
    }

    /// Test SLTU detection
    #[test]
    fn sltu() {
        generate_test!(RVT::R, RV32I::SLTU, RV32_OP_CODES_ARITH_REG, 3, false);
    }

    /// Test XOR detection
    #[test]
    fn xor() {
        generate_test!(RVT::R, RV32I::XOR, RV32_OP_CODES_ARITH_REG, 4, false);
    }

    /// Test SRL detection
    #[test]
    fn srl() {
        let final_instr_type = InstrType {
            instr_type: RVT::R,
            instr_op: RV32I::SRL,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_ARITH_REG, 5, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test SRA detection
    #[test]
    fn sra() {
        let final_instr_type = InstrType {
            instr_type: RVT::R,
            instr_op: RV32I::SRA,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_ARITH_REG, 5, true);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test OR detection
    #[test]
    fn or() {
        generate_test!(RVT::R, RV32I::OR, RV32_OP_CODES_ARITH_REG, 6, false);
    }

    /// Test AND detection
    #[test]
    fn and() {
        generate_test!(RVT::R, RV32I::AND, RV32_OP_CODES_ARITH_REG, 7, false);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Load Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test LB detection
    #[test]
    fn lb() {
        generate_test!(RVT::I, RV32I::LB, RV32_OP_CODES_MEM_LD, 0, false);
    }

    /// Test LH detection
    #[test]
    fn lh() {
        generate_test!(RVT::I, RV32I::LH, RV32_OP_CODES_MEM_LD, 1, false);
    }

    /// Test LW detection
    #[test]
    fn lw() {
        generate_test!(RVT::I, RV32I::LW, RV32_OP_CODES_MEM_LD, 2, false);
    }

    /// Test LBU detection
    #[test]
    fn lbu() {
        generate_test!(RVT::I, RV32I::LBU, RV32_OP_CODES_MEM_LD, 4, false);
    }

    /// Test LHU detection
    #[test]
    fn lhu() {
        generate_test!(RVT::I, RV32I::LHU, RV32_OP_CODES_MEM_LD, 5, false);
    }

    /// Test Invalid Load Instruction
    #[test]
    fn invalid_load() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::Invalid,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_LD, 3, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_LD, 3, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_LD, 6, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_LD, 6, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_LD, 7, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_LD, 7, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Store Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test SB detection
    #[test]
    fn sb() {
        generate_test!(RVT::S, RV32I::SB, RV32_OP_CODES_MEM_ST, 0, false);
    }

    /// Test SH detection
    #[test]
    fn sh() {
        generate_test!(RVT::S, RV32I::SH, RV32_OP_CODES_MEM_ST, 1, false);
    }

    /// Test SW detection
    #[test]
    fn sw() {
        generate_test!(RVT::S, RV32I::SW, RV32_OP_CODES_MEM_ST, 2, false);
    }

    /// Test Invalid Store Instruction
    #[test]
    fn invalid_store() {
        let final_instr_type = InstrType {
            instr_type: RVT::S,
            instr_op: RV32I::Invalid,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 3, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 3, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 4, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 4, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 5, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 5, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 6, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 6, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 7, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_MEM_ST, 7, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Branch Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test BEQ detection
    #[test]
    fn beq() {
        generate_test!(RVT::B, RV32I::BEQ, RV32_OP_CODES_BR, 0, false);
    }

    /// Test BNE detection
    #[test]
    fn bne() {
        generate_test!(RVT::B, RV32I::BNE, RV32_OP_CODES_BR, 1, false);
    }

    /// Test BLT detection
    #[test]
    fn blt() {
        generate_test!(RVT::B, RV32I::BLT, RV32_OP_CODES_BR, 4, false);
    }

    /// Test BGE detection
    #[test]
    fn bge() {
        generate_test!(RVT::B, RV32I::BGE, RV32_OP_CODES_BR, 5, false);
    }

    /// Test BLTU detection
    #[test]
    fn bltu() {
        generate_test!(RVT::B, RV32I::BLTU, RV32_OP_CODES_BR, 6, false);
    }

    /// Test BGEU detection
    #[test]
    fn bgeu() {
        generate_test!(RVT::B, RV32I::BGEU, RV32_OP_CODES_BR, 7, false);
    }

    /// Test Invalid branch instruction
    #[test]
    fn invalid_branch() {
        let final_instr_type = InstrType {
            instr_type: RVT::B,
            instr_op: RV32I::Invalid,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_BR, 2, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_BR, 2, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_BR, 3, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_BR, 3, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Jump Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test JALR detection
    #[test]
    fn jalr() {
        generate_test!(RVT::I, RV32I::JALR, RV32_OP_CODES_JALR, 0, false);
    }

    /// Test JAL detection
    #[test]
    fn jal() {
        let final_instr_type = InstrType {
            instr_type: RVT::J,
            instr_op: RV32I::JAL,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 0, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 0, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 1, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 1, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 2, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 2, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 3, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 3, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 4, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 4, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 5, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 5, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 6, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 6, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 7, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JAL, 7, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    /// Test invalid jump
    #[test]
    fn invalid_jalr() {
        let final_instr_type = InstrType {
            instr_type: RVT::I,
            instr_op: RV32I::Invalid,
        };

        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 1, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 1, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 2, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 2, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 3, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 3, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 4, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 4, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 5, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 5, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 6, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 6, false);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 7, true);
        assert_eq!(parsed_instr_type, final_instr_type);
        let parsed_instr_type = InstrType::new(RV32_OP_CODES_JALR, 7, false);
        assert_eq!(parsed_instr_type, final_instr_type);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // LUI Instruction Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test LUI detection
    #[test]
    fn lui() {
        generate_test!(RVT::U, RV32I::LUI, RV32_OP_CODES_LUI, 0, false);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // LUI Instruction Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test AUIPC detection
    #[test]
    fn auipc() {
        generate_test!(RVT::U, RV32I::AUIPC, RV32_OP_CODES_AUIPC, 0, false);
    }
}
