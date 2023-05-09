#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] =
    b"This is a veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeery long string inside of a static field";

fn base_16_bytes(mut n: usize, buf: &mut [u8]) -> &[u8] {
    if n == 0 {
        return b"0x0";
    }
    let mut i = 2;
    while n > 0 {
        buf[i] = (n % 16) as u8;
        if n % 16 < 10 {
            buf[i] += b'0';
        } else {
            buf[i] += b'A' - 10;
        }
        n /= 16;
        i += 1;
    }
    buf[i] = b'x';
    buf[i + 1] = b'0';
    i += 2;

    let slice = &mut buf[..i];
    slice.reverse();
    &*slice
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    let mut i = 0;

    // for addr in (0..0x10000000).step_by(4) {
    //     if unsafe { *(addr as *const u8) } != b'T' {
    //         continue;
    //     }
    //     if unsafe { *((addr + 4) as *const u8) } != b'h' {
    //         continue;
    //     }

    //     let mut buff = [0; 32];
    //     base_16_bytes(addr, &mut buff);

    //     for b in buff {
    //         if b == 0 {
    //             break;
    //         }
    //         unsafe {
    //             *vga_buffer.offset(i as isize) = b;
    //             *vga_buffer.offset(i as isize + 1) = 0xb;
    //             i += 2;
    //         }
    //     }
    //     unsafe {
    //         *vga_buffer.offset(i as isize) = b' ';
    //         *vga_buffer.offset(i as isize + 1) = 0xb;
    //         i += 2;
    //     }

    //     // unsafe {
    //     //     *vga_buffer.offset(i % (80 * 25) as isize) = *addr.offset(i as isize / 2);
    //     //     *vga_buffer.offset(i % (80 * 25) as isize + 1) = 0xb;
    //     //     i += 2;
    //     // }
    // }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
