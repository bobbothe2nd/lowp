use crate::bit::{NO_INF, ops::{is_nan, sign_mask}, round_shift_u64};

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
    if is_nan::<F, E, W, MAX, NAN, INF>(a) || is_nan::<F, E, W, MAX, NAN, INF>(b) {
        return NAN;
    }

    let a_abs = a & !sign_mask::<W>();
    let b_abs = b & !sign_mask::<W>();

    let sa = a & sign_mask::<W>();
    let sb = b & sign_mask::<W>();
    let sign = sa ^ sb;

    let a_zero = a_abs == 0;
    let b_zero = b_abs == 0;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            if (a_inf && b_zero) || (b_inf && a_zero) {
                return const { sign_mask::<W>() | NAN };
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
