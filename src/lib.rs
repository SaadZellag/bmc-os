use std::{fs::File, io::Read};

pub fn bootloader() -> Vec<u8> {
    let mut file = File::open("target/bootloader").expect("Could not open file");

    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("Could not read");

    content
}
