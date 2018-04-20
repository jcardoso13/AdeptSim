//! This module contains the necessary methods and struct to operate over a
//! memory. The memory by default is created with an address space of 21 bits,
//! which means the memory should use 8MB. You should consider storing this in
//! the heap instead of the stack.

/// Memory is represented has 4 banks of 1 byte each.
#[derive(Default)]
pub struct Memory {
    bank_0: Vec<u8>,
    bank_1: Vec<u8>,
    bank_2: Vec<u8>,
    bank_3: Vec<u8>,
}

impl Memory {
    const MEMORY_ADDR_SIZE: u32 = 21;

    /// Create memory component. Memory is byte addressable and has a bank per
    /// byte.
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
        let masked_pc = Self::mask_addr(pc);

        // Concatenate addresses
        let final_data: u32 = u32::from(self.bank_3[masked_pc]) << 24
            | u32::from(self.bank_2[masked_pc]) << 16
            | u32::from(self.bank_1[masked_pc]) << 8
            | u32::from(self.bank_0[masked_pc]);

        final_data
    }

    /// Mask address to be read or written depending on MEMORY_ADDR_SIZE.
    ///
    /// # Arguments
    /// * `addr` => address to mask
    ///
    /// # Return Value
    /// Masked address converted to usize as to ease Vec addressing
    fn mask_addr(addr: u32) -> usize {
        (addr & ((1 << Self::MEMORY_ADDR_SIZE) + (1 << Self::MEMORY_ADDR_SIZE) - 1)) as usize
    }

    /// Write some garbage data to memory. This only used for tests please
    /// ignore.
    fn __write_garbage(&mut self, data: u32, addr: u32) {
        let data_0 = (data & 0x0000_00ff) as u8;
        let data_1 = ((data & 0x0000_ff00) >> 8) as u8;
        let data_2 = ((data & 0x00ff_0000) >> 16) as u8;
        let data_3 = ((data & 0xff00_0000) >> 24) as u8;

        let masked_addr = Self::mask_addr(addr);

        self.bank_0[masked_addr] = data_0;
        self.bank_1[masked_addr] = data_1;
        self.bank_2[masked_addr] = data_2;
        self.bank_3[masked_addr] = data_3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pc() {
        let mut mem = Box::new(Memory::new());

        // With MEMORY_ADDR_SIZE = 21 the 10 MSB should be ignored by the
        // read_pc and write_garbage method.
        mem.__write_garbage(0xdead_beef, 0x0040_babe);
        mem.__write_garbage(0xbeef_dead, 0x0000_babe);

        assert_eq!(0xbeef_dead, mem.read_pc(0x0000_babe));
        assert_eq!(0xbeef_dead, mem.read_pc(0x0040_babe));
    }

    #[test]
    fn test_mask_addr() {
        assert_eq!(0x0000_babe, Memory::mask_addr(0x0000_babe));
        assert_eq!(0x002d_babe, Memory::mask_addr(0xdead_babe));
        assert_eq!(0x002f_0000, Memory::mask_addr(0xbeef_0000));
        assert_eq!(0x0031_3131, Memory::mask_addr(0x3131_3131));
    }
}
