pub const SIGN64: u64 = 0x8000_0000_0000_0000;
pub const EXP64_MASK: u64 = 0x7ff0_0000_0000_0000;
pub const FRAC64_MASK: u64 = 0x000f_ffff_ffff_ffff;

pub const SIGN32: u32 = 0x8000_0000;
pub const EXP32_MASK: u32 = 0x7f80_0000;
pub const FRAC32_MASK: u32 = 0x007f_ffff;

pub const NO_INF: u8 = 0xff;
pub const SIGN: u8 = 0x80;

#[inline(always)]
pub const fn is_nan<const F: u8, const E: u8, const NAN: u8, const INF: u8>(
    x: u8,
) -> bool {
    if const { INF != NO_INF } {
        let exp_mask = const { (1u8 << E) - 1 };
        let frac_mask = const { (1u8 << F) - 1 };
        let exp = (x >> F) & exp_mask;
        let frac = x & frac_mask;

        exp == exp_mask && frac != 0
    } else if const { NAN == 0x7f } {
        x == 0x7f || x == 0xff
    } else {
        x == NAN
    }
}

#[inline(always)]
const fn decode_finite<const F: u8, const E: u8>(x: u8) -> i32 {
    let frac_mask = const { 1u8 << F } - 1;
    let e = (x >> F) & (const { 1u8 << E } - 1);
    let frac = x & frac_mask;

    if e == 0 {
        frac as i32
    } else {
        (const { 1i32 << F } + frac as i32) << (e - 1)
    }
}

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

pub mod float8;

#[cfg(feature = "half")]
pub mod half;
