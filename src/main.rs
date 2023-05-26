#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bmc_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bmc_os::{
    display::{
        color::Color256,
        draw_line, draw_pixel,
        graphics::{draw_shape, Triangle, PALETTE},
        set_graphics_color, DRAWER,
    },
    println,
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
use x86_64::instructions::interrupts;

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    bmc_os::init();

    // let morbius = include_bytes!("../its-morbin-time.rgb");

    // for (i, rgb) in morbius.chunks_exact(3).enumerate() {
    //     let color = Color256::new(rgb[0] / 32, rgb[1] / 32, rgb[2] / 64);
    //     set_graphics_color(color);
    //     let x = i % 320;
    //     let y = i / 320;
    //     draw_pixel(x, y);
    // }

    let badapple = include_bytes!("../badapple.raw");

    draw_pixel(0, 0);

    interrupts::without_interrupts(|| {
        let drawer = DRAWER.lock();
        let frame_buffer = drawer.get_frame_buffer();
        for (i, rgb) in badapple.chunks_exact(3).enumerate() {
            let color = Color256::new(rgb[0] / 32, rgb[1] / 32, rgb[2] / 64);
            // set_graphics_color(color);
            let x = i % 320;
            let y = (i / 320) % 240;
            drawer.set_pixel(x, y, color.as_u8());
            // unsafe {
            //     let offset = (320 * y + x) / 4;
            //     // let plane_mask = 0x1 << (x & 3);
            //     // VGA.lock()
            //     //     .sequencer_registers
            //     //     .set_plane_mask(PlaneMask::from_bits(plane_mask).unwrap());
            //     frame_buffer.add(offset).write_volatile(color.as_u8());
            // }
        }
    });

    println!("Haha yes");

    // let triangle = Triangle {
    //     points: [(125, 50), (200, 50), (175, 200)],
    // };

    // set_graphics_color(Color256::White);
    // draw_shape(&triangle);

    // for i in 0..15 {
    //     println!("Hello index {}", i);
    // }

    // x86_64::instructions::interrupts::int3();

    // draw_line((10, 10), (10, 100));

    loop {}
}
