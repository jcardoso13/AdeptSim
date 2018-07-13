extern crate adapt_mem_adept;
#[macro_use]
extern crate clap;

use clap::{App, Arg};

mod riscv;

use riscv::decoder::Instruction;

include!(concat!(env!("OUT_DIR"), "/gitv.rs"));

fn main() {
    let matches = App::new("adept_disassembler")
        .version(crate_version!())
        .long_version(&*format!("{}-{}", crate_version!(), LONG_VERSION))
        .author(crate_authors!())
        .about("Disassemble RV32I elfs")
        .arg(
            Arg::with_name("input_elf")
                .value_name("INPUTFILE")
                .help("Sets the input elf file")
                .index(1)
                .required(true),
        )
        .get_matches();

    if let Some(filename) = matches.value_of("input_elf") {
        eprintln!("Loading elf: {}", filename);

        let mem_data = match adapt_mem_adept::get_elf_data(filename) {
            Ok(chunks) => chunks,
            Err(e) => panic!(e.to_string()),
        };

        for chunk in mem_data {
            println!("{:x}", chunk.address);
            for offset in 0..(chunk.length >> 2) {
                let actual_offset = offset << 2;

                let address = (chunk.address as u32) + (actual_offset as u32);

                let bytes = &(chunk.data[actual_offset..actual_offset + 4]);

                let mut instruction = u32::from(bytes[0]);
                instruction += u32::from(bytes[1]) << 8;
                instruction += u32::from(bytes[2]) << 16;
                instruction += u32::from(bytes[3]) << 24;

                let decoded = Instruction::new(instruction);

                println!("{:x}:\t{:x}\t{:#?}", address, instruction, decoded);
            }
        }
    }
}

fn byte_in_char(byte_in: u8) -> char {
    if byte_in > 127 {
        return '.';
    } else {
        return byte_in as char;
    }
}

#[cfg(test)]
mod tests {
    ////////////////////////////////////////////////////////////////////////////////
    // Byte to Char Conversion Test
    ////////////////////////////////////////////////////////////////////////////////
    /// Test Registers Printing
    #[test]
    fn byte_to_char_test() {
        // 128 = non_ASCII
        assert_eq!('.', super::byte_in_char(128));
        // 97 = letter 'a'
        assert_eq!('a', super::byte_in_char(97));
        // 65 = letter 'A'
        assert_eq!('A', super::byte_in_char(65));
    }
}
