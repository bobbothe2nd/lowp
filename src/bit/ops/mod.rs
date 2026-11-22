pub mod add;
pub mod fma;
pub mod mul;
pub mod div;
pub mod rem;

#[inline(always)]
pub const fn is_nan<const F: u8, const E: u8, const W: u8, const MAX: u8, const NAN: u8, const INF: u8>(x: u8) -> bool {
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
