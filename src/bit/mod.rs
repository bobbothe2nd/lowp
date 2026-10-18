pub const SIGN64: u64 = 0x8000_0000_0000_0000;
pub const EXP64_MASK: u64 = 0x7ff0_0000_0000_0000;
pub const FRAC64_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const SIGN32: u32 = 0x8000_0000;
pub const EXP32_MASK: u32 = 0x7f80_0000;
pub const FRAC32_MASK: u32 = 0x007f_ffff;

pub mod float8;
