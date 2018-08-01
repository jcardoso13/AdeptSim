extern crate git2;
#[macro_use]
extern crate clap;

use git2::Repository;
use std::env;
use std::env::VarError;
use std::fmt;
use std::fs::File;
use std::io::Error as IoError;
use std::io::Write;
use std::path::Path;

/// Error type
#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Var(VarError),
}

/// Conversion from IoError
impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

/// Conversion from GoblinError
impl From<VarError> for Error {
    fn from(err: VarError) -> Error {
        Error::Var(err)
    }
}

/// Display trait implementation for Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(io_error) => io_error.fmt(f),
            Error::Var(var_error) => var_error.fmt(f),
        }
    }
}

macro_rules! write_yaml_string_line {
    ($output:ident, $prefix:expr, $label:expr, $value:expr) => {
        writeln!($output, "{}{}: \"{}\"", $prefix, $label, $value)
    };
}

macro_rules! write_yaml_line {
    ($output:ident, $prefix:expr, $label:expr, $value:expr) => {
        writeln!($output, "{}{}: {}", $prefix, $label, $value)
    };
}

macro_rules! write_yaml_pair {
    ($output:ident, $label:expr, $value:expr) => {
        write_yaml_string_line!($output, "", $label, $value)
    };
}

macro_rules! write_clap_yaml_header {
    ($output:ident, $name:expr, $version:expr, $commit:expr, $author:expr, $about:expr) => {{
        write_yaml_pair!($output, "name", $name)?;
        write_yaml_pair!($output, "version", $version)?;
        writeln!($output, "long_version: \"{}-{}\"", $version, $commit)?;
        write_yaml_pair!($output, "author", $author)?;
        write_yaml_pair!($output, "about", $about)
    }};
}

macro_rules! write_clap_yaml_arg_params {
    ($output:ident, $param:expr) => (
        write_yaml_line!($output,"        ",$param.0,$param.1)
    );
    ($output:ident, $headparam:expr, $($params:expr),+) => {{
        write_clap_yaml_arg_params!($output,$headparam)?;
        write_clap_yaml_arg_params!($output,$($params),+)
    }};
}

macro_rules! write_clap_yaml_arg_header {
    ($output:ident) => {
        write_yaml_line!($output, "", "args", "")
    };
}

macro_rules! write_clap_yaml_arg {
    ($output:ident, $arg:expr, $($params:expr),+) => {{
        write_yaml_line!($output, "    - ", $arg, "")?;
        write_clap_yaml_arg_params!($output,$($params),+)
    }};
}

fn main() -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;

    // Long Version Fetching:
    let long_version = match Repository::open(".") {
        Ok(repo) => match repo.head() {
            Ok(head) => match head.target() {
                Some(oid) => format!("{}", oid),
                _ => String::from("no_git (no HEAD reference)"),
            },
            _ => String::from("no_git (unresolved HEAD)"),
        },
        _ => String::from("no_git (no repository found)"),
    };

    // Main Binary:
    let dest_path = Path::new(&out_dir).join("main.yaml");
    let mut f = File::create(&dest_path)?;

    write_clap_yaml_header!(
        f,
        crate_name!(),
        crate_version!(),
        long_version,
        crate_authors!(),
        crate_description!()
    )?;
    write_clap_yaml_arg_header!(f)?;
    write_clap_yaml_arg!(
        f,
        "input_elf",
        ("value_name", "\"INPUTFILE\""),
        ("help", "\"Sets the input elf file\""),
        ("required", "true"),
        ("index", "1")
    )?;

    // Disassembler Binary:
    let dest_path = Path::new(&out_dir).join("disassembler.yaml");
    let mut f = File::create(&dest_path)?;

    write_clap_yaml_header!(
        f,
        "adept_disassembler",
        crate_version!(),
        long_version,
        crate_authors!(),
        "Disassemble RV32I elfs"
    )?;
    write_clap_yaml_arg_header!(f)?;
    write_clap_yaml_arg!(
        f,
        "input_elf",
        ("value_name", "\"INPUTFILE\""),
        ("help", "\"Sets the input elf file\""),
        ("required", "true"),
        ("index", "1")
    )?;
    write_clap_yaml_arg!(
        f,
        "PC",
        ("help", "\"Displays Program Counter\""),
        ("short", "p"),
        ("long", "pc")
    )?;
    write_clap_yaml_arg!(
        f,
        "Instruction",
        (
            "help",
            "\"Displays a unsigned 32 bit in hex format Instruction\""
        ),
        ("short", "i"),
        ("long", "instruction")
    )?;
    write_clap_yaml_arg!(
        f,
        "AssemblyCode",
        ("help", "\"Displays Assembly code\""),
        ("short", "a"),
        ("long", "assembly")
    )?;
    write_clap_yaml_arg!(
        f,
        "ASCII",
        ("help", "\" Displays the Data\""),
        ("short", "c"),
        ("long", "ascii")
    )?;

    Ok(())
}
