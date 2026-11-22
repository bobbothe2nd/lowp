use crate::bit::{
    NO_INF, ops::{encode_finite, encode_zero, is_nan, sign_mask},
};

#[inline(always)]
pub const fn mul<
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
    if is_nan::<F, E, W, MAX, NAN, INF>(a)
        || is_nan::<F, E, W, MAX, NAN, INF>(b)
    {
        return NAN;
    }

    let sm = const { sign_mask::<W>() };

    let a_abs = a & !sm;
    let b_abs = b & !sm;

    let sign = (a ^ b) & sm;
    let negative = sign != 0;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if (a_inf && b_zero) || (b_inf && a_zero) {
                return NAN;
            }

            return sign | INF;
        }
    }

    if a_zero || b_zero {
        return encode_zero::<W, NAN>(negative);
    }

    let frac_mask = const { (1u8 << F) - 1 };
    let exp_mask = const { (1u8 << E) - 1 };

    let a_exp = (a_abs >> F) & exp_mask;
    let b_exp = (b_abs >> F) & exp_mask;

    let a_frac = a_abs & frac_mask;
    let b_frac = b_abs & frac_mask;

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

    encode_finite::<F, E, B, W, MAX, NAN, INF>(
        negative,
        a_sig * b_sig,
        a_power + b_power,
    )
}
