use crate::bit::{
    ops::{encode_finite, is_nan, sign_mask},
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

    let sm = const { sign_mask::<W>() };
    let sign_toggle = if SUB { sm } else { 0 };

    let a_abs = a & !sm;
    let b_abs = b & !sm;

    if const { INF != NO_INF } {
        let a_inf = a_abs == INF;
        let b_inf = b_abs == INF;

        if a_inf || b_inf {
            let sa = a & sm;
            let sb = (b & sm) ^ sign_toggle;

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

    let sa = (a & sm != 0) ^ false;
    let sb = (b & sm != 0) ^ SUB;

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

    encode_finite::<F, E, B, W, MAX, NAN, INF>(sum < 0, sum.unsigned_abs(), 1 - B as i32 - F as i32)
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
