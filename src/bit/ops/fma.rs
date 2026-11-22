use crate::bit::{
    ops::{is_nan, sign_mask},
    round_shift_u64, NO_INF,
};

#[inline(always)]
pub const fn fma<
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
    c: u8,
) -> u8 {
    if is_nan::<F, E, W, MAX, NAN, INF>(a)
        || is_nan::<F, E, W, MAX, NAN, INF>(b)
        || is_nan::<F, E, W, MAX, NAN, INF>(c)
    {
        return NAN;
    }

    let sign_mask = const { sign_mask::<W>() };

    let a_abs = a & !sign_mask;
    let b_abs = b & !sign_mask;
    let c_abs = c & !sign_mask;

    let a_sign = a & sign_mask;
    let b_sign = b & sign_mask;
    let c_sign = c & sign_mask;

    let product_sign = a_sign ^ b_sign;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;
    let c_zero = c_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;
        let c_inf = c_abs == INF;

        if a_inf || b_inf {
            if (a_inf && b_zero) || (b_inf && a_zero) {
                return NAN;
            }

            if c_inf && product_sign != c_sign {
                return NAN;
            }

            return product_sign | INF;
        }

        if c_inf {
            return c;
        }
    }

    if a_zero || b_zero {
        if c_zero {
            return if product_sign == c_sign {
                product_sign
            } else {
                0
            };
        }

        return c;
    }

    let frac_mask = const { (1u8 << F) - 1 };
    let exp_mask = const { (1u8 << E) - 1 };

    let a_exp = (a_abs >> F) & exp_mask;
    let b_exp = (b_abs >> F) & exp_mask;
    let c_exp = (c_abs >> F) & exp_mask;

    let a_frac = a_abs & frac_mask;
    let b_frac = b_abs & frac_mask;
    let c_frac = c_abs & frac_mask;

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

    let (c_sig, c_power) = if c_exp == 0 {
        (c_frac as u64, const { 1i32 - B as i32 - F as i32 })
    } else {
        (
            (const { 1u64 << F }) | c_frac as u64,
            c_exp as i32 - const { B as i32 + F as i32 },
        )
    };

    let product_sig = a_sig * b_sig;
    let product_power = a_power + b_power;

    let base_power = if c_zero || product_power < c_power {
        product_power
    } else {
        c_power
    };

    let product = product_sig << (product_power - base_power) as u32;

    let addend = if c_zero {
        0
    } else {
        c_sig << (c_power - base_power) as u32
    };

    let (magnitude, sign) = if product_sign == c_sign || c_zero {
        (product + addend, product_sign)
    } else if product >= addend {
        (product - addend, product_sign)
    } else {
        (addend - product, c_sign)
    };

    if magnitude == 0 {
        return 0;
    }

    let result_hi = 63 - magnitude.leading_zeros();
    let unbiased_exp = result_hi as i32 + base_power;

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_power = min_normal_exp - F as i32;

    if unbiased_exp < min_normal_exp {
        let shift = min_subnormal_power - base_power;

        let sig = if shift > 0 {
            round_shift_u64(magnitude, shift as u32)
        } else {
            magnitude << (-shift) as u32
        };

        if sig == 0 {
            return sign;
        }

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let mut target_exp = unbiased_exp;

    let shift = result_hi as i32 - F as i32;

    let mut sig = if shift > 0 {
        round_shift_u64(magnitude, shift as u32)
    } else {
        magnitude << (-shift) as u32
    };

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;
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

    let encoded = ((encoded_exp as u8) << F) | (sig as u8 & frac_mask);

    if encoded > MAX {
        return if const { INF != NO_INF } {
            sign | INF
        } else {
            sign | MAX
        };
    }

    sign | encoded
}
