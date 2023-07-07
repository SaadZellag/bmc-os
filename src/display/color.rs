use core::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color256 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color256 {
    pub const BLACK: Color256 = Color256::new(0, 0, 0);
    pub const WHITE: Color256 = Color256::new(255, 255, 255);
    pub const RED: Color256 = Color256::new(255, 0, 0);
    pub const GREEN: Color256 = Color256::new(0, 255, 0);
    pub const BLUE: Color256 = Color256::new(0, 0, 255);

    pub const LIGHT_BLUE: Color256 = Color256::new(0, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub fn as_u8(self) -> u8 {
        let r = self.r / 32;
        let g = self.g / 32;
        let b = self.b / 64;
        ((r << 5) & 0b11100000) | ((g << 2) & 0b00011100) | (b & 0b00000011)
    }

    pub fn apply_alpha(&self, alpha: u8) -> Self {
        Self {
            r: (self.r as u16 * alpha as u16 / 255) as u8,
            g: (self.g as u16 * alpha as u16 / 255) as u8,
            b: (self.b as u16 * alpha as u16 / 255) as u8,
        }
    }
}

impl Into<u8> for Color256 {
    fn into(self) -> u8 {
        self.as_u8()
    }
}

impl Add<Color256> for Color256 {
    type Output = Color256;

    fn add(self, rhs: Color256) -> Self::Output {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
        }
    }
}
