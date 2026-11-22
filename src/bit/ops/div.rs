use crate::bit::{
    ops::{is_nan, sign_mask},
    NO_INF,
};

#[inline(always)]
pub const fn div<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    a: u8,
    b: u8,
) -> u8 {
    if is_nan::<F, E, W, MAX, NAN, INF>(a) || is_nan::<F, E, W, MAX, NAN, INF>(b) {
        return NAN;
    }

    let sign_mask = const { sign_mask::<W>() };

    let a_abs = a & !sign_mask;
    let b_abs = b & !sign_mask;

    let sign = (a ^ b) & sign_mask;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if a_inf && b_inf {
                return sign_mask | NAN;
            }

            if a_inf {
                return sign | INF;
            }

            return sign;
        }
    }

    if b_zero {
        if a_zero {
            return sign_mask | NAN;
        }

        return sign | MAX;
    }

    if a_zero {
        return sign;
    }

    let frac_mask = const { (1u8 << F) - 1 };
    let exp_mask = const { (1u8 << E) - 1 };

    let a_exp = (a_abs >> F) & exp_mask;
    let b_exp = (b_abs >> F) & exp_mask;

    let a_frac = a_abs & frac_mask;
    let b_frac = b_abs & frac_mask;

    let (a_sig, a_power) = if a_exp == 0 {
        (a_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            (const { 1u64 << F }) | a_frac as u64,
            a_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let (b_sig, b_power) = if b_exp == 0 {
        (b_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            (const { 1u64 << F }) | b_frac as u64,
            b_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let power = a_power - b_power;

    let a_hi = 63 - a_sig.leading_zeros();
    let b_hi = 63 - b_sig.leading_zeros();

    let mut ratio_exp = a_hi as i32 - b_hi as i32;

    if (a_sig << b_hi) < (b_sig << a_hi) {
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
