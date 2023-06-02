#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableSize {
    size_bytes: usize,
}

impl TableSize {
    pub fn from_bytes(size: usize) -> Self {
        Self { size_bytes: size }
    }

    pub fn from_kb(size: usize) -> Self {
        Self::from_bytes(size * 1024)
    }

    pub fn from_mb(size: usize) -> Self {
        Self::from_kb(size * 1024)
    }

    pub fn to_vec_size<T>(self) -> usize {
        self.size_bytes / core::mem::size_of::<T>()
    }

    pub fn to_vec_rounded<T>(self) -> usize {
        // Taking next power of 2 and dividing to take the previous power of 2
        self.to_vec_size::<T>().next_power_of_two() / 2
    }

    pub fn size_bytes(self) -> usize {
        self.size_bytes
    }

    pub fn size_kb(self) -> usize {
        self.size_bytes / 1024
    }

    pub fn size_mb(self) -> usize {
        self.size_kb() / 1024
    }
}

impl Default for TableSize {
    fn default() -> Self {
        Self::from_mb(8)
    }
}
