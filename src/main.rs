extern crate adapt_mem_adept;
#[macro_use]
extern crate clap;

use clap::{App, Arg};

include!(concat!(env!("OUT_DIR"), "/gitv.rs"));

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .long_version(&*format!("{}_{}", crate_version!(), LONG_VERSION))
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
        println!("Value for input_elf: {}", filename);

        let _result = match adapt_mem_adept::get_elf_data(filename) {
            Ok(chunks) => {
                println!("{:#?}", chunks);
                chunks
            }
            Err(e) => panic!(e.to_string()),
        };
    }
}
