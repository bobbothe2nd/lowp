use crate::bit::{
    ops::sign_mask, round_shift_u64, EXP32_MASK, EXP64_MASK, FRAC32_MASK, FRAC64_MASK, NO_INF,
    SIGN32, SIGN64,
};

#[inline(always)]
pub const fn float_to_float<
    const SRC_F: u8,
    const SRC_E: u8,
    const SRC_B: i8,
    const SRC_W: u8,
    const SRC_MAX: u8,
    const SRC_NAN: u8,
    const SRC_INF: u8,
    const DST_F: u8,
    const DST_E: u8,
    const DST_B: i8,
    const DST_W: u8,
    const DST_MAX: u8,
    const DST_NAN: u8,
    const DST_INF: u8,
>(
    bits: u8,
) -> u8 {
    if const {
        SRC_F == DST_F
            && SRC_E == DST_E
            && SRC_B == DST_B
            && SRC_W == DST_W
            && SRC_NAN == DST_NAN
            && SRC_INF == DST_INF
            && DST_MAX == SRC_MAX
    } {
        return bits;
    }

    let src_sign_mask = const { sign_mask::<SRC_W>() };
    let dst_sign_mask = const { sign_mask::<DST_W>() };

    let sign = if bits & src_sign_mask != 0 {
        dst_sign_mask
    } else {
        0
    };

    let abs = bits & !src_sign_mask;

    let src_exp_mask = (1 << SRC_E) - 1;
    let src_frac_mask = (1 << SRC_F) - 1;

    let src_exp = (abs >> SRC_F) & src_exp_mask;
    let src_frac = abs & src_frac_mask;

    if src_exp == src_exp_mask {
        if src_frac != 0 {
            return DST_NAN;
        }

        return if const { DST_INF == NO_INF } {
            sign | DST_MAX
        } else {
            sign | DST_INF
        };
    }

    if abs == 0 {
        return sign;
    }

    let (mantissa, exponent) = if src_exp == 0 {
        (src_frac as u64, 1 - SRC_B - (SRC_F as i8))
    } else {
        (
            ((1 << SRC_F) | src_frac) as u64,
            (src_exp as i8) - SRC_B - (SRC_F as i8),
        )
    };

    let dst_exp_mask = (1 << DST_E) - 1;

    let dst_max_exp = if const { DST_INF != NO_INF } {
        dst_exp_mask - 1
    } else {
        dst_exp_mask
    };

    let min_normal_exp = 1 - DST_B;
    let min_subnormal_exp = min_normal_exp - DST_F as i8;

    let highest_bit = 63 - mantissa.leading_zeros() as i8;
    let unbiased_exp = highest_bit + exponent;

    if unbiased_exp < min_subnormal_exp {
        return sign;
    }

    if unbiased_exp < min_normal_exp {
        let shift = min_subnormal_exp - exponent;

        let sig = round_shift_u64(mantissa, shift as u32);

        if sig >= (1 << DST_F) {
            return sign | (1 << DST_F);
        }

        return sign | sig as u8;
    }

    let shift = highest_bit - DST_F as i8;

    let mut sig = if shift > 0 {
        round_shift_u64(mantissa, shift as u32)
    } else {
        mantissa << (-shift as u32)
    };

    let mut dst_exp = unbiased_exp + DST_B;

    if sig >= (1 << (DST_F + 1)) {
        sig >>= 1;
        dst_exp += 1;
    }

    if dst_exp > dst_max_exp as i8 {
        return sign
            | if const { DST_INF == NO_INF } {
                DST_MAX
            } else {
                DST_INF
            };
    }

    sign | ((dst_exp as u8) << DST_F) | (sig as u8 & ((1 << DST_F) - 1))
}

#[inline(always)]
pub const fn from_f32<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: f32,
) -> u8 {
    let bits = x.to_bits();

    let sign_mask = const { sign_mask::<W>() };
    let sign = if bits & SIGN32 != 0 { sign_mask } else { 0 };
    let abs = bits & !SIGN32;

    let src_exp = (bits & EXP32_MASK) >> 23;
    let src_frac = bits & FRAC32_MASK;

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };

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
        ((src_frac as u64) << (23 - h), -126 - (23 - h) as i32)
    } else {
        ((0x0080_0000 | src_frac) as u64, src_exp as i32 - 127)
    };

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift = (24 - B as i32 - F as i32 - exponent) as u32;
        let sig = round_shift_u64(mantissa, shift);

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + B as i32;

    if target_exp > max_exp as i32 {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    let mut sig = round_shift_u64(mantissa, (23 - F) as u32);
    let mut encoded_exp = target_exp;

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;
        encoded_exp += 1;
    }

    if encoded_exp > max_exp as i32 {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    let encoded = ((encoded_exp as u8) << F) | (sig as u8 & const { (1u8 << F) - 1 });

    if encoded > MAX {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    sign | encoded
}

#[inline(always)]
pub const fn from_f64<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: f64,
) -> u8 {
    let bits = x.to_bits();

    let sign_mask = const { sign_mask::<W>() };
    let sign = if bits & SIGN64 != 0 { sign_mask } else { 0 };
    let abs = bits & !SIGN64;

    let src_exp = ((bits & EXP64_MASK) >> 52) as u32;
    let src_frac = bits & FRAC64_MASK;

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };

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
        (src_frac << (52 - h), -1022 - (52 - h) as i32)
    } else {
        (0x0010_0000_0000_0000 | src_frac, src_exp as i32 - 1023)
    };

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift = (53 - B as i32 - F as i32 - exponent) as u32;
        let sig = round_shift_u64(mantissa, shift);

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + B as i32;

    if target_exp > max_exp as i32 {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    let mut sig = round_shift_u64(mantissa, (52 - F) as u32);
    let mut encoded_exp = target_exp;

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;
        encoded_exp += 1;
    }

    if encoded_exp > max_exp as i32 {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    let encoded = ((encoded_exp as u8) << F) | (sig as u8 & const { (1u8 << F) - 1 });

    if encoded > MAX {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    sign | encoded
}

#[inline(always)]
pub const fn to_f32<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const NAN: u8,
    const INF: u8,
>(
    bits: u8,
) -> f32 {
    let sign_mask = const { sign_mask::<W>() };
    let bits = bits & const { ((1u16 << W) - 1) as u8 };

    let sign = if bits & sign_mask != 0 {
        0x8000_0000
    } else {
        0
    };

    let abs = bits & !sign_mask;

    if NAN != sign_mask && abs == NAN {
        return f32::NAN;
    }

    if const { INF != NO_INF } && abs == INF {
        return f32::from_bits(sign | 0x7f80_0000);
    }

    let exp_mask = const { (1u8 << E) - 1 };
    let frac_mask = const { (1u8 << F) - 1 };

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

        while sig & (1u32 << F) == 0 {
            sig <<= 1;
            unbiased_exp -= 1;
        }

        let f32_exp = (unbiased_exp + 127) as u32;
        let f32_frac = (sig & frac_mask as u32) << (23 - F);

        return f32::from_bits(sign | (f32_exp << 23) | f32_frac);
    }

    let unbiased_exp = exp as i32 - B as i32;
    let f32_exp = (unbiased_exp + 127) as u32;
    let f32_frac = (frac as u32) << (23 - F);

    f32::from_bits(sign | (f32_exp << 23) | f32_frac)
}

#[inline(always)]
pub const fn to_f64<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const NAN: u8,
    const INF: u8,
>(
    bits: u8,
) -> f64 {
    let sign_mask = const { sign_mask::<W>() };
    let bits = bits & const { ((1u16 << W) - 1) as u8 };

    let sign = if bits & sign_mask != 0 {
        0x8000_0000_0000_0000
    } else {
        0
    };

    let abs = bits & !sign_mask;

    if NAN != sign_mask && abs == NAN {
        return f64::NAN;
    }

    if const { INF != NO_INF } && abs == INF {
        return f64::from_bits(sign | 0x7ff0_0000_0000_0000);
    }

    let exp_mask = const { (1u8 << E) - 1 };
    let frac_mask = const { (1u8 << F) - 1 };

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

        while sig & (1u64 << F) == 0 {
            sig <<= 1;
            unbiased_exp -= 1;
        }

        let f64_exp = (unbiased_exp + 1023) as u64;
        let f64_frac = (sig & frac_mask as u64) << (52 - F);

        return f64::from_bits(sign | (f64_exp << 52) | f64_frac);
    }

    let unbiased_exp = exp as i32 - B as i32;
    let f64_exp = (unbiased_exp + 1023) as u64;
    let f64_frac = (frac as u64) << (52 - F);

    f64::from_bits(sign | (f64_exp << 52) | f64_frac)
}
