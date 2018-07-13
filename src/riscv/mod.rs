//! Helper RISC-V functions for decoding

pub mod decoder;
pub mod isa;
pub mod labels;

// Instruction OP codes
const RV32_OP_CODES_ARITH_IMM: u8 = 0x13;
const RV32_OP_CODES_ARITH_REG: u8 = 0x33;
const RV32_OP_CODES_MEM_LD: u8 = 0x03;
const RV32_OP_CODES_MEM_ST: u8 = 0x23;
const RV32_OP_CODES_BR: u8 = 0x63;
const RV32_OP_CODES_JALR: u8 = 0x67;
const RV32_OP_CODES_JAL: u8 = 0x6f;
const RV32_OP_CODES_AUIPC: u8 = 0x17;
const RV32_OP_CODES_LUI: u8 = 0x37;
