#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bmc_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use arrayvec::ArrayVec;
use bmc_os::{
    allocator,
    display::{
        color::Color256,
        ensure_graphics_mode,
        graphics::{draw_shape, draw_sprite, flush_buffer, Triangle, PALETTE},
        set_graphics_color,
        sprite::{Sprite, SpriteBlock},
    },
    events::{self, add_event, next_event},
    game::{Event, Game},
    load_sprite, load_sprite_block,
    memory::{self, BootInfoFrameAllocator},
    println, set_pixel,
};
use bootloader::{entry_point, BootInfo};
use cozy_chess::{Board, Color, File, GameStatus, Piece, Rank, Square};
use engine::search::tt::TTEntry;

use engine::{
    engine::{Engine, EngineOptions, MAX_DEPTH},
    handler::SearchHandler,
    search::{tt::TranspositionTable, SearchSharedState},
    utils::tablesize::TableSize,
};
use vga::{
    colors::Color16,
    registers::PlaneMask,
    vga::VGA,
    writers::{
        Graphics320x200x256, Graphics320x240x256, Graphics640x480x16, GraphicsWriter, Text80x25,
        TextWriter,
    },
};
use x86_64::{
    instructions::interrupts,
    structures::paging::{PageTable, Translate},
    VirtAddr,
};

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
    use bmc_os::set_text_color;

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
