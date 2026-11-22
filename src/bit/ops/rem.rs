use crate::bit::{NO_INF, ops::{is_nan, sign_mask}, round_shift_u64};

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
    if is_nan::<F, E, W, MAX, NAN, INF>(a) || is_nan::<F, E, W, MAX, NAN, INF>(b) {
        return NAN;
    }

    let a_abs = a & !sign_mask::<W>();
    let b_abs = b & !sign_mask::<W>();

    let a_sign = a & sign_mask::<W>();

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if a_inf || b_zero {
                return const { sign_mask::<W>() | NAN };
            }

            return a;
        }
    }

    if b_zero {
        return const { sign_mask::<W>() | NAN };
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

    let power_diff = a_power - b_power;

    let (remainder, result_power) = if power_diff >= 0 {
        let mut r = 0u64;
        let mut q_odd;

        let hi = 63 - a_sig.leading_zeros();
        let mut i = hi as i32;

        while i >= 0 {
            let bit = (a_sig >> i as u32) & 1;

            r = (r << 1) | bit;

            q_odd = r >= b_sig;

            if q_odd {
                r -= b_sig;
            }

            i -= 1;
        }

        let mut i = 0;

        while i < power_diff {
            r <<= 1;

            q_odd = r >= b_sig;

            if q_odd {
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

        let d = b_sig << shift;

        (a_sig % d, a_power)
    };

    if remainder == 0 {
        return a_sign;
    }

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_exp = min_normal_exp - F as i32;

    let result_hi = 63 - remainder.leading_zeros();
    let unbiased_exp = result_hi as i32 + result_power;

    if unbiased_exp < min_normal_exp {
        let shift = min_subnormal_exp - result_power;

        let sig = if shift > 0 {
            round_shift_u64(remainder, shift as u32)
        } else {
            remainder << (-shift) as u32
        };

        if sig == 0 {
            return a_sign;
        }

        if sig >= const { 1u64 << F } {
            return a_sign | const { 1u8 << F };
        }

        return a_sign | sig as u8;
    }

    let mut target_exp = unbiased_exp;

    let mut sig = if result_hi > F as u32 {
        round_shift_u64(remainder, result_hi - F as u32)
    } else {
        remainder << (F as u32 - result_hi)
    };

    if sig >= (1u64 << (F + 1)) {
        sig >>= 1;
        target_exp += 1;
    }

    let encoded_exp = target_exp + B as i32;

    let max_exp = if const { INF != NO_INF } {
        (1u32 << E) - 2
    } else {
        (1u32 << E) - 1
    };

    if encoded_exp > max_exp as i32 {
        return a_sign | MAX;
    }

    let encoded = ((encoded_exp as u8) << F) | (sig as u8 & frac_mask);

    if encoded > MAX {
        return a_sign | MAX;
    }

    a_sign | encoded
}
