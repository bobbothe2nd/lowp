use half::{bf16, f16};

use crate::bit::{SIGN, float8::NO_INF, round_shift_u64};

#[inline(always)]
pub const fn f8_from_f16<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: f16,
) -> u8 {
    let bits = x.to_bits();

    let sign = ((bits >> 8) as u8) & SIGN;
    let abs = bits & 0x7fff;

    let src_exp = ((bits >> 10) & 0x1f) as u8;
    let src_frac = bits & 0x03ff;

    let max_exp = if const { INF != NO_INF } {
        (1 << E) - 2
    } else {
        (1 << E) - 1
    };

    let max_sig = (1 << (F + 1)) - 1;

    if src_exp == 0x1f {
        if src_frac != 0 {
            return MAX;
        }

        return if const { INF == NO_INF } {
            sign | MAX
        } else {
            sign | INF
        };
    }

    if abs == 0 {
        return sign;
    }

    let (mantissa, exponent) = if src_exp == 0 {
        let h = 15 - src_frac.leading_zeros();

        let mantissa = (src_frac as u64) << (10 - h);
        let exponent = -14 - (10 - h) as i32;

        (mantissa, exponent)
    } else {
        let mantissa = (0x400 | src_frac) as u64;
        let exponent = src_exp as i32 - 15;

        (mantissa, exponent)
    };

    let min_normal_exp = 1 - B as i32;
    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift = (11i32 - B as i32 - F as i32 - exponent) as u32;

        let sig = round_shift_u64(mantissa, shift);

        if sig >= (1u64 << F) {
            return sign | (1 << F);
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + B as i32;

    if target_exp > max_exp as i32 {
        return sign | if const { INF == NO_INF } { MAX } else { INF };
    }

    let shift = 10 - F;
    let mut sig = round_shift_u64(mantissa, shift as u32);

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;

        if target_exp + 1 > max_exp as i32 {
            return sign | ((max_exp << F) as u8) | ((max_sig - 1) as u8);
        }

        let encoded = ((target_exp + 1) as u8) << F;

        return sign | encoded | (sig as u8 & ((1 << F) - 1));
    }

    if target_exp == max_exp as i32 && sig >= max_sig as u64 {
        return sign | if const { INF == NO_INF } { MAX } else { INF };
    }

    sign | ((target_exp as u8) << F) | (sig as u8 & ((1 << F) - 1))
}

#[inline(always)]
pub const fn f8_to_f16<const F: u8, const E: u8, const B: i8, const NAN: u8, const INF: u8>(
    bits: u8,
) -> f16 {
    let sign = ((bits & SIGN) as u16) << 8;
    let abs = bits & !SIGN;

    if const { NAN != SIGN } && abs == NAN {
        return f16::from_bits(0x7fff);
    }

    if const { INF != NO_INF } && abs == INF {
        return f16::from_bits(sign | 0x7c00);
    }

    let exp_mask = const { (1 << E) - 1 };
    let frac_mask = const { (1 << F) - 1 };

    let exp = (abs >> F) & exp_mask;
    let frac = abs & frac_mask;

    if const { INF != NO_INF } && exp == exp_mask && frac != 0 {
        return f16::from_bits(0x7fff);
    }

    if exp == 0 && frac == 0 {
        return f16::from_bits(sign);
    }

    if exp == 0 {
        let mut sig = frac as u16;
        let mut unbiased_exp = const { 1 - B as i32 };

        while (sig & const { 1 << F }) == 0 {
            sig <<= 1;
            unbiased_exp -= 1;
        }

        let f16_exp = (unbiased_exp + 15) as u16;
        let f16_frac = (sig & frac_mask as u16) << const { 10 - F };

        return f16::from_bits(sign | (f16_exp << 10) | f16_frac);
    }

    let unbiased_exp = exp as i32 - B as i32;
    let f16_exp = (unbiased_exp + 15) as u16;
    let f16_frac = (frac as u16) << const { 10 - F };

    f16::from_bits(sign | (f16_exp << 10) | f16_frac)
}

#[inline(always)]
pub const fn f8_from_bf16<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: bf16,
) -> u8 {
    let bits = x.to_bits();

    let sign = ((bits >> 8) as u8) & SIGN;
    let abs = bits & 0x7fff;

    let src_exp = ((bits >> 7) & 0xff) as u16;
    let src_frac = bits & 0x007f;

    let max_exp = if const { INF != NO_INF } {
        (1 << E) - 2
    } else {
        (1 << E) - 1
    };

    let max_sig = (1 << (F + 1)) - 1;

    if src_exp == 0xff {
        if src_frac != 0 {
            return MAX;
        }

        return if const { INF == NO_INF } {
            sign | MAX
        } else {
            sign | INF
        };
    }

    if abs == 0 {
        return sign;
    }

    let (mantissa, exponent) = if src_exp == 0 {
        let h = 15 - src_frac.leading_zeros();

        let mantissa = (src_frac as u64) << (7 - h);
        let exponent = -126 - (7 - h) as i32;

        (mantissa, exponent)
    } else {
        let mantissa = (SIGN as u16 | src_frac) as u64;
        let exponent = src_exp as i32 - 127;

        (mantissa, exponent)
    };

    let min_normal_exp = 1 - B as i32;
    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift = (8i32 - B as i32 - F as i32 - exponent) as u32;

        let sig = round_shift_u64(mantissa, shift);

        if sig >= (1u64 << F) {
            return sign | (1 << F);
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + B as i32;

    if target_exp > max_exp as i32 {
        return sign | if const { INF == NO_INF } { MAX } else { INF };
    }

    let mut sig = round_shift_u64(mantissa, const { 7 - F } as u32);

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;

        if target_exp + 1 > max_exp as i32 {
            return sign | ((max_exp << F) as u8) | ((max_sig - 1) as u8);
        }

        let encoded = ((target_exp + 1) as u8) << F;

        return sign | encoded | (sig as u8 & ((1 << F) - 1));
    }

    if target_exp == max_exp as i32 && sig >= max_sig as u64 {
        return sign | if const { INF == NO_INF } { MAX } else { INF };
    }

    sign | ((target_exp as u8) << F) | (sig as u8 & ((1 << F) - 1))
}

#[inline(always)]
pub const fn f8_to_bf16<const F: u8, const E: u8, const B: i8, const NAN: u8, const INF: u8>(
    bits: u8,
) -> bf16 {
    let sign = ((bits & SIGN) as u16) << 8;
    let abs = bits & !SIGN;

    if const { NAN != SIGN } && abs == NAN {
        return bf16::from_bits(0x7fff);
    }

    if const { INF != NO_INF } && abs == INF {
        return bf16::from_bits(sign | 0x7f80);
    }

    let exp_mask = const { (1 << E) - 1 };
    let frac_mask = const { (1 << F) - 1 };

    let exp = (abs >> F) & exp_mask;
    let frac = abs & frac_mask;

    if const { INF != NO_INF } && exp == exp_mask && frac != 0 {
        return bf16::from_bits(0x7fff);
    }

    if exp == 0 && frac == 0 {
        return bf16::from_bits(sign);
    }

    if exp == 0 {
        let mut sig = frac as u16;
        let mut unbiased_exp = const { 1 - B as i32 };

        while (sig & const { 1 << F }) == 0 {
            sig <<= 1;
            unbiased_exp -= 1;
        }

        let bf16_exp = (unbiased_exp + 127) as u16;
        let bf16_frac = (sig & frac_mask as u16) << const { 7 - F };

        return bf16::from_bits(sign | (bf16_exp << 7) | bf16_frac);
    }

    let unbiased_exp = exp as i32 - B as i32;
    let bf16_exp = (unbiased_exp + 127) as u16;
    let bf16_frac = (frac as u16) << const { 7 - F };

    bf16::from_bits(sign | (bf16_exp << 7) | bf16_frac)
}
