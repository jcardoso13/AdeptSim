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
    // Return decoded instruction
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
        let imm: Option<i32> = match instr.instr_type {
            RVT::I => Some((raw_instr & 0xfff0_0000) as i32 >> 20),
            RVT::S => {
                Some((((raw_instr & 0xfe00_0000) >> 20) | ((raw_instr & 0x0000_0780) >> 7)) as i32)
            }
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

    /// The dumbest of tests
    #[test]
    fn addi_pos() {
        let final_instr = Instruction {
            rd: Some(4),
            rs2: None,
            rs1: Some(3),
            imm: Some(15),
            shamt: None,
            instr: InstrType::new(0x13, 0, false)
        };

        // ADDI R4 <= R3 + 15
        let parsed_instr = Instruction::new(0x00f1_8213);
        assert_eq!(parsed_instr, final_instr);
    }
}
