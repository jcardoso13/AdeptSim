extern crate git2;

use git2::Repository;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gitv.rs");
    let mut f = File::create(&dest_path).unwrap();

    let version = match Repository::open(".") {
        Ok(repo) => match repo.head() {
            Ok(head) => match head.target() {
                Some(oid) => format!("{}", oid),
                _ => String::from("no_git (no HEAD reference)"),
            },
            _ => String::from("no_git (unresolved HEAD)"),
        },
        _ => String::from("no_git (no repository found)"),
    };

    write!(f, "\nconst LONG_VERSION: &str = \"{}\";\n", version).unwrap();
}
