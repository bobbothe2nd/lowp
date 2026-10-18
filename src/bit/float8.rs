use crate::bit::{EXP32_MASK, EXP64_MASK, FRAC32_MASK, FRAC64_MASK, SIGN32, SIGN64};

pub const NO_INF: u8 = 0xff;

const SIGN: u8 = 0x80;

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

    if remainder > halfway
        || (remainder == halfway && (truncated & 1) != 0)
    {
        truncated + 1
    } else {
        truncated
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

    let src_exp = ((bits & EXP32_MASK) >> 23) as u32;
    let src_frac = bits & FRAC32_MASK;

    let max_exp = (1u32 << E) - 1;
    let max_sig = (1u32 << (F + 1)) - 1;

    if src_exp == 0xff {
        if src_frac != 0 {
            return NAN;
        }

        return if INF != NO_INF {
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

    let min_normal_exp = 1 - B as i32;

    let min_subnormal_exp = min_normal_exp - F as i32;

    if exponent < min_subnormal_exp {
        return sign;
    }

    if exponent < min_normal_exp {
        let shift =
            (24i32 - B as i32 - F as i32 - exponent) as u32;

        let sig = round_shift_u64(mantissa, shift);

        if sig >= (1u64 << F) {
            return sign | (1u8 << F);
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + B as i32;

    if target_exp > max_exp as i32 {
        return sign | MAX;
    }

    let shift = 23 - F;

    let mut sig = round_shift_u64(mantissa, shift as u32);

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;

        if target_exp + 1 > max_exp as i32 {
            return sign
                | ((max_exp << F) as u8
                    | (max_sig - 1) as u8);
        }

        let encoded = ((target_exp + 1) as u8) << F;
        return sign | encoded | (sig as u8 & ((1 << F) - 1));
    }

    if target_exp == max_exp as i32 && sig >= max_sig as u64 {
        return sign | MAX;
    }

    sign
        | ((target_exp as u8) << F)
        | (sig as u8 & ((1 << F) - 1))
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

    let max_exp = (1u32 << E) - 1;
    let max_sig = (1u32 << (F + 1)) - 1;

    if src_exp == 0x7ff {
        if src_frac != 0 {
            return NAN;
        }

        return if INF != NO_INF {
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
        let shift =
            (53i32 - (B as i32) - F as i32 - exponent) as u32;

        let sig = round_shift_u64(mantissa, shift);

        if sig >= (1u64 << F) {
            return sign | (1u8 << F);
        }

        return sign | sig as u8;
    }

    let target_exp = exponent + (B as i32);

    if target_exp > max_exp as i32 {
        return sign | MAX;
    }

    let shift = 52 - F;

    let mut sig = round_shift_u64(mantissa, shift as u32);

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;

        if target_exp + 1 > max_exp as i32 {
            return sign
                | ((max_exp << F) as u8
                    | (max_sig - 1) as u8);
        }

        let encoded = ((target_exp + 1) as u8) << F;
        return sign
            | encoded
            | (sig as u8 & ((1 << F) - 1));
    }

    if target_exp == max_exp as i32 && sig >= max_sig as u64 {
        return sign | MAX;
    }

    sign
        | ((target_exp as u8) << F)
        | (sig as u8 & ((1 << F) - 1))
}

#[inline(always)]
pub const fn to_f32<
    const F: u8,
    const E: u8,
    const B: i8,
    const NAN: u8,
    const INF: u8,
>(bits: u8) -> f32 {
    let sign = ((bits as u32) & 0x80) << 24;
    let abs = bits & 0x7f;

    if NAN != SIGN && abs == NAN {
        return f32::NAN;
    }

    if INF != NO_INF && abs == INF {
        return f32::from_bits(sign | 0x7f80_0000);
    }

    let exp_mask = (1u8 << E) - 1;
    let frac_mask = (1u8 << F) - 1;
    let exp = (abs >> F) & exp_mask;
    let frac = abs & frac_mask;

    if exp == 0 && frac == 0 {
        return f32::from_bits(sign);
    }

    if exp == 0 {
        let mut sig = frac as u32;
        let mut unbiased_exp = 1 - B as i32;

        while (sig & (1u32 << F)) == 0 {
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
    const NAN: u8,
    const INF: u8,
>(bits: u8) -> f64 {
    let sign = ((bits as u64) & 0x80) << 56;
    let abs = bits & 0x7f;

    if NAN != SIGN && abs == NAN {
        return f64::NAN;
    }

    if INF != NO_INF && abs == INF {
        return f64::from_bits(sign | 0x7ff0_0000_0000_0000);
    }

    let exp_mask = (1u8 << E) - 1;
    let frac_mask = (1u8 << F) - 1;
    let exp = (abs >> F) & exp_mask;
    let frac = abs & frac_mask;

    if exp == 0 && frac == 0 {
        return f64::from_bits(sign);
    }

    if exp == 0 {
        let mut sig = frac as u64;
        let mut unbiased_exp = 1 - B as i32;

        while (sig & (1u64 << F)) == 0 {
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

#[inline(always)]
const fn decode_finite<const F: u8, const E: u8>(
    abs: u8,
) -> i32 {
    let frac_mask = (1u8 << F) - 1;
    let e = (abs >> F) & ((1u8 << E) - 1);
    let frac = abs & frac_mask;

    if e == 0 {
        frac as i32
    } else {
        ((1i32 << F) + frac as i32) << (e - 1)
    }
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
    if (NAN == SIGN) && (a == NAN || b == NAN) {
        return NAN;
    }

    let a_abs = a & !SIGN;
    let b_abs = b & !SIGN;

    if INF != NO_INF {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            let sa = a >> 7;
            let sb = (b >> 7) ^ (SUB as u8);

            if a_inf && b_inf && sa != sb {
                return NAN;
            }

            return if a_inf {
                a
            } else {
                b ^ ((SUB as u8) << 7)
            };
        }
    }

    if (NAN != 0x80) && (a_abs == NAN || b_abs == NAN) {
        return NAN;
    }

    if a_abs == 0 {
        if b_abs == 0 {
            return if SUB {
                a & !b
            } else {
                a & b
            };
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

    if q < (1u32 << F) {
        return sign | q as u8;
    }

    let h = 31 - q.leading_zeros();
    let shift = h - (F as u32);

    let mut significand = q >> shift;

    if shift != 0 {
        let remainder_mask = (1u32 << shift) - 1;
        let remainder = q & remainder_mask;
        let halfway = 1u32 << (shift - 1);

        if remainder > halfway
            || (remainder == halfway && (significand & 1) != 0)
        {
            significand += 1;
        }
    }

    let mut exponent = h as i32 - (B as i32 + F as i32 - 1);

    if significand >= (1 << (F + 1)) {
        significand >>= 1;
        exponent += 1;
    }

    let encoded_exp = exponent + B as i32;
    let max_exp = ((1u16 << E) - 1) as i32;

    if encoded_exp > max_exp {
        return sign | MAX;
    }

    let encoded = ((encoded_exp as u8) << F)
        | (significand - (1 << F)) as u8;

    if encoded > MAX {
        return sign | MAX;
    }

    return sign | encoded;
}
