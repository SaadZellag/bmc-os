use std::path::Path;
use std::{env, process::Command};

fn main() {
    let in_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let in_path = Path::new(&in_dir).join("src/bootloader.asm");
    let dest_path = Path::new("target/bootloader");

    Command::new("nasm")
        .arg(&in_path)
        .arg("-f")
        .arg("bin")
        .arg("-o")
        .arg(&dest_path)
        .output()
        .expect("Failed to compile");

    println!("cargo:rerun-if-changed={}", in_path.display());
}
