use crate::bit::{
    NO_INF, ops::{encode_finite, encode_zero, is_nan, sign_mask},
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

    let sm = const { sign_mask::<W>() };

    let a_abs = a & !sm;
    let b_abs = b & !sm;
    let c_abs = c & !sm;

    let a_sign = a & sm;
    let b_sign = b & sm;
    let c_sign = c & sm;

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
            if c_zero {
                let negative = product_sign == c_sign && product_sign != 0;
                return encode_zero::<W, NAN>(negative);
            }
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
        (a_frac as u64, 1i32 - B as i32 - F as i32)
    } else {
        (
            (1u64 << F) | a_frac as u64,
            a_exp as i32 - B as i32 - F as i32,
        )
    };

    let (b_sig, b_power) = if b_exp == 0 {
        (b_frac as u64, 1i32 - B as i32 - F as i32)
    } else {
        (
            (1u64 << F) | b_frac as u64,
            b_exp as i32 - B as i32 - F as i32,
        )
    };

    let (c_sig, c_power) = if c_exp == 0 {
        (c_frac as u64, 1i32 - B as i32 - F as i32)
    } else {
        (
            (1u64 << F) | c_frac as u64,
            c_exp as i32 - B as i32 - F as i32,
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

    let (magnitude, negative) = if product_sign == c_sign || c_zero {
        (product + addend, product_sign != 0)
    } else if product >= addend {
        (product - addend, product_sign != 0)
    } else {
        (addend - product, c_sign != 0)
    };

    if magnitude == 0 {
        return 0;
    }

    encode_finite::<F, E, B, W, MAX, NAN, INF>(
        negative,
        magnitude,
        base_power,
    )
}
