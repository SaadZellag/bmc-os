#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bmc_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use bmc_os::{
    allocator,
    events::{add_event, next_event},
    game::{Event, Game},
    memory::{self, BootInfoFrameAllocator},
};
use bootloader::{entry_point, BootInfo};

use x86_64::VirtAddr;

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
    use bmc_os::{println, set_text_color};
    use vga::colors::Color16;

    set_text_color!(Color16::Red, Color16::Black);
    println!("{}", info);
    loop {}
}
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    bmc_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let mut game = Game::new();

    add_event(Event::ReturnToMenu);

    loop {
        let event = next_event();
        game.handle_event(&event);
        game.draw();
    }
}
