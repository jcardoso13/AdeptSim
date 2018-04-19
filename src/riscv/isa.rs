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
    /// Store Type
    S,
    /// Branch Type
    B,
    /// Upper Type
    U,
    /// Jump Type
    J,
    /// Invalid Type
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

    macro_rules! __create_instrtype {
        // Create instruction type
        ($type:expr, $op:expr) => {
            InstrType {
                instr_type: $type,
                instr_op: $op,
            }
        };
    }

    /// Generate the standard test for most instructions. Create correct object
    /// and then compare with generated object.
    macro_rules! generate_test {
        // Create test for several functions
        ($type:expr, $op:expr, $op_code:expr, [ $($x:expr),+ ]) => {{
            let final_instr_type = __create_instrtype!($type, $op);

            $(
                let parsed_instr_type = InstrType::new($op_code, $x, false);
                assert_eq!(parsed_instr_type, final_instr_type);
                let parsed_instr_type = InstrType::new($op_code, $x, true);
                assert_eq!(parsed_instr_type, final_instr_type);
            )*
        }};

        // Create test for a single function
        ($type:expr, $op:expr, $op_code:expr, $funct:expr) => {{
            generate_test!($type, $op, $op_code, [$funct]);
        }};

        // Create test for a single function with a specific option_op
        ($type:expr, $op:expr, $op_code:expr, $funct:expr, $option_op:expr) => {{
            let final_instr_type = __create_instrtype!($type, $op);

            let parsed_instr_type = InstrType::new($op_code, $funct, $option_op);
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
        generate_test!(RVT::I, RV32I::ADDI, RV32_OP_CODES_ARITH_IMM, 0);
    }

    /// Test SLTI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn slti() {
        generate_test!(RVT::I, RV32I::SLTI, RV32_OP_CODES_ARITH_IMM, 2);
    }

    /// Test SLTIU detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn sltiu() {
        generate_test!(RVT::I, RV32I::SLTIU, RV32_OP_CODES_ARITH_IMM, 3);
    }

    /// Test XORI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn xori() {
        generate_test!(RVT::I, RV32I::XORI, RV32_OP_CODES_ARITH_IMM, 4);
    }

    /// Test ORI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn ori() {
        generate_test!(RVT::I, RV32I::ORI, RV32_OP_CODES_ARITH_IMM, 6);
    }

    /// Test ANDI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn andi() {
        generate_test!(RVT::I, RV32I::ANDI, RV32_OP_CODES_ARITH_IMM, 7);
    }

    /// Test SLLI detection
    /// When creating the InstrType instance the third argument for this
    /// instruction is don't care.
    #[test]
    fn slli() {
        generate_test!(RVT::I, RV32I::SLLI, RV32_OP_CODES_ARITH_IMM, 1);
    }

    /// Test SRLI detection
    /// When creating the InstrType instance the third argument selects between
    /// the logical or arithmetic right shift.
    #[test]
    fn srli() {
        generate_test!(RVT::I, RV32I::SRLI, RV32_OP_CODES_ARITH_IMM, 5, false);
    }

    /// Test SRAI detection
    /// When creating the InstrType instance the third argument selects between
    /// the logical or arithmetic right shift.
    #[test]
    fn srai() {
        generate_test!(RVT::I, RV32I::SRAI, RV32_OP_CODES_ARITH_IMM, 5, true);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Register Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test ADD detection
    #[test]
    fn add() {
        generate_test!(RVT::R, RV32I::ADD, RV32_OP_CODES_ARITH_REG, 0, false);
    }

    /// Test SUB detection
    #[test]
    fn sub() {
        generate_test!(RVT::R, RV32I::SUB, RV32_OP_CODES_ARITH_REG, 0, true);
    }

    /// Test SLL detection
    #[test]
    fn sll() {
        generate_test!(RVT::R, RV32I::SLL, RV32_OP_CODES_ARITH_REG, 1);
    }

    /// Test SLT detection
    #[test]
    fn slt() {
        generate_test!(RVT::R, RV32I::SLT, RV32_OP_CODES_ARITH_REG, 2);
    }

    /// Test SLTU detection
    #[test]
    fn sltu() {
        generate_test!(RVT::R, RV32I::SLTU, RV32_OP_CODES_ARITH_REG, 3);
    }

    /// Test XOR detection
    #[test]
    fn xor() {
        generate_test!(RVT::R, RV32I::XOR, RV32_OP_CODES_ARITH_REG, 4);
    }

    /// Test SRL detection
    #[test]
    fn srl() {
        generate_test!(RVT::R, RV32I::SRL, RV32_OP_CODES_ARITH_REG, 5, false);
    }

    /// Test SRA detection
    #[test]
    fn sra() {
        generate_test!(RVT::R, RV32I::SRA, RV32_OP_CODES_ARITH_REG, 5, true);
    }

    /// Test OR detection
    #[test]
    fn or() {
        generate_test!(RVT::R, RV32I::OR, RV32_OP_CODES_ARITH_REG, 6);
    }

    /// Test AND detection
    #[test]
    fn and() {
        generate_test!(RVT::R, RV32I::AND, RV32_OP_CODES_ARITH_REG, 7);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Load Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test LB detection
    #[test]
    fn lb() {
        generate_test!(RVT::I, RV32I::LB, RV32_OP_CODES_MEM_LD, 0);
    }

    /// Test LH detection
    #[test]
    fn lh() {
        generate_test!(RVT::I, RV32I::LH, RV32_OP_CODES_MEM_LD, 1);
    }

    /// Test LW detection
    #[test]
    fn lw() {
        generate_test!(RVT::I, RV32I::LW, RV32_OP_CODES_MEM_LD, 2);
    }

    /// Test LBU detection
    #[test]
    fn lbu() {
        generate_test!(RVT::I, RV32I::LBU, RV32_OP_CODES_MEM_LD, 4);
    }

    /// Test LHU detection
    #[test]
    fn lhu() {
        generate_test!(RVT::I, RV32I::LHU, RV32_OP_CODES_MEM_LD, 5);
    }

    /// Test Invalid Load Instruction
    #[test]
    fn invalid_load() {
        generate_test!(RVT::I, RV32I::Invalid, RV32_OP_CODES_MEM_LD, [3, 6, 7]);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Store Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test SB detection
    #[test]
    fn sb() {
        generate_test!(RVT::S, RV32I::SB, RV32_OP_CODES_MEM_ST, 0);
    }

    /// Test SH detection
    #[test]
    fn sh() {
        generate_test!(RVT::S, RV32I::SH, RV32_OP_CODES_MEM_ST, 1);
    }

    /// Test SW detection
    #[test]
    fn sw() {
        generate_test!(RVT::S, RV32I::SW, RV32_OP_CODES_MEM_ST, 2);
    }

    /// Test Invalid Store Instruction
    #[test]
    fn invalid_store() {
        generate_test!(
            RVT::S,
            RV32I::Invalid,
            RV32_OP_CODES_MEM_ST,
            [3, 4, 5, 6, 7]
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Branch Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test BEQ detection
    #[test]
    fn beq() {
        generate_test!(RVT::B, RV32I::BEQ, RV32_OP_CODES_BR, 0);
    }

    /// Test BNE detection
    #[test]
    fn bne() {
        generate_test!(RVT::B, RV32I::BNE, RV32_OP_CODES_BR, 1);
    }

    /// Test BLT detection
    #[test]
    fn blt() {
        generate_test!(RVT::B, RV32I::BLT, RV32_OP_CODES_BR, 4);
    }

    /// Test BGE detection
    #[test]
    fn bge() {
        generate_test!(RVT::B, RV32I::BGE, RV32_OP_CODES_BR, 5);
    }

    /// Test BLTU detection
    #[test]
    fn bltu() {
        generate_test!(RVT::B, RV32I::BLTU, RV32_OP_CODES_BR, 6);
    }

    /// Test BGEU detection
    #[test]
    fn bgeu() {
        generate_test!(RVT::B, RV32I::BGEU, RV32_OP_CODES_BR, 7);
    }

    /// Test Invalid branch instruction
    #[test]
    fn invalid_branch() {
        generate_test!(RVT::B, RV32I::Invalid, RV32_OP_CODES_BR, [2, 3]);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Jump Instruction Tests
    ////////////////////////////////////////////////////////////////////////////////
    /// Test JALR detection
    #[test]
    fn jalr() {
        generate_test!(RVT::I, RV32I::JALR, RV32_OP_CODES_JALR, 0);
    }

    /// Test JAL detection
    #[test]
    fn jal() {
        generate_test!(
            RVT::J,
            RV32I::JAL,
            RV32_OP_CODES_JAL,
            [0, 1, 2, 3, 4, 5, 6, 7]
        );
    }

    /// Test invalid jump
    #[test]
    fn invalid_jalr() {
        generate_test!(
            RVT::I,
            RV32I::Invalid,
            RV32_OP_CODES_JALR,
            [1, 2, 3, 4, 5, 6, 7]
        );
    }

    ////////////////////////////////////////////////////////////////////////////////
    // LUI Instruction Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test LUI detection
    #[test]
    fn lui() {
        generate_test!(RVT::U, RV32I::LUI, RV32_OP_CODES_LUI, 0);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // AUIPC Instruction Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test AUIPC detection
    #[test]
    fn auipc() {
        generate_test!(RVT::U, RV32I::AUIPC, RV32_OP_CODES_AUIPC, 0);
    }
}
