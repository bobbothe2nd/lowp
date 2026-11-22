use crate::bit::{
    ops::{is_nan, sign_mask},
    NO_INF,
};

#[inline(always)]
pub const fn addsub<
    const F: u8,
    const E: u8,
    const B: i8,
    const W: u8,
    const MAX: u8,
    const NAN: u8,
    const INF: u8,
    const SUB: bool,
>(
    a: u8,
    b: u8,
) -> u8 {
    if is_nan::<F, E, W, MAX, NAN, INF>(a) || is_nan::<F, E, W, MAX, NAN, INF>(b) {
        return NAN;
    }

    let sign_mask = const { sign_mask::<W>() };
    let sign_toggle = if SUB { sign_mask } else { 0 };

    let a_abs = a & !sign_mask;
    let b_abs = b & !sign_mask;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            let sa = a & sign_mask;
            let sb = (b & sign_mask) ^ sign_toggle;

            if a_inf && b_inf && sa != sb {
                return NAN;
            }

            return if a_inf { a } else { b ^ sign_toggle };
        }
    }

    if a_abs == 0 {
        if b_abs == 0 {
            return if SUB { a & !b } else { a & b };
        }

        return b ^ sign_toggle;
    }

    if b_abs == 0 {
        return a;
    }

    let sa = a & sign_mask != 0;
    let sb = (b & sign_mask != 0) ^ SUB;

    let mut ma = decode_finite::<F, E>(a_abs);
    let mut mb = decode_finite::<F, E>(b_abs);

    if sa {
        ma = -ma;
    }

    if sb {
        mb = -mb;
    }

    let sum = ma + mb;

    if sum == 0 {
        return 0;
    }

    let sign = if sum < 0 { sign_mask } else { 0 };
    let q = sum.unsigned_abs();

    if q < const { 1 << F } {
        return sign | q as u8;
    }

    let h = 63 - q.leading_zeros();
    let shift = h - (F as u32);

    let mut mantissa = q >> shift;

    if shift != 0 {
        let remainder_mask = (1 << shift) - 1;
        let remainder = q & remainder_mask;
        let halfway = 1 << (shift - 1);

        if remainder > halfway || (remainder == halfway && (mantissa & 1) != 0) {
            mantissa += 1;
        }
    }

    let mut exponent = h as i32 - (B as i32 + F as i32 - 1);

    if mantissa >= (1 << (F + 1)) {
        mantissa >>= 1;
        exponent += 1;
    }

    let encoded_exp = exponent + B as i32;
    let max_exp = ((1u16 << E) - 1) as i32;

    if encoded_exp > max_exp {
        return sign | MAX;
    }

    let encoded = ((encoded_exp as u8) << F) | (mantissa - const { 1 << F }) as u8;

    if encoded > MAX {
        return sign | MAX;
    }

    sign | encoded
}

#[inline(always)]
const fn decode_finite<const F: u8, const E: u8>(x: u8) -> i64 {
    let frac_mask = const { 1u8 << F } - 1;
    let e = (x >> F) & (const { 1u8 << E } - 1);
    let frac = x & frac_mask;

    if e == 0 {
        frac as i64
    } else {
        (const { 1 << F } + frac as i64) << (e - 1)
    }
}
