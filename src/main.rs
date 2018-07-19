extern crate adapt_mem_adept;
#[macro_use]
extern crate clap;

use clap::App;

mod mem;
mod riscv;

use mem::{MemStoreOp, Memory};
use riscv::decoder::Instruction;
use riscv::isa::RV32I;

fn main() {
    let yaml = load_yaml!(concat!(env!("OUT_DIR"), "/main.yaml"));
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(filename) = matches.value_of("input_elf") {
        eprintln!("Loading elf: {}", filename);

        let mem_data = match adapt_mem_adept::get_elf_data(filename) {
            Ok(chunks) => chunks,
            Err(e) => panic!(e.to_string()),
        };

        let mut my_mem = Box::new(Memory::new());

        for chunk in mem_data {
            for offset in 0..chunk.length {
                let address = (chunk.address as u32) + (offset as u32);
                my_mem.write_data(
                    &MemStoreOp::from(RV32I::SB),
                    address,
                    chunk.data[offset].into(),
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
