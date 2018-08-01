extern crate adapt_mem_adept;
#[macro_use]
extern crate clap;
extern crate adept_lib;

use clap::App;

use adept_lib::riscv::decoder::Instruction;

fn main() {
    let yaml = load_yaml!(concat!(env!("OUT_DIR"), "/disassembler.yaml"));
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(filename) = matches.value_of("input_elf") {
        eprintln!("Loading elf: {}", filename);

        let mem_data = match adapt_mem_adept::get_adept_data(filename) {
            Ok(chunks) => chunks,
            Err(e) => panic!(e.to_string()),
        };

        let show_disassembled = matches.is_present("AssemblyCode");
        let show_hex = matches.is_present("Instruction");
        let show_counter = matches.is_present("PC");
        let show_ascii = matches.is_present("ASCII");
        let show_all = !(show_disassembled || show_hex || show_counter || show_ascii);

        for chunk in mem_data {
            let base_address = chunk.get_base_address();
            let chunk_length = chunk.get_contents_length();
            let chunk_data = chunk.get_contents();
            println!("{:x}", base_address);
            for offset in 0..(chunk_length >> 2) {
                let actual_offset = offset << 2;

                let address = (base_address as u32) + (actual_offset as u32);

                let bytes = &(chunk_data[actual_offset..actual_offset + 4]);

                let mut instruction = u32::from(bytes[0]);
                instruction += u32::from(bytes[1]) << 8;
                instruction += u32::from(bytes[2]) << 16;
                instruction += u32::from(bytes[3]) << 24;

                let decoded = Instruction::new(instruction);

                if show_counter || show_all {
                    print!("{:>8}", address);
                }
                if show_hex || show_all {
                    print!("{:>8}", instruction);
                }
                if show_ascii || show_all {
                    print!(
                        "[{}{}{}{}] ",
                        byte_in_char(bytes[3]),
                        byte_in_char(bytes[2]),
                        byte_in_char(bytes[1]),
                        byte_in_char(bytes[0])
                    );
                }
                if show_disassembled || show_all {
                    print!("{}", decoded);
                }
                println!();
            }
        }
    }
}

fn byte_in_char(byte_in: u8) -> char {
    if byte_in > 126 || byte_in < 32 {
        '.'
    } else {
        byte_in as char
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
