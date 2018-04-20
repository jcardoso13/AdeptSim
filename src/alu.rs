use riscv::isa::RV32I;

/// ALU operations codes
enum AluOpList {
    Add,
    Sub,
    Sll,
    Slt,
    Sltu,
    Xor,
    Srl,
    Sra,
    Or,
    And,
    Invalid,
}

impl From<RV32I> for AluOpList {
    fn from(instr: RV32I) -> Self {
        match instr {
            RV32I::ADDI | RV32I::ADD => AluOpList::Add,
            RV32I::SLTI | RV32I::SLT | RV32I::BLT | RV32I::BGE => AluOpList::Slt,
            RV32I::SLTIU | RV32I::SLTU | RV32I::BLTU | RV32I::BGEU => AluOpList::Sltu,
            RV32I::XORI | RV32I::XOR => AluOpList::Xor,
            RV32I::ORI | RV32I::OR => AluOpList::Or,
            RV32I::ANDI | RV32I::AND => AluOpList::And,
            RV32I::SLLI | RV32I::SLL => AluOpList::Sll,
            RV32I::SRLI | RV32I::SRL => AluOpList::Srl,
            RV32I::SRAI | RV32I::SRA => AluOpList::Sra,
            RV32I::SUB | RV32I::BEQ | RV32I::BNE => AluOpList::Sub,
            _ => AluOpList::Invalid,
        }
    }
}

pub struct AluOp {
    op: AluOpList,
    switch_2_imm: bool,
}

impl From<RV32I> for AluOp {
    fn from(instr: RV32I) -> Self {
        let switch_2_imm = match instr {
            RV32I::ADDI
            | RV32I::SLTI
            | RV32I::SLTIU
            | RV32I::XORI
            | RV32I::ORI
            | RV32I::ANDI
            | RV32I::SLLI
            | RV32I::SRLI
            | RV32I::SRAI => true,
            _ => false,
        };
        let op = AluOpList::from(instr);
        AluOp { op, switch_2_imm }
    }
}

/// Perform ALU operations
///
/// # Arguments
/// * `op_a` => first operand
/// * `op_b` => second operand
/// * `imm` => immediate
/// * `op` => ALU operation to perform
/// * `switch_2_imm` => switch operand b for the immediate
///
/// # Return Value
/// Result of the ALU operation
pub fn alu(op_a: i32, op_b: i32, imm: i32, op: &AluOp) -> i32 {
    let operand_b = if op.switch_2_imm { imm } else { op_b };

    match op.op {
        AluOpList::Add => op_a + operand_b,
        AluOpList::Sub => op_a - operand_b,
        AluOpList::Sll => op_a << (operand_b & 0x0000_001f),
        AluOpList::Slt => {
            if op_a < operand_b {
                1
            } else {
                0
            }
        }
        AluOpList::Sltu => {
            if (op_a as u32) < operand_b as u32 {
                1
            } else {
                0
            }
        }
        AluOpList::Xor => op_a ^ operand_b,
        AluOpList::Srl => (op_a as u32 >> (operand_b & 0x0000_001f)) as i32,
        AluOpList::Sra => op_a >> (operand_b & 0x0000_001f),
        AluOpList::Or => op_a | operand_b,
        AluOpList::And => op_a & operand_b,
        AluOpList::Invalid => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // Test Immediate (ADDI)
        let result = alu(1, 2, 3, &AluOp::from(RV32I::ADDI));
        assert_eq!(4, result);
        // Test ADD
        let result = alu(1, 2, 3, &AluOp::from(RV32I::ADD));
        assert_eq!(3, result);
    }

    #[test]
    fn test_sub() {
        // Test SUB
        let result = alu(1, 2, 3, &AluOp::from(RV32I::SUB));
        assert_eq!(-1, result);
    }

    #[test]
    fn test_sll() {
        // Test SLLI
        let result = alu(1, 2, 3, &AluOp::from(RV32I::SLLI));
        assert_eq!(8, result);
        // Only look at the least significant 5 bits of the immediate
        let result = alu(1, 2, 32, &AluOp::from(RV32I::SLLI));
        assert_eq!(1, result);
        // Test SLL
        let result = alu(1, 2, 3, &AluOp::from(RV32I::SLL));
        assert_eq!(4, result);
        // Only look at the least significant 5 bits of operand b
        let result = alu(1, 35, 32, &AluOp::from(RV32I::SLL));
        assert_eq!(8, result);
    }

    #[test]
    fn test_slt() {
        // Test Immediate
        let result = alu(-1, 2, 3, &AluOp::from(RV32I::SLTI));
        assert_eq!(1, result);
        // Test SLT
        let result = alu(2, -5, 3, &AluOp::from(RV32I::SLT));
        assert_eq!(0, result);
    }

    #[test]
    fn test_sltu() {
        // Test Immediate
        let result = alu(-2, 4, 2, &AluOp::from(RV32I::SLTIU));
        assert_eq!(0, result);
        // Test SLTU
        let result = alu(2, -5, 3, &AluOp::from(RV32I::SLTU));
        assert_eq!(1, result);
    }

    #[test]
    fn test_xor() {
        // Test Immediate
        let result = alu(-2, 4, 2, &AluOp::from(RV32I::XORI));
        assert_eq!(-2 ^ 2, result);
        // Test XOR
        let result = alu(2, -5, 3, &AluOp::from(RV32I::XOR));
        assert_eq!(2 ^ -5, result);
    }

    #[test]
    fn test_srl() {
        // Test SRLI
        let result = alu(12, 2, 3, &AluOp::from(RV32I::SRLI));
        assert_eq!(1, result);
        // Only look at the least significant 5 bits of the immediate
        let result = alu(12, 2, 32, &AluOp::from(RV32I::SRLI));
        assert_eq!(12, result);
        // Ignore sign
        let result = alu(-12, 2, 5, &AluOp::from(RV32I::SRLI));
        assert_eq!(((0xffff_fff4 as u32) >> 5) as i32, result);
        // Test SRL
        let result = alu(16, 2, 3, &AluOp::from(RV32I::SRL));
        assert_eq!(4, result);
        // Only look at the least significant 5 bits of operand b
        let result = alu(16, 35, 32, &AluOp::from(RV32I::SRL));
        assert_eq!(2, result);
        // Ignore sign
        let result = alu(-12, 2, 5, &AluOp::from(RV32I::SRL));
        assert_eq!(((0xffff_fff4 as u32) >> 2) as i32, result);
    }

    #[test]
    fn test_sra() {
        // Test SRAI
        let result = alu(12, 2, 3, &AluOp::from(RV32I::SRAI));
        assert_eq!(1, result);
        // Only look at the least significant 5 bits of the immediate
        let result = alu(12, 2, 32, &AluOp::from(RV32I::SRAI));
        assert_eq!(12, result);
        // Ignore sign
        let result = alu(-12, 2, 5, &AluOp::from(RV32I::SRAI));
        assert_eq!(-12 >> 5, result);
        // Test SRL
        let result = alu(16, 2, 3, &AluOp::from(RV32I::SRA));
        assert_eq!(4, result);
        // Only look at the least significant 5 bits of operand b
        let result = alu(16, 35, 32, &AluOp::from(RV32I::SRA));
        assert_eq!(2, result);
        // Ignore sign
        let result = alu(-12, 2, 5, &AluOp::from(RV32I::SRA));
        assert_eq!(-12 >> 2, result);
    }

    #[test]
    fn test_or() {
        // Test Immediate
        let result = alu(1, 2, 5, &AluOp::from(RV32I::ORI));
        assert_eq!(5, result);
        // Test Or
        let result = alu(1, 2, 4, &AluOp::from(RV32I::OR));
        assert_eq!(3, result);
    }

    #[test]
    fn test_and() {
        // Test Immediate
        let result = alu(1, 2, 5, &AluOp::from(RV32I::ANDI));
        assert_eq!(1, result);
        // Test Or
        let result = alu(1, 2, 4, &AluOp::from(RV32I::AND));
        assert_eq!(0, result);
    }

    #[test]
    fn test_beq() {
        let result = alu(1, 2, 5, &AluOp::from(RV32I::BEQ));
        assert_eq!(-1, result);
    }

    #[test]
    fn test_bne() {
        let result = alu(1, 2, 5, &AluOp::from(RV32I::BNE));
        assert_eq!(-1, result);
    }

    #[test]
    fn test_blt() {
        let result = alu(1, 2, 5, &AluOp::from(RV32I::BLT));
        assert_eq!(1, result);
    }

    #[test]
    fn test_bge() {
        let result = alu(1, 2, 5, &AluOp::from(RV32I::BGE));
        assert_eq!(1, result);
    }

    #[test]
    fn test_bltu() {
        let result = alu(1, -2, 5, &AluOp::from(RV32I::BLTU));
        assert_eq!(1, result);
    }

    #[test]
    fn test_bgeu() {
        let result = alu(1, -2, 5, &AluOp::from(RV32I::BGEU));
        assert_eq!(1, result);
    }
}
