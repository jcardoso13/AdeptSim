//! Before any reads or writes are performed we check the validity of the
//! destination/source IDs. An ID is valid if it's between 0 and 31. For any
//! number larger than 31 we return 0.
//!
//! # Example:
//!
//! ```
//! # use adept_sim::register_file::RegisterFile;
//! let mut my_reg_file = RegisterFile::new();
//! // Write 5 to register 0 is ignored
//! my_reg_file.write(0, 5);
//! // However a write to 21 is valid
//! my_reg_file.write(21, 31);
//! assert_eq!((0, 31), my_reg_file.read(0, 21));
//! ```

#[derive(Default)]
pub struct RegisterFile {
    registers: Vec<i32>,
}

impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile {
            // Create 31 registers, register 0 is always 0
            registers: vec![0; 31],
        }
    }

    /// Write data to a register given an ID
    ///
    /// Before writing the data the ID is checked. If it's register 0 or id is
    /// greater than 31, ignore write.
    ///
    /// # Arguments
    /// * `rsd` => the identification number of the destination register which
    /// will store the data
    /// * `data` => the data to be stored
    pub fn write(&mut self, rsd: u8, data: i32) {
        if rsd != 0 && rsd < 32 {
            self.registers[rsd as usize - 1] = data;
        }
    }

    /// Read the contents of two registers simultaneously
    ///
    /// Before reading the data the ID is checked. If it's register 0 or id is
    /// greater than 31, return 0.
    ///
    /// # Arguments
    /// * `rs1` => id the of the first source register
    /// * `rs2` => id the of the second source register
    pub fn read(&self, rs1: u8, rs2: u8) -> (i32, i32) {
        let rs1_read = if rs1 == 0 || rs1 >= 32 {
            0
        } else {
            self.registers[rs1 as usize - 1]
        };

        let rs2_read = if rs2 == 0 || rs2 >= 32 {
            0
        } else {
            self.registers[rs2 as usize - 1]
        };

        (rs1_read, rs2_read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_write() {
        let mut reg_file = RegisterFile::new();

        reg_file.write(32, 54);
        reg_file.write(128, 88);

        assert_eq!((0, 0), reg_file.read(32, 128));
    }

    #[test]
    fn invalid_read() {
        let reg_file = RegisterFile::new();

        assert_eq!((0, 0), reg_file.read(32, 128));
    }

    #[test]
    fn test_rw() {
        let mut reg_file = RegisterFile::new();

        reg_file.write(4, 54);
        reg_file.write(16, 74);

        assert_eq!((74, 54), reg_file.read(16, 4));
    }

    #[test]
    fn test_rw_to_r0() {
        let mut reg_file = RegisterFile::new();

        reg_file.write(0, 54);
        reg_file.write(0, 74);

        assert_eq!((0, 0), reg_file.read(0, 1));
    }
}
