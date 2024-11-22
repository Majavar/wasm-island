use crate::map_generator::{Interpolation, NoiseKind};

pub const DEFAULT_SEED: u32 = 0;
pub const DEFAULT_INTERPOLATION: Interpolation = Interpolation::Linear;
pub const DEFAULT_NOISE: NoiseKind = NoiseKind::Value;
pub const DEFAULT_WIDTH: usize = 512;
pub const DEFAULT_HEIGHT: usize = 512;
