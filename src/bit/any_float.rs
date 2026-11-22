use crate::bit::NO_INF;

#[inline(always)]
pub const fn is_nan<
    const F: u8,
    const E: u8,
    const W: u8,
    const NAN: u8,
    const INF: u8,
>(x: u8) -> bool {
    let width_mask = const { (1u8 << W) - 1 };
    let x = x & width_mask;

    if const { INF != NO_INF } {
        let exp_mask = const { (1u8 << E) - 1 };
        let frac_mask = const { (1u8 << F) - 1 };
        let exp = (x >> F) & exp_mask;
        let frac = x & frac_mask;

        exp == exp_mask && frac != 0
    } else if const { NAN == 0x7f } {
        x == 0x7f || x == width_mask ^ 0x7f
    } else {
        x == NAN
    }
}
