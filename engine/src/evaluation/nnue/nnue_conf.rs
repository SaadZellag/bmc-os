use crate::evaluation::nnue::features::SPC;
pub const NUM_FEATURES: usize = 768;
pub const L1: usize = 32; // Also M
pub const L2: usize = 32; // Also K
pub type CurrentFeatures = SPC;