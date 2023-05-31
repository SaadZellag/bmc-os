#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bmc_os::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bmc_os::{
    display::{
        color::Color256,
        ensure_graphics_mode,
        graphics::{draw_shape, draw_sprite, flush_buffer, Triangle, PALETTE},
        set_graphics_color,
        sprite::Sprite,
    },
    load_sprite, println, set_pixel,
};
use cozy_chess::{Board, Color, File, Piece, Rank, Square};

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

    let board = Board::default();

    let chessboard = load_sprite!("../sprites/chessboard.data", 160);
    let w_pawn = load_sprite!("../sprites/WhitePawn.data", 20);
    let w_rook = load_sprite!("../sprites/WhiteRook.data", 20);
    let w_knight = load_sprite!("../sprites/WhiteKnight.data", 20);
    let w_bishop = load_sprite!("../sprites/WhiteBishop.data", 20);
    let w_queen = load_sprite!("../sprites/WhiteQueen.data", 20);
    let w_king = load_sprite!("../sprites/WhiteKing.data", 20);

    let b_pawn = load_sprite!("../sprites/BlackPawn.data", 20);
    let b_rook = load_sprite!("../sprites/BlackRook.data", 20);
    let b_knight = load_sprite!("../sprites/BlackKnight.data", 20);
    let b_bishop = load_sprite!("../sprites/BlackBishop.data", 20);
    let b_queen = load_sprite!("../sprites/BlackQueen.data", 20);
    let b_king = load_sprite!("../sprites/BlackKing.data", 20);

    let start_x = (320 - 160) / 2;
    let start_y = (240 - 160) / 2;

    draw_sprite(&chessboard, start_x, start_y);

    for (y, &rank) in Rank::ALL.iter().enumerate() {
        for (x, &file) in File::ALL.iter().enumerate() {
            let square = Square::new(file, rank);

            let sprite = match (board.color_on(square), board.piece_on(square)) {
                (Some(Color::White), Some(Piece::Pawn)) => &w_pawn,
                (Some(Color::White), Some(Piece::Rook)) => &w_rook,
                (Some(Color::White), Some(Piece::Knight)) => &w_knight,
                (Some(Color::White), Some(Piece::Bishop)) => &w_bishop,
                (Some(Color::White), Some(Piece::Queen)) => &w_queen,
                (Some(Color::White), Some(Piece::King)) => &w_king,
                (Some(Color::Black), Some(Piece::Pawn)) => &b_pawn,
                (Some(Color::Black), Some(Piece::Rook)) => &b_rook,
                (Some(Color::Black), Some(Piece::Knight)) => &b_knight,
                (Some(Color::Black), Some(Piece::Bishop)) => &b_bishop,
                (Some(Color::Black), Some(Piece::Queen)) => &b_queen,
                (Some(Color::Black), Some(Piece::King)) => &b_king,
                _ => {
                    continue;
                }
            };

            draw_sprite(sprite, start_x + x * 20, start_y + (7 - y) * 20);
        }
    }

    flush_buffer();

    // for (i, rgb) in morbius.chunks_exact(3).enumerate() {
    //     let color = Color256::new(rgb[0] / 32, rgb[1] / 32, rgb[2] / 64);
    //     set_graphics_color(color);
    //     let x = i % 320;
    //     let y = i / 320;
    //     draw_pixel(x, y);
    // }

    // let badapple = include_bytes!("../badapple.raw");

    // println!("LUL");

    // // interrupts::without_interrupts(|| {
    // for (i, rgb) in badapple.chunks_exact(3).enumerate() {
    //     let color = Color256::new(rgb[0] / 32, rgb[1] / 32, rgb[2] / 64);
    //     // set_graphics_color(color);
    //     let x = i % 320;
    //     let y = (i / 320) % 240;
    //     set_pixel!(x, y, color);

    //     if x == 0 && y == 0 {
    //         flush_buffer();
    //     }
    //     // unsafe {
    //     //     let offset = (320 * y + x) / 4;
    //     //     // let plane_mask = 0x1 << (x & 3);
    //     //     // VGA.lock()
    //     //     //     .sequencer_registers
    //     //     //     .set_plane_mask(PlaneMask::from_bits(plane_mask).unwrap());
    //     //     frame_buffer.add(offset).write_volatile(color.as_u8());
    //     // }
    // }
    // // });

    // println!("Haha yes");

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
