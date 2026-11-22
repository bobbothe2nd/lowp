use crate::bit::{NO_INF, round_shift_u64};

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

    let src_sign_mask = 1 << (SRC_W - 1);
    let dst_sign_mask = 1 << (DST_W - 1);

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

    let src_max_exp = if const { SRC_INF != NO_INF } {
        src_exp_mask
    } else {
        src_exp_mask
    };

    if src_exp == src_max_exp {
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
        (
            src_frac as u64,
            1 - SRC_B - (SRC_F as i8),
        )
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
        return sign | if const { DST_INF == NO_INF } {
            DST_MAX
        } else {
            DST_INF
        };
    }

    sign
        | ((dst_exp as u8) << DST_F)
        | (sig as u8 & ((1 << DST_F) - 1))
}
