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
pub(crate) const fn from_f32(x: f32) -> u8 {
    crate::bit::cvt::from_f32::<
        { crate::f4::FORMAT.frac },
        { crate::f4::FORMAT.exp },
        { crate::f4::FORMAT.bias },
        { crate::f4::FORMAT.width },
        { crate::f4::FORMAT.max },
        { crate::f4::FORMAT.nan },
        {
            match crate::f4::FORMAT.inf {
                Some(v) => v,
                None => crate::bit::NO_INF,
            }
        },
    >(x)
}

#[inline(always)]
pub(crate) const fn from_f64(x: f64) -> u8 {
    crate::bit::cvt::from_f64::<
        { crate::f4::FORMAT.frac },
        { crate::f4::FORMAT.exp },
        { crate::f4::FORMAT.bias },
        { crate::f4::FORMAT.width },
        { crate::f4::FORMAT.max },
        { crate::f4::FORMAT.nan },
        {
            match crate::f4::FORMAT.inf {
                Some(v) => v,
                None => crate::bit::NO_INF,
            }
        },
    >(x)
}

macro_rules! lut {
    (
        $lut:ident,
        $backup:ident,
        $a:ident,
        $b:ident,
        $c:ident$(,)?
    ) => {{
        unsafe {
            ::core::hint::assert_unchecked($a & 0xf0 == 0);
            ::core::hint::assert_unchecked($b & 0xf0 == 0);
            ::core::hint::assert_unchecked($c & 0xf0 == 0);
        }
        #[cfg(feature = "lut-f4")]
        {
            let idx = (($c as usize) << 8) | (($a as usize) << 4) | ($b as usize);

            unsafe {
                ::core::hint::assert_unchecked(idx < super::lut::$lut.len());
            }

            super::lut::$lut[idx] & 0x0f
        }
        #[cfg(not(feature = "lut-f4"))]
        {
            super::ops::$backup::<
                { $crate::f4::FORMAT.frac },
                { $crate::f4::FORMAT.exp },
                { $crate::f4::FORMAT.bias },
                { $crate::f4::FORMAT.width },
                { $crate::f4::FORMAT.max },
                { $crate::f4::FORMAT.nan },
                {
                    match $crate::f4::FORMAT.inf {
                        Some(v) => v,
                        None => $crate::bit::NO_INF,
                    }
                },
            >($a, $b, $c)
        }
    }};

    (
        $lut:ident,
        $backup:ident,
        $a:ident,
        $b:ident,
        $c:literal$(,)?
    ) => {{
        unsafe {
            ::core::hint::assert_unchecked($a & 0xf0 == 0);
            ::core::hint::assert_unchecked($b & 0xf0 == 0);
        }
        #[cfg(feature = "lut-f4")]
        {
            let idx = ($c << 8) | (($a as usize) << 4) | ($b as usize);

            unsafe {
                ::core::hint::assert_unchecked(idx < super::lut::$lut.len());
            }

            super::lut::$lut[idx] >> 4
        }
        #[cfg(not(feature = "lut-f4"))]
        {
            super::ops::$backup::<
                { $crate::f4::FORMAT.frac },
                { $crate::f4::FORMAT.exp },
                { $crate::f4::FORMAT.bias },
                { $crate::f4::FORMAT.width },
                { $crate::f4::FORMAT.max },
                { $crate::f4::FORMAT.nan },
                {
                    match $crate::f4::FORMAT.inf {
                        Some(v) => v,
                        None => $crate::bit::NO_INF,
                    }
                },
            >($a, $b)
        }
    }};

    (
        $lut:ident,
        $backup:ident,
        $a:ident,
        $b:literal,
        $c:literal$(,)?
    ) => {{
        unsafe {
            ::core::hint::assert_unchecked($a & 0xf0 == 0);
        }
        #[cfg(feature = "lut-f4")]
        {
            let idx = ($c << 8) | (($a as usize) << 4) | ($b as usize);

            unsafe {
                ::core::hint::assert_unchecked(idx < super::lut::$lut.len());
            }

            super::lut::$lut[idx] >> 4
        }
        #[cfg(not(feature = "lut-f4"))]
        {
            super::ops::$backup::<
                { $crate::f4::FORMAT.frac },
                { $crate::f4::FORMAT.exp },
                { $crate::f4::FORMAT.bias },
                { $crate::f4::FORMAT.width },
                { $crate::f4::FORMAT.max },
                { $crate::f4::FORMAT.nan },
                {
                    match $crate::f4::FORMAT.inf {
                        Some(v) => v,
                        None => $crate::bit::NO_INF,
                    }
                },
            >($a)
        }
    }};
}

#[inline(always)]
pub const unsafe fn add(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, add, a, b, 0)
}

#[inline(always)]
pub const unsafe fn mul(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, mul, a, b, 1)
}

#[inline(always)]
pub const unsafe fn sub(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, sub, a, b, 2)
}

#[inline(always)]
pub const unsafe fn div(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, div, a, b, 3)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn div_euclid(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 4)
}

#[inline(always)]
pub const unsafe fn rem(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, rem, a, b, 5)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn log(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 6)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn atan2(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 7)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn powf(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 8)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn hypot(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 9)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn midpoint(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 10)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn rem_euclid(a: u8, b: u8) -> u8 {
    lut!(LUT_F4, __none, a, b, 11)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn cos(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 0, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn sin(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 1, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn acos(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 2, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn asin(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 3, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn acosh(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 4, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn asinh(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 5, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn ln(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 6, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn ln_1p(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 7, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn log2(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 8, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn log10(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 9, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn tan(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 10, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn tanh(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 11, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn atan(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 12, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn atanh(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 13, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn exp(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 14, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn exp2(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 15, 12)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn exp_m1(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 0, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn sqrt(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 1, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn cbrt(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 2, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn recip(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 3, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn floor(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 4, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn ceil(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 5, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn trunc(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 6, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn to_degrees(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 7, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn to_radians(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 8, 13)
}

#[inline(always)]
#[cfg(feature = "lut-f4")]
pub const unsafe fn fract(a: u8) -> u8 {
    lut!(LUT_F4, inv1, a, 9, 13)
}

#[inline(always)]
pub const unsafe fn fma(a: u8, b: u8, c: u8) -> u8 {
    lut!(LUT_F4, fma, a, b, c)
}
