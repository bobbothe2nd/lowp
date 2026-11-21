use core::hint::unreachable_unchecked;

macro_rules! match_f4_cvt {
    ($x:ident) => {
        match $x {
            0x0 => 0.0,
            0x1 => 0.5,
            0x2 => 1.0,
            0x3 => 1.5,
            0x4 => 2.0,
            0x5 => 3.0,
            0x6 => 4.0,
            0x7 => 6.0,
            0x8 => -0.0,
            0x9 => -0.5,
            0xA => -1.0,
            0xB => -1.5,
            0xC => -2.0,
            0xD => -3.0,
            0xE => -4.0,
            0xF => -6.0,
            _ => unsafe { unreachable_unchecked() },
        }
    };

    ($x:ident $cvt:path) => {
        match $x {
            0x0 => $cvt(0.0),
            0x1 => $cvt(0.5),
            0x2 => $cvt(1.0),
            0x3 => $cvt(1.5),
            0x4 => $cvt(2.0),
            0x5 => $cvt(3.0),
            0x6 => $cvt(4.0),
            0x7 => $cvt(6.0),
            0x8 => $cvt(-0.0),
            0x9 => $cvt(-0.5),
            0xA => $cvt(-1.0),
            0xB => $cvt(-1.5),
            0xC => $cvt(-2.0),
            0xD => $cvt(-3.0),
            0xE => $cvt(-4.0),
            0xF => $cvt(-6.0),
            _ => unsafe { unreachable_unchecked() },
        }
    };
}

#[inline(always)]
pub const unsafe fn to_f32(x: u8) -> f32 {
    match_f4_cvt!(x)
}

#[inline(always)]
pub const unsafe fn to_f64(x: u8) -> f64 {
    match_f4_cvt!(x)
}

#[inline(always)]
#[cfg(feature = "half")]
pub const unsafe fn to_f16(x: u8) -> half::f16 {
    match_f4_cvt!(x half::f16::from_f32_const)
}

#[inline(always)]
#[cfg(feature = "half")]
pub const unsafe fn to_bf16(x: u8) -> half::bf16 {
    match_f4_cvt!(x half::bf16::from_f32_const)
}

#[inline(always)]
pub(crate) const fn from_float<const E: u32, const M: u32, const B: i32>(x: u64) -> u8 {
    const FP4_BIAS: i32 = 1;
    const FP4_SIG_BITS: u32 = 2;
    const FP4_MANT_MASK: u8 = 0x1;
    const FP4_MAXNORM: u8 = 0x7;

    let sign = ((x >> (E + M)) as u8) << 3;
    let absx = x & ((1 << (E + M)) - 1);

    let exp_mask = (1 << E) - 1;
    let exp_field = (absx >> M) & exp_mask;
    let mantissa_mask = (1 << M) - 1;
    let mantissa = absx & mantissa_mask;

    let inf_bits = exp_mask << M;

    let mindenorm_o2 = ((B - 2) as u64) << M;

    let overflow_threshold = ((B + 2) as u64) << M | (1 << (M - 1));

    let minnorm = (B as u64) << M;

    let exp = exp_field as i32 - B + FP4_BIAS;

    let fp4_dp_half_ulp = 1 << const { M - FP4_SIG_BITS };

    let mut sign = sign;
    let mut res: u8;

    if absx <= mindenorm_o2 {
        res = 0;
    } else if absx > overflow_threshold {
        if absx > inf_bits {
            sign = 0;
        }

        res = FP4_MAXNORM;
    } else if absx >= minnorm {
        res = ((exp as u8) << (FP4_SIG_BITS - 1))
            | ((mantissa >> (M - (FP4_SIG_BITS - 1))) as u8 & FP4_MANT_MASK);

        let round = absx & ((fp4_dp_half_ulp << 1) - 1);

        let halfway = fp4_dp_half_ulp;
        let fp4_mantissa = res & FP4_MANT_MASK;

        if round > halfway || (round == halfway && (fp4_mantissa & 1) != 0) {
            res = res.wrapping_add(1);
        }
    } else {
        let shift = if exp >= 1 { 0 } else { (1 - exp) as u32 };

        let denorm_mant = (1u64 << M) | mantissa;

        let total_shift = M - (FP4_SIG_BITS - 1) + shift;

        res = (denorm_mant >> total_shift) as u8;

        let round_mask = (1u64 << total_shift) - 1;
        let round = denorm_mant & round_mask;

        let halfway = 1u64 << (total_shift - 1);

        if round > halfway || (round == halfway && (res & 1) != 0) {
            res = res.wrapping_add(1);
        }
    }

    res | sign
}

#[inline(always)]
#[cfg(feature = "half")]
pub(crate) const fn from_f16(x: half::f16) -> u8 {
    from_float::<5, 10, 15>(x.to_bits() as u64)
}

#[inline(always)]
#[cfg(feature = "half")]
pub(crate) const fn from_bf16(x: half::bf16) -> u8 {
    from_float::<8, 7, 127>(x.to_bits() as u64)
}

#[inline(always)]
pub(crate) const fn from_f32(x: f32) -> u8 {
    from_float::<8, 23, 127>(x.to_bits() as u64)
}

#[inline(always)]
pub(crate) const fn from_f64(x: f64) -> u8 {
    from_float::<11, 52, 1023>(x.to_bits())
}

macro_rules! lut {
    (
        $lut:ident,
        $a:ident,
        $b:ident,
        $c:ident$(,)?
    ) => {{
        unsafe {
            ::core::hint::assert_unchecked($a & 0xf0 == 0);
            ::core::hint::assert_unchecked($b & 0xf0 == 0);
            ::core::hint::assert_unchecked($c & 0xf0 == 0);
        }

        let idx = (($c as usize) << 8) | (($a as usize) << 4) | ($b as usize);

        unsafe {
            ::core::hint::assert_unchecked(idx < super::lut::$lut.len());
        }

        super::lut::$lut[idx] & 0x0f
    }};

    (
        $lut:ident,
        $a:ident,
        $b:expr,
        $c:literal$(,)?
    ) => {{
        unsafe {
            ::core::hint::assert_unchecked($a & 0xf0 == 0);
            ::core::hint::assert_unchecked($b & 0xf0 == 0);
        }

        let idx = ($c << 8) | (($a as usize) << 4) | ($b as usize);

        unsafe {
            ::core::hint::assert_unchecked(idx < super::lut::$lut.len());
        }

        super::lut::$lut[idx] >> 4
    }};

    (
        $lut:ident,
        $a:ident,
        $b:ident$(,)?
    ) => {{
        unsafe {
            ::core::hint::assert_unchecked($a & 0xf0 == 0);
            ::core::hint::assert_unchecked($b & 0xf0 == 0);
        }

        let idx = (($a as usize) << 4) | ($b as usize);

        unsafe {
            ::core::hint::assert_unchecked(idx < super::lut::$lut.len());
        }

        super::lut::$lut[idx]
    }};
}

#[inline(always)]
pub const unsafe fn add(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 0)
}

#[inline(always)]
pub const unsafe fn mul(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 1)
}

#[inline(always)]
pub const unsafe fn sub(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 2)
}

#[inline(always)]
pub const unsafe fn div(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 3)
}

#[inline(always)]
pub const unsafe fn div_euclid(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 4)
}

#[inline(always)]
pub const unsafe fn rem(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 5)
}

#[inline(always)]
pub const unsafe fn log(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 6)
}

#[inline(always)]
pub const unsafe fn atan2(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 7)
}

#[inline(always)]
pub const unsafe fn powf(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 8)
}

#[inline(always)]
pub const unsafe fn hypot(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 9)
}

#[inline(always)]
pub const unsafe fn midpoint(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 10)
}

#[inline(always)]
pub const unsafe fn rem_euclid(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, a, b, 11)
}

#[inline(always)]
pub const unsafe fn cos(a: u8) -> u8 {
    lut!(LUT_F4, a, 0, 12)
}

#[inline(always)]
pub const unsafe fn sin(a: u8) -> u8 {
    lut!(LUT_F4, a, 1, 12)
}

#[inline(always)]
pub const unsafe fn acos(a: u8) -> u8 {
    lut!(LUT_F4, a, 2, 12)
}

#[inline(always)]
pub const unsafe fn asin(a: u8) -> u8 {
    lut!(LUT_F4, a, 3, 12)
}

#[inline(always)]
pub const unsafe fn acosh(a: u8) -> u8 {
    lut!(LUT_F4, a, 4, 12)
}

#[inline(always)]
pub const unsafe fn asinh(a: u8) -> u8 {
    lut!(LUT_F4, a, 5, 12)
}

#[inline(always)]
pub const unsafe fn ln(a: u8) -> u8 {
    lut!(LUT_F4, a, 6, 12)
}

#[inline(always)]
pub const unsafe fn ln_1p(a: u8) -> u8 {
    lut!(LUT_F4, a, 7, 12)
}

#[inline(always)]
pub const unsafe fn log2(a: u8) -> u8 {
    lut!(LUT_F4, a, 8, 12)
}

#[inline(always)]
pub const unsafe fn log10(a: u8) -> u8 {
    lut!(LUT_F4, a, 9, 12)
}

#[inline(always)]
pub const unsafe fn tan(a: u8) -> u8 {
    lut!(LUT_F4, a, 10, 12)
}

#[inline(always)]
pub const unsafe fn tanh(a: u8) -> u8 {
    lut!(LUT_F4, a, 11, 12)
}

#[inline(always)]
pub const unsafe fn atan(a: u8) -> u8 {
    lut!(LUT_F4, a, 12, 12)
}

#[inline(always)]
pub const unsafe fn atanh(a: u8) -> u8 {
    lut!(LUT_F4, a, 13, 12)
}

#[inline(always)]
pub const unsafe fn exp(a: u8) -> u8 {
    lut!(LUT_F4, a, 14, 12)
}

#[inline(always)]
pub const unsafe fn exp2(a: u8) -> u8 {
    lut!(LUT_F4, a, 15, 12)
}

#[inline(always)]
pub const unsafe fn exp_m1(a: u8) -> u8 {
    lut!(LUT_F4, a, 0, 13)
}

#[inline(always)]
pub const unsafe fn sqrt(a: u8) -> u8 {
    lut!(LUT_F4, a, 1, 13)
}

#[inline(always)]
pub const unsafe fn cbrt(a: u8) -> u8 {
    lut!(LUT_F4, a, 2, 13)
}

#[inline(always)]
pub const unsafe fn recip(a: u8) -> u8 {
    lut!(LUT_F4, a, 3, 13)
}

#[inline(always)]
pub const unsafe fn floor(a: u8) -> u8 {
    lut!(LUT_F4, a, 4, 13)
}

#[inline(always)]
pub const unsafe fn ceil(a: u8) -> u8 {
    lut!(LUT_F4, a, 5, 13)
}

#[inline(always)]
pub const unsafe fn trunc(a: u8) -> u8 {
    lut!(LUT_F4, a, 6, 13)
}

#[inline(always)]
pub const unsafe fn to_degrees(a: u8) -> u8 {
    lut!(LUT_F4, a, 7, 13)
}

#[inline(always)]
pub const unsafe fn to_radians(a: u8) -> u8 {
    lut!(LUT_F4, a, 8, 13)
}

#[inline(always)]
pub const unsafe fn fract(a: u8) -> u8 {
    lut!(LUT_F4, a, 9, 13)
}

#[inline(always)]
pub const unsafe fn fma(a: u8, b: u8, c: u8) -> u8 {
    lut!(LUT_F4, a, b, c)
}
