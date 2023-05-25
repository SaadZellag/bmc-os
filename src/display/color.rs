#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color256(u8);

impl Color256 {
    pub const Black: Color256 = Color256::new(0, 0, 0);
    pub const White: Color256 = Color256::new(7, 7, 3);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self(((r << 5) & 0b11100000) | ((g << 2) & 0b00011100) | (b & 0b00000011))
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }
}
