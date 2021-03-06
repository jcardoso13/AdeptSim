extern crate adapt_mem_adept;
#[macro_use]
extern crate clap;
extern crate adept_lib;

use clap::App;

use adept_lib::mem::{MemStoreOp, Memory};
use adept_lib::riscv::decoder::Instruction;
use adept_lib::riscv::isa::RV32I;

fn main() {
    let yaml = load_yaml!(concat!(env!("OUT_DIR"), "/main.yaml"));
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(filename) = matches.value_of("input_elf") {
        eprintln!("Loading elf: {}", filename);

        let mem_data = match adapt_mem_adept::get_adept_data(filename) {
            Ok(chunks) => chunks,
            Err(e) => panic!(e.to_string()),
        };

        let mut my_mem = Box::new(Memory::new());

        for chunk in mem_data {
            let base_address = chunk.get_base_address();
            for offset in 0..(chunk.get_contents_length() >> 2) {
                let actual_offset = offset << 2;
                let address = (base_address as u32) + (actual_offset as u32);
                my_mem.write_data(
                    &MemStoreOp::from(RV32I::SB),
                    address,
                    // This call to unwrap is safe because actual_offset is
                    // guaranteed to be within contents_length
                    chunk.get_word(actual_offset).unwrap(),
                );
            }
        }
        eprintln!("Finished loading memory from elf");

        let mut pc = 0 as u32;

        loop {
            let instruction = my_mem.read_pc(pc);
            let decoded = Instruction::new(instruction);
            if !decoded.is_valid() {
                break;
            }
            println!("{:#?}", decoded);
            pc += 4;
        }
    }
}
