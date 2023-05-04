use std::{env, fs::File, io::Write, path::Path, process::Command};

use bmc_os::bootloader;

fn main() {
    let mut args = std::env::args();
    args.next();

    let _path = args.next().expect("Provide a path to the OS exe");
    // Do nothing with it so far

    let content = bootloader();

    let mut file = File::create("bmc-os.img").expect("Could not create file");
    file.write_all(&content).expect("Could not write");
    file.flush().expect("Could not flush");
}
