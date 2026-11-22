use crate::bit::NO_INF;

use super::{round_shift_u64, EXP32_MASK, EXP64_MASK, FRAC32_MASK, FRAC64_MASK, SIGN32, SIGN64};

const SIGN: u8 = 0x80;

#[inline(always)]
pub const fn is_nan<const F: u8, const E: u8, const NAN: u8, const INF: u8>(x: u8) -> bool {
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
pub const fn from_f32<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: f32,
) -> u8 {
    let bits = x.to_bits();

    let sign = ((bits >> 31) as u8) << 7;
    let abs = bits & !SIGN32;

    let src_exp = (bits & EXP32_MASK) >> 23;
    let src_frac = bits & FRAC32_MASK;

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };
    let max_sig = (1u32 << (F + 1)) - 1;

    if src_exp == 0xff {
        if src_frac != 0 {
            return NAN;
        }

        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    if abs == 0 {
        return sign;
    }

    let (mantissa, exponent) = if src_exp == 0 {
        let h = 31 - src_frac.leading_zeros();
        let mantissa = (src_frac as u64) << (23 - h);
        let exponent = -126 - (23 - h) as i32;
        (mantissa, exponent)
    } else {
        let mantissa = (0x0080_0000 | src_frac) as u64;
        let exponent = src_exp as i32 - 127;
        (mantissa, exponent)
    };

    let min_normal_exp = const { 1 - B as i32 };

    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift = (24i32 - B as i32 - F as i32 - exponent) as u32;

        let sig = round_shift_u64(mantissa, shift);

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + B as i32;

    if target_exp > max_exp as i32 {
        return sign | if const { INF == NO_INF } { MAX } else { INF };
    }

    let shift = 23 - F;

    let mut sig = round_shift_u64(mantissa, shift as u32);

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;

        if target_exp + 1 > max_exp as i32 {
            return sign | ((max_exp << F) as u8 | (max_sig - 1) as u8);
        }

        let encoded = ((target_exp + 1) as u8) << F;
        return sign | encoded | (sig as u8 & ((1 << F) - 1));
    }

    sign | ((target_exp as u8) << F) | (sig as u8 & ((1 << F) - 1))
}

#[inline(always)]
pub const fn from_f64<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: f64,
) -> u8 {
    let bits = x.to_bits();

    let sign = ((bits >> 63) as u8) << 7;
    let abs = bits & !SIGN64;

    let src_exp = ((bits & EXP64_MASK) >> 52) as u32;
    let src_frac = bits & FRAC64_MASK;

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };
    let max_sig = (1u32 << (F + 1)) - 1;

    if src_exp == 0x7ff {
        if src_frac != 0 {
            return NAN;
        }

        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    if abs == 0 {
        return sign;
    }

    let (mantissa, exponent) = if src_exp == 0 {
        let h = 63 - src_frac.leading_zeros();
        let mantissa = src_frac << (52 - h);
        let exponent = -1022 - (52 - h) as i32;
        (mantissa, exponent)
    } else {
        let mantissa = 0x0010_0000_0000_0000 | src_frac;
        let exponent = src_exp as i32 - 1023;
        (mantissa, exponent)
    };

    let min_normal_exp = 1 - (B as i32);
    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift = (const { 53i32 - (B as i32) - F as i32 } - exponent) as u32;

        let sig = round_shift_u64(mantissa, shift);

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + (B as i32);

    if target_exp > max_exp as i32 {
        return sign | if const { INF == NO_INF } { MAX } else { INF };
    }

    let mut sig = round_shift_u64(mantissa, const { 52 - F } as u32);

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;

        if target_exp + 1 > max_exp as i32 {
            return sign | ((max_exp << F) as u8 | (max_sig - 1) as u8);
        }

        let encoded = ((target_exp + 1) as u8) << F;
        return sign | encoded | (sig as u8 & ((1 << F) - 1));
    }

    sign | ((target_exp as u8) << F) | (sig as u8 & ((1 << F) - 1))
}

#[inline(always)]
pub const fn to_f32<const F: u8, const E: u8, const B: i8, const NAN: u8, const INF: u8>(
    bits: u8,
) -> f32 {
    let sign = ((bits & SIGN) as u32) << 24;
    let abs = bits & !SIGN;

    if const { NAN != SIGN } && abs == NAN {
        return f32::NAN;
    }

    if const { INF != NO_INF } && abs == INF {
        return f32::from_bits(sign | 0x7f80_0000);
    }

    let exp_mask = const { 1u8 << E } - 1;
    let frac_mask = const { 1u8 << F } - 1;
    let exp = (abs >> F) & exp_mask;
    let frac = abs & frac_mask;

    if const { INF != NO_INF } && exp == exp_mask && frac != 0 {
        return f32::NAN;
    }

    if exp == 0 && frac == 0 {
        return f32::from_bits(sign);
    }

    if exp == 0 {
        let mut sig = frac as u32;
        let mut unbiased_exp = const { 1 - B as i32 };

        while (sig & (1u32 << F)) == 0 {
            sig <<= 1;
            unbiased_exp -= 1;
        }

        let f32_exp = (unbiased_exp + 127) as u32;
        let f32_frac = (sig & frac_mask as u32) << const { 23 - F };

        return f32::from_bits(sign | (f32_exp << 23) | f32_frac);
    }

    let unbiased_exp = exp as i32 - B as i32;
    let f32_exp = (unbiased_exp + 127) as u32;
    let f32_frac = (frac as u32) << const { 23 - F };

    f32::from_bits(sign | (f32_exp << 23) | f32_frac)
}

#[inline(always)]
pub const fn to_f64<const F: u8, const E: u8, const B: i8, const NAN: u8, const INF: u8>(
    bits: u8,
) -> f64 {
    let sign = ((bits & SIGN) as u64) << 56;
    let abs = bits & !SIGN;

    if const { NAN != SIGN } && abs == NAN {
        return f64::NAN;
    }

    if const { INF != NO_INF } && abs == INF {
        return f64::from_bits(sign | 0x7ff0_0000_0000_0000);
    }

    let exp_mask = const { 1u8 << E } - 1;
    let frac_mask = const { 1u8 << F } - 1;
    let exp = (abs >> F) & exp_mask;
    let frac = abs & frac_mask;

    if const { INF != NO_INF } && exp == exp_mask && frac != 0 {
        return f64::NAN;
    }

    if exp == 0 && frac == 0 {
        return f64::from_bits(sign);
    }

    if exp == 0 {
        let mut sig = frac as u64;
        let mut unbiased_exp = const { 1 - B as i32 };

        while (sig & const { 1u64 << F }) == 0 {
            sig <<= 1;
            unbiased_exp -= 1;
        }

        let f64_exp = (unbiased_exp + 1023) as u64;
        let f64_frac = (sig & frac_mask as u64) << (const { 52 - F });

        return f64::from_bits(sign | (f64_exp << 52) | f64_frac);
    }

    let unbiased_exp = exp as i32 - B as i32;
    let f64_exp = (unbiased_exp + 1023) as u64;
    let f64_frac = (frac as u64) << (const { 52 - F });

    f64::from_bits(sign | (f64_exp << 52) | f64_frac)
}
