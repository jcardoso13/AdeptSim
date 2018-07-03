extern crate adapt_mem_adept;
#[macro_use]
extern crate clap;

use clap::{App, Arg};

mod mem;
mod riscv;

use mem::{MemStoreOp, Memory};
use riscv::isa::{RVT,RV32I};
use riscv::decoder::Instruction;

include!(concat!(env!("OUT_DIR"), "/gitv.rs"));

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .long_version(&*format!("{}-{}", crate_version!(), LONG_VERSION))
        .author(crate_authors!())
        .about(crate_description!())
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

        loop{
            let instruction = my_mem.read_pc(pc);
            let decoded = Instruction::new(instruction);
            if !decoded.is_valid() {
                break;
            }
            println!("{:#?}",decoded);
            pc = pc + 4;
        }
        
    }
}
