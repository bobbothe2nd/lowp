pub const SIGN64: u64 = 0x8000_0000_0000_0000;
pub const EXP64_MASK: u64 = 0x7ff0_0000_0000_0000;
pub const FRAC64_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const SIGN32: u32 = 0x8000_0000;
pub const EXP32_MASK: u32 = 0x7f80_0000;
pub const FRAC32_MASK: u32 = 0x007f_ffff;

#[inline(always)]
const fn round_shift_u64(x: u64, shift: u32) -> u64 {
    if shift == 0 {
        return x;
    }

    if shift >= 64 {
        return 0;
    }

    let truncated = x >> shift;
    let remainder_mask = (1u64 << shift) - 1;
    let remainder = x & remainder_mask;
    let halfway = 1u64 << (shift - 1);

    if remainder > halfway || (remainder == halfway && (truncated & 1) != 0) {
        truncated + 1
    } else {
        truncated
    }
}

pub mod f4_lut;

pub mod float4;
pub mod float8;

#[cfg(feature = "half")]
pub mod half;
