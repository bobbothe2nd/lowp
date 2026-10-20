use super::{EXP32_MASK, EXP64_MASK, FRAC32_MASK, FRAC64_MASK, SIGN32, SIGN64, SIGN, NO_INF, round_shift_u64, decode_finite, is_nan};

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

#[inline(always)]
pub const fn addsub<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
    const SUB: bool,
>(
    a: u8,
    b: u8,
) -> u8 {
    if is_nan::<F, E, NAN, INF>(a) || is_nan::<F, E, NAN, INF>(b) {
        return NAN;
    }

    let a_abs = a & !SIGN;
    let b_abs = b & !SIGN;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            let sa = a >> 7;
            let sb = (b >> 7) ^ (SUB as u8);

            if a_inf && b_inf && sa != sb {
                return NAN;
            }

            return if a_inf { a } else { b ^ ((SUB as u8) << 7) };
        }
    }

    if a_abs == 0 {
        if b_abs == 0 {
            return if SUB { a & !b } else { a & b };
        }

        return if SUB { b ^ SIGN } else { b };
    }

    if b_abs == 0 {
        return a;
    }

    let sa = a >> 7;
    let sb = (b >> 7) ^ (SUB as u8);

    let mut ma = decode_finite::<F, E>(a_abs);
    let mut mb = decode_finite::<F, E>(b_abs);

    if sa != 0 {
        ma = -ma;
    }

    if sb != 0 {
        mb = -mb;
    }

    let sum = ma + mb;

    if sum == 0 {
        return 0;
    }

    let sign = if sum < 0 { SIGN } else { 0 };
    let q = sum.unsigned_abs();

    if q < const { 1 << F } {
        return sign | q as u8;
    }

    let h = 63 - q.leading_zeros();
    let shift = h - (F as u32);

    let mut mantissa = q >> shift;

    if shift != 0 {
        let remainder_mask = (1 << shift) - 1;
        let remainder = q & remainder_mask;
        let halfway = 1 << (shift - 1);

        if remainder > halfway || (remainder == halfway && (mantissa & 1) != 0) {
            mantissa += 1;
        }
    }

    let mut exponent = h as i32 - (B as i32 + F as i32 - 1);

    if mantissa >= (1 << (F + 1)) {
        mantissa >>= 1;
        exponent += 1;
    }

    let encoded_exp = exponent + B as i32;
    let max_exp = ((1u16 << E) - 1) as i32;

    if encoded_exp > max_exp {
        return sign | MAX;
    }

    let encoded = ((encoded_exp as u8) << F) | (mantissa - const { 1 << F }) as u8;

    if encoded > MAX {
        return sign | MAX;
    }

    sign | encoded
}

#[inline(always)]
pub const fn mul<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    a: u8,
    b: u8,
) -> u8 {
    if is_nan::<F, E, NAN, INF>(a) || is_nan::<F, E, NAN, INF>(b) {
        return NAN;
    }

    let a_abs = a & !SIGN;
    let b_abs = b & !SIGN;

    let sa = a & SIGN;
    let sb = b & SIGN;
    let sign = sa ^ sb;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if (a_inf && b_zero) || (b_inf && a_zero) {
                return const { SIGN | NAN };
            }

            return sign | INF;
        }
    }

    if a_zero || b_zero {
        return sign;
    }

    let frac_mask = const { 1u8 << F } - 1;
    let exp_mask = const { 1u8 << E } - 1;

    let a_exp = (a_abs >> F) & exp_mask;
    let b_exp = (b_abs >> F) & exp_mask;

    let a_frac = a_abs & frac_mask;
    let b_frac = b_abs & frac_mask;

    let (a_sig, a_power) = if a_exp == 0 {
        (a_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            (const { 1u64 << F } | a_frac as u64),
            a_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let (b_sig, b_power) = if b_exp == 0 {
        (b_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            (const { 1u64 << F } | b_frac as u64),
            b_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let product = a_sig * b_sig;
    let power = a_power + b_power;

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_exp = min_normal_exp - F as i32;

    let product_hi = 63 - product.leading_zeros();
    let unbiased_exp = product_hi as i32 + power;

    if unbiased_exp < min_normal_exp {
        let shift = min_subnormal_exp - power;

        let sig = if shift > 0 {
            round_shift_u64(product, shift as u32)
        } else {
            product << (-shift) as u32
        };

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };

    let product_hi = 63 - product.leading_zeros();
    let mut target_exp = product_hi as i32 + power;

    let mut sig = if product_hi > F as u32 {
        round_shift_u64(product, product_hi - F as u32)
    } else {
        product << (F as u32 - product_hi)
    };

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;
        target_exp += 1;
    }

    let encoded_exp = target_exp + B as i32;

    if encoded_exp > max_exp as i32 {
        return sign | MAX;
    }

    let encoded = ((encoded_exp as u8) << F) | (sig as u8 & frac_mask);

    if encoded > MAX {
        return sign | MAX;
    }

    sign | encoded
}

#[inline(always)]
const fn round_div_u64(num: u64, den: u64) -> u64 {
    let q = num / den;
    let r = num % den;

    let twice_r = r << 1;

    if twice_r > den || (twice_r == den && (q & 1) != 0) {
        q + 1
    } else {
        q
    }
}

#[inline(always)]
pub const fn div<
    const F: u8,
    const E: u8,
    const B: i8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    a: u8,
    b: u8,
) -> u8 {
    if is_nan::<F, E, NAN, INF>(a) || is_nan::<F, E, NAN, INF>(b) {
        return NAN;
    }

    let a_abs = a & !SIGN;
    let b_abs = b & !SIGN;

    let sign = (a ^ b) & SIGN;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if a_inf && b_inf {
                return const { SIGN | NAN };
            }

            if a_inf {
                return sign | INF;
            }

            return sign;
        }
    }

    if b_zero {
        if a_zero {
            return const { SIGN | NAN };
        }

        return sign | MAX;
    }

    if a_zero {
        return sign;
    }

    let frac_mask = const { 1u8 << F } - 1;
    let exp_mask = const { 1u8 << E } - 1;

    let a_exp = (a_abs >> F) & exp_mask;
    let b_exp = (b_abs >> F) & exp_mask;

    let a_frac = a_abs & frac_mask;
    let b_frac = b_abs & frac_mask;

    let (a_sig, a_power) = if a_exp == 0 {
        (a_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            const { 1u64 << F } | a_frac as u64,
            a_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let (b_sig, b_power) = if b_exp == 0 {
        (b_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            const { 1u64 << F } | b_frac as u64,
            b_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let power = a_power - b_power;

    let a_hi = 63 - a_sig.leading_zeros();
    let b_hi = 63 - b_sig.leading_zeros();

    let mut ratio_exp = a_hi as i32 - b_hi as i32;

    if a_sig << b_hi < b_sig << a_hi {
        ratio_exp -= 1;
    }

    let mut target_exp = power + ratio_exp;

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_exp = min_normal_exp - F as i32;

    if target_exp < min_normal_exp {
        let scale = power - min_subnormal_exp;

        let sub_sig = if scale >= 0 {
            round_div_u64(a_sig << scale as u32, b_sig)
        } else {
            round_div_u64(a_sig, b_sig << (-scale) as u32)
        };

        if sub_sig == 0 {
            return sign;
        }

        if sub_sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sub_sig as u8;
    }

    let scale = power - target_exp + F as i32;

    let mut mantissa = if scale >= 0 {
        round_div_u64(a_sig << scale as u32, b_sig)
    } else {
        round_div_u64(a_sig, b_sig << (-scale) as u32)
    };

    if mantissa >= (1u64 << (F + 1)) {
        mantissa >>= 1;
        target_exp += 1;
    }

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };
    let encoded_exp = target_exp + B as i32;

    if encoded_exp > max_exp as i32 {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    let encoded = ((encoded_exp as u8) << F) | (mantissa as u8 & frac_mask);

    if encoded > MAX {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    sign | encoded
}
