use crate::bit::{round_shift_u64, NO_INF};

mod add;
mod div;
mod fma;
mod mul;
mod rem;

pub use self::{div::div, fma::fma, mul::mul, rem::rem};

#[inline(always)]
pub const fn add<
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
    add::addsub::<F, E, B, W, MAX, NAN, INF, false>(a, b)
}

#[inline(always)]
pub const fn sub<
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
    add::addsub::<F, E, B, W, MAX, NAN, INF, true>(a, b)
}

#[inline(always)]
pub const fn is_nan<
    const F: u8,
    const E: u8,
    const W: u8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    x: u8,
) -> bool {
    if const { MAX == NAN && W != 8 } {
        false
    } else {
        crate::bit::float8::is_nan::<F, E, NAN, INF>(x)
    }
}

#[inline(always)]
pub const fn sign_mask<const W: u8>() -> u8 {
    const { 1 << (W - 1) }
}

#[inline(always)]
pub const fn encode_zero<const W: u8, const NAN: u8>(negative: bool) -> u8 {
    if const { NAN == sign_mask::<W>() } {
        0
    } else if negative {
        sign_mask::<W>()
    } else {
        0
    }
}

#[inline(always)]
pub const fn encode_finite<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
>(
    sign: bool,
    significand: u64,
    power: i32,
) -> u8 {
    let sign_mask = sign_mask::<W>();
    let sign = if sign { sign_mask } else { 0 };

    if significand == 0 {
        return if const { INF == NO_INF && MAX == 0 } {
            0
        } else {
            sign
        };
    }

    let min_normal_exp = const { 1 - B as i32 };
    let min_subnormal_exp = min_normal_exp - F as i32;

    let hi = 63 - significand.leading_zeros();
    let unbiased_exp = hi as i32 + power;

    if unbiased_exp < min_normal_exp {
        let shift = min_subnormal_exp - power;

        let sig = if shift > 0 {
            round_shift_u64(significand, shift as u32)
        } else {
            significand << (-shift) as u32
        };

        if sig >= const { 1u64 << F } {
            return sign | const { 1u8 << F };
        }

        return sign | sig as u8;
    }

    let mut target_exp = unbiased_exp;

    let shift = hi as i32 - F as i32;

    let mut sig = if shift > 0 {
        round_shift_u64(significand, shift as u32)
    } else {
        significand << (-shift) as u32
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

    let frac_mask = const { (1u8 << F) - 1 };

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
