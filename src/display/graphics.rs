use vga::colors::PALETTE_SIZE;

pub const PALETTE: [u8; PALETTE_SIZE] = {
    let mut palette = [0_u8; PALETTE_SIZE];
    let mut i = 0;

    let mut r: u8 = 0;

    while r < 8 {
        let mut g: u8 = 0;
        while g < 8 {
            let mut b: u8 = 0;
            while b < 4 {
                palette[i] = r * 8;
                palette[i + 1] = g * 8;
                palette[i + 2] = b * 16;
                i += 3;
                b += 1;
            }
            g += 1;
        }
        r += 1;
    }

    palette
};
