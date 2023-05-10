#![no_std]
#![no_main]

mod display;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..20 {
        println!("Hello index {}", i);
    }
    panic!("Hello I panicked here");

    loop {}
}
