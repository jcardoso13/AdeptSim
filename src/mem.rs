//! This module contains the necessary methods and struct to operate over a
//! memory. The memory by default is created with an address space of 21 bits,
//! which means the memory should use 8MB. You should consider storing this in
//! the heap instead of the stack.
//!
//! # Example:
//!
//! ```
//! # use adept_sim::mem::{Memory, MemStoreOp, MemLoadOp};
//! # use adept_sim::riscv::isa::RV32I;
//! let mut my_mem = Box::new(Memory::new());
//! # my_mem.write_data(&MemStoreOp::from(RV32I::SW), 0x0040_babc, 0xdead_beef);
//! // To read the PC use the read_pc method
//! assert_eq!(0xdead_beef, my_mem.read_pc(0x0040_babc));
//! // To store data use the write_data method. You can use the object returned
//! // by the decoder directly in the method.
//! my_mem.write_data(&MemStoreOp::from(RV32I::SW), 0x0040_babc, 0xdead_babe);
//! // To load data use the read_data method
//! assert_eq!(0xdead_babe, my_mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc));
//! ```
use riscv::isa::RV32I;

/// Memory is represented has 4 banks of 1 byte each.
#[derive(Default, Debug)]
pub struct Memory {
    bank_0: Vec<u8>,
    bank_1: Vec<u8>,
    bank_2: Vec<u8>,
    bank_3: Vec<u8>,
}

impl Memory {
    const MEMORY_ADDR_SIZE: u32 = 21;

    /// Create memory component. Memory is byte addressable, little endian, and
    /// has a bank per byte.
    pub fn new() -> Self {
        Memory {
            bank_0: vec![0; 1 << Self::MEMORY_ADDR_SIZE],
            bank_1: vec![0; 1 << Self::MEMORY_ADDR_SIZE],
            bank_2: vec![0; 1 << Self::MEMORY_ADDR_SIZE],
            bank_3: vec![0; 1 << Self::MEMORY_ADDR_SIZE],
        }
    }

    /// Read PC value from memory. This method does not have any stalls.
    ///
    /// # Arguments
    /// * `pc` => address to read instruction from
    ///
    /// # Return Value
    /// The instruction in the selected address
    pub fn read_pc(&self, pc: u32) -> u32 {
        // Memory has a 32-bit address space but here we only use
        // MEMORY_ADDR_SIZE bits to address the memory. Thus, we are going to
        // mask the pc address.
        let masked_pc = Self::mask_addr(pc) >> 2;

        // Concatenate addresses
        let final_data: u32 = u32::from(self.bank_3[masked_pc]) << 24
            | u32::from(self.bank_2[masked_pc]) << 16
            | u32::from(self.bank_1[masked_pc]) << 8
            | u32::from(self.bank_0[masked_pc]);

        final_data
    }

    // Mask address to be read or written depending on MEMORY_ADDR_SIZE.
    //
    // # Arguments
    // * `addr` => address to mask
    //
    // # Return Value
    // Masked address converted to usize as to ease Vec addressing
    fn mask_addr(addr: u32) -> usize {
        (addr & ((1 << Self::MEMORY_ADDR_SIZE) + (1 << Self::MEMORY_ADDR_SIZE) - 1)) as usize
    }

    // Write some garbage data to memory. This is only used in tests, please
    // ignore.
    fn __write_garbage(&mut self, data: u32, addr: u32) {
        let data_0 = (data & 0x0000_00ff) as u8;
        let data_1 = ((data & 0x0000_ff00) >> 8) as u8;
        let data_2 = ((data & 0x00ff_0000) >> 16) as u8;
        let data_3 = ((data & 0xff00_0000) >> 24) as u8;

        let masked_addr = Self::mask_addr(addr) >> 2;

        self.bank_0[masked_addr] = data_0;
        self.bank_1[masked_addr] = data_1;
        self.bank_2[masked_addr] = data_2;
        self.bank_3[masked_addr] = data_3;
    }

    // Get data from one memory bank at a specific address
    //
    // # Arguments
    // * `addr` => address to read data from
    // * `select_bank` => ...
    //
    // # Return Value
    // Value selected from the memory bank at the specified address
    fn get_data(&self, addr: usize, select_bank: u8) -> u8 {
        match select_bank {
            0 => self.bank_0[addr],
            1 => self.bank_1[addr],
            2 => self.bank_2[addr],
            3 => self.bank_3[addr],
            _ => panic!("LSBs in read address is greater than 3"),
        }
    }

    // Write data to one memory bank at a specific address
    //
    // # Arguments
    // * `addr` => address to read data from
    // * `select_bank` => ...
    // * `data` => data to write
    fn put_data(&mut self, addr: usize, select_bank: u8, data: u8) {
        match select_bank {
            0 => self.bank_0[addr] = data,
            1 => self.bank_1[addr] = data,
            2 => self.bank_2[addr] = data,
            3 => self.bank_3[addr] = data,
            _ => panic!("LSBs in read address is greater than 3"),
        }
    }

    /// Perform a read operation on the memory
    ///
    /// # Arguments
    /// * `op` => read operation to perform (load byte, half, or word)
    /// * `addr` => memory address to read from
    ///
    /// # Return Value
    /// Value read from memory
    pub fn load_data(&self, op: &MemLoadOp, addr: u32) -> i32 {
        let masked_addr = Self::mask_addr(addr) >> 2;
        let addr_lsbs = (addr & 0x0000_0003) as u8;

        match *op {
            MemLoadOp::LoadByte => {
                let data = self.get_data(masked_addr, addr_lsbs);

                let sign_extend: u32 = if ((data & 0x80) >> 7) == 1 {
                    0xffff_ff00
                } else {
                    0x0000_0000
                };

                // Cat and sign extend
                (sign_extend | u32::from(data)) as i32
            }
            MemLoadOp::LoadHalf => {
                let data_0 = self.get_data(masked_addr, addr_lsbs);
                let data_1 = self.get_data(masked_addr, addr_lsbs + 1);

                let sign_extend = if ((data_1 & 0x80) >> 7) == 1 {
                    0xffff_0000
                } else {
                    0x0000_0000
                };

                // Cat and sign extend
                (sign_extend | u32::from(data_1) << 8 | u32::from(data_0)) as i32
            }
            MemLoadOp::LoadWord => {
                let data_0 = self.get_data(masked_addr, addr_lsbs);
                let data_1 = self.get_data(masked_addr, addr_lsbs + 1);
                let data_2 = self.get_data(masked_addr, addr_lsbs + 2);
                let data_3 = self.get_data(masked_addr, addr_lsbs + 3);

                (u32::from(data_3) << 24 | u32::from(data_2) << 16 | u32::from(data_1) << 8
                    | u32::from(data_0)) as i32
            }
            MemLoadOp::LoadByteUnsigned => i32::from(self.get_data(masked_addr, addr_lsbs)),
            MemLoadOp::LoadHalfUnsigned => {
                let data_0 = self.get_data(masked_addr, addr_lsbs);
                let data_1 = self.get_data(masked_addr, addr_lsbs + 1);

                (u32::from(data_1) << 8 | u32::from(data_0)) as i32
            }
            MemLoadOp::InvalidLoad => panic!("Invalid Load operation on Memory"),
        }
    }

    /// Perform a write operation on the memory
    ///
    /// # Arguments
    /// * `op` => write operation to perform (store byte, half, or word)
    /// * `addr` => memory address to write to
    /// * `data` => ...
    pub fn write_data(&mut self, op: &MemStoreOp, addr: u32, data: u32) {
        let split_data = (
            data & 0x0000_00ff,
            (data & 0x0000_ff00) >> 8,
            (data & 0x00ff_0000) >> 16,
            (data & 0xff00_0000) >> 24,
        );
        let masked_addr = Self::mask_addr(addr) >> 2;
        let addr_lsbs = (addr & 0x0000_0003) as u8;

        match *op {
            MemStoreOp::StoreByte => self.put_data(masked_addr, addr_lsbs, split_data.0 as u8),
            MemStoreOp::StoreHalf => {
                self.put_data(masked_addr, addr_lsbs, split_data.0 as u8);
                self.put_data(masked_addr, addr_lsbs + 1, split_data.1 as u8);
            }
            MemStoreOp::StoreWord => {
                self.put_data(masked_addr, addr_lsbs, split_data.0 as u8);
                self.put_data(masked_addr, addr_lsbs + 1, split_data.1 as u8);
                self.put_data(masked_addr, addr_lsbs + 2, split_data.2 as u8);
                self.put_data(masked_addr, addr_lsbs + 3, split_data.3 as u8);
            }
            MemStoreOp::InvalidStore => panic!("Invalid write operation on Memory"),
        }
    }
}

/// Memory Load Operations
pub enum MemLoadOp {
    LoadByte,
    LoadHalf,
    LoadWord,
    LoadByteUnsigned,
    LoadHalfUnsigned,
    InvalidLoad,
}

impl From<RV32I> for MemLoadOp {
    fn from(instr: RV32I) -> Self {
        match instr {
            RV32I::LB => MemLoadOp::LoadByte,
            RV32I::LH => MemLoadOp::LoadHalf,
            RV32I::LW => MemLoadOp::LoadWord,
            RV32I::LBU => MemLoadOp::LoadByteUnsigned,
            RV32I::LHU => MemLoadOp::LoadHalfUnsigned,
            _ => MemLoadOp::InvalidLoad,
        }
    }
}

/// Memory Store Operations
pub enum MemStoreOp {
    StoreByte,
    StoreHalf,
    StoreWord,
    InvalidStore,
}

impl From<RV32I> for MemStoreOp {
    fn from(instr: RV32I) -> Self {
        match instr {
            RV32I::SB => MemStoreOp::StoreByte,
            RV32I::SH => MemStoreOp::StoreHalf,
            RV32I::SW => MemStoreOp::StoreWord,
            _ => MemStoreOp::InvalidStore,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    ////////////////////////////////////////
    // PC
    ////////////////////////////////////////
    #[test]
    fn test_read_pc() {
        let mut mem = Box::new(Memory::new());

        // With MEMORY_ADDR_SIZE = 21 the 10 MSB should be ignored by the
        // read_pc and write_garbage method.
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        mem.__write_garbage(0xbeef_dead, 0x0000_babc);

        assert_eq!(0xbeef_dead, mem.read_pc(0x0000_babc));
        assert_eq!(0xbeef_dead, mem.read_pc(0x0040_babc));
    }

    #[test]
    fn test_mask_addr() {
        assert_eq!(0x0000_babe, Memory::mask_addr(0x0000_babe));
        assert_eq!(0x002d_babe, Memory::mask_addr(0xdead_babe));
        assert_eq!(0x002f_0000, Memory::mask_addr(0xbeef_0000));
        assert_eq!(0x0031_3131, Memory::mask_addr(0x3131_3131));
    }

    ////////////////////////////////////////
    // Load Operations
    ////////////////////////////////////////
    #[test]
    #[should_panic]
    fn test_load_data_invalid_with_mem_load_op() {
        let mem = Box::new(Memory::new());
        let _ = mem.load_data(&MemLoadOp::InvalidLoad, 0x3141142);
    }

    #[test]
    #[should_panic]
    fn test_load_data_invalid_with_rv32i() {
        let mem = Box::new(Memory::new());
        let _ = mem.load_data(&MemLoadOp::from(RV32I::ADD), 0x3141142);
    }

    #[test]
    fn test_load_data_byte() {
        let mut mem = Box::new(Memory::new());
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        // Sign Extension
        assert_eq!(
            (0xffff_ffef as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babc)
        );
        assert_eq!(
            (0xffff_ffbe as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babd)
        );
        assert_eq!(
            (0xffff_ffad as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babe)
        );
        assert_eq!(
            (0xffff_ffde as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babf)
        );
        // Non-Sign extension
        mem.__write_garbage(0x4624_3667, 0x0040_babc);
        assert_eq!(
            0x0000_0067,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babc)
        );
        assert_eq!(
            0x0000_0036,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babd)
        );
        assert_eq!(
            0x0000_0024,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babe)
        );
        assert_eq!(
            0x0000_0046,
            mem.load_data(&MemLoadOp::from(RV32I::LB), 0x0040_babf)
        );
    }

    #[test]
    fn test_load_data_half() {
        let mut mem = Box::new(Memory::new());
        // Sign Extension
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        assert_eq!(
            (0xffff_beef as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babc)
        );
        assert_eq!(
            (0xffff_adbe as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babd)
        );
        assert_eq!(
            (0xffff_dead as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babe)
        );
        // Non-Sign Extension
        mem.__write_garbage(0x4624_3667, 0x0040_babc);
        assert_eq!(
            0x0000_3667,
            mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babc)
        );
        assert_eq!(
            0x0000_2436,
            mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babd)
        );
        assert_eq!(
            0x0000_4624,
            mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babe)
        );
    }

    #[test]
    #[should_panic]
    fn test_load_data_half_invalid_lsb() {
        let mut mem = Box::new(Memory::new());
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        let _ = mem.load_data(&MemLoadOp::from(RV32I::LH), 0x0040_babf);
    }

    #[test]
    fn test_load_data_word() {
        let mut mem = Box::new(Memory::new());
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        assert_eq!(
            (0xdead_beef as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
    }

    #[test]
    #[should_panic]
    fn test_load_data_word_lsb_different_than_zero() {
        let mem = Box::new(Memory::new());
        let _ = mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babd);
    }

    #[test]
    fn test_load_data_byte_unsigned() {
        let mut mem = Box::new(Memory::new());
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        assert_eq!(
            0x0000_00ef,
            mem.load_data(&MemLoadOp::from(RV32I::LBU), 0x0040_babc)
        );
        assert_eq!(
            0x0000_00be,
            mem.load_data(&MemLoadOp::from(RV32I::LBU), 0x0040_babd)
        );
        assert_eq!(
            0x0000_00ad,
            mem.load_data(&MemLoadOp::from(RV32I::LBU), 0x0040_babe)
        );
        assert_eq!(
            0x0000_00de,
            mem.load_data(&MemLoadOp::from(RV32I::LBU), 0x0040_babf)
        );
    }

    #[test]
    fn test_load_data_half_unsigned() {
        let mut mem = Box::new(Memory::new());
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        assert_eq!(
            0x0000_beef,
            mem.load_data(&MemLoadOp::from(RV32I::LHU), 0x0040_babc)
        );
        assert_eq!(
            0x0000_adbe,
            mem.load_data(&MemLoadOp::from(RV32I::LHU), 0x0040_babd)
        );
        assert_eq!(
            0x0000_dead,
            mem.load_data(&MemLoadOp::from(RV32I::LHU), 0x0040_babe)
        );
    }

    #[test]
    #[should_panic]
    fn test_load_data_half_unsigned_invalid_lsb() {
        let mem = Box::new(Memory::new());
        let _ = mem.load_data(&MemLoadOp::from(RV32I::LHU), 0x0040_babf);
    }

    ////////////////////////////////////////
    // Write Operations
    ////////////////////////////////////////
    #[test]
    #[should_panic]
    fn test_store_data_invalid_with_mem_store_op() {
        let mut mem = Box::new(Memory::new());
        let _ = mem.write_data(&MemStoreOp::InvalidStore, 0x3141142, 0xdead_beef);
    }

    #[test]
    #[should_panic]
    fn test_store_data_invalid_with_rv32i() {
        let mut mem = Box::new(Memory::new());
        let _ = mem.write_data(&MemStoreOp::from(RV32I::ADD), 0x3141142, 0xdead_beef);
    }

    #[test]
    fn test_write_data_byte() {
        let mut mem = Box::new(Memory::new());
        // Sanity write
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        // Actual real write
        mem.write_data(&MemStoreOp::from(RV32I::SB), 0x0040_babc, 0x0000_0042);
        assert_eq!(
            (0xdead_be42 as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
        mem.write_data(&MemStoreOp::from(RV32I::SB), 0x0040_babd, 0x0000_0042);
        assert_eq!(
            (0xdead_4242 as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
        mem.write_data(&MemStoreOp::from(RV32I::SB), 0x0040_babe, 0x0000_0042);
        assert_eq!(
            (0xde42_4242 as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
        mem.write_data(&MemStoreOp::from(RV32I::SB), 0x0040_babf, 0x0000_0042);
        assert_eq!(
            0x4242_4242,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
    }

    #[test]
    fn test_write_data_half() {
        let mut mem = Box::new(Memory::new());
        // Sanity write
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        // Actual real write
        mem.write_data(&MemStoreOp::from(RV32I::SH), 0x0040_babc, 0x0000_6942);
        assert_eq!(
            (0xdead_6942 as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
        mem.write_data(&MemStoreOp::from(RV32I::SH), 0x0040_babd, 0x0000_3142);
        assert_eq!(
            (0xde31_4242 as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
        mem.write_data(&MemStoreOp::from(RV32I::SH), 0x0040_babe, 0x0000_abcd);
        assert_eq!(
            (0xabcd_4242 as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
    }

    #[test]
    #[should_panic]
    fn test_write_data_half_invalid_lsb() {
        let mut mem = Box::new(Memory::new());
        mem.write_data(&MemStoreOp::from(RV32I::SH), 0x0040_babf, 0xdeadbeef);
    }

    #[test]
    fn test_write_data_word() {
        let mut mem = Box::new(Memory::new());
        // Sanity write
        mem.__write_garbage(0xdead_beef, 0x0040_babc);
        // Actual real write
        mem.write_data(&MemStoreOp::from(RV32I::SW), 0x0040_babc, 0xbabe_31ab);
        assert_eq!(
            (0xbabe_31ab as u32) as i32,
            mem.load_data(&MemLoadOp::from(RV32I::LW), 0x0040_babc)
        );
    }

    #[test]
    #[should_panic]
    fn test_write_data_word_lsb_different_than_zero() {
        let mut mem = Box::new(Memory::new());
        mem.write_data(&MemStoreOp::from(RV32I::LW), 0x0040_babd, 0xabcd_ef12);
    }
}
