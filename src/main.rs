#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bmc_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bmc_os::println;

use core::panic::PanicInfo;

// Should never be called, but just to satisty compiler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use bmc_os::tests::test_panic_handler;
    test_panic_handler(info)
}

// This is the panic called
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use bmc_os::display::Color;
    use bmc_os::set_color;

    set_color!(Color::Red, Color::Black);
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    bmc_os::init();

    for i in 0..14 {
        println!("Hello index {}", i);
    }

    x86_64::instructions::interrupts::int3();
    // panic!("Hello I panicked here");
    // println!("Henlo");

    loop {}
}
