use crate::bit::{
    ops::{is_nan, sign_mask, encode_finite},
    NO_INF,
};

#[inline(always)]
pub const fn rem<
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

    let a_sign = a & sm;
    let negative = a_sign != 0;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if a_inf {
                return if const { NAN != MAX } {
                    sm
                } else {
                    0
                } | NAN;
            }

            return a;
        }
    }

    if b_zero {
        return if const { NAN != MAX } {
            sm
        } else {
            0
        } | NAN;
    }

    if a_zero {
        return a;
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

    let power_diff = a_power - b_power;

    let (remainder, result_power) = if power_diff >= 0 {
        let mut r = 0u64;

        let hi = 63 - a_sig.leading_zeros();
        let mut i = hi as i32;

        while i >= 0 {
            r = (r << 1) | ((a_sig >> i as u32) & 1);

            if r >= b_sig {
                r -= b_sig;
            }

            i -= 1;
        }

        let mut i = 0;

        while i < power_diff {
            r <<= 1;

            if r >= b_sig {
                r -= b_sig;
            }

            i += 1;
        }

        (r, b_power)
    } else {
        let shift = (-power_diff) as u32;

        if shift >= 64 || b_sig > (a_sig >> shift) {
            return a;
        }

        let divisor = b_sig << shift;

        (a_sig % divisor, a_power)
    };

    encode_finite::<F, E, B, W, MAX, NAN, INF>(
        negative,
        remainder,
        result_power,
    )
}
