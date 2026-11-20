use core::num::FpCategory;

use num_traits::{AsPrimitive, Float, MulAdd, Num, NumCast, One, ToPrimitive, Zero};

use crate::hf8;

macro_rules! thin_op {
    ($trait:ident $ret:ident $($name:ident)*) => {$(
        fn $name(self) -> $ret {
            <f32 as $trait>::$name(self.to_f32()).into()
        }
    )*};
}

impl Float for hf8 {
    fn infinity() -> Self {
        Self::INFINITY
    }

    fn neg_infinity() -> Self {
        Self::NEG_INFINITY
    }

    fn nan() -> Self {
        Self::NAN
    }

    fn neg_zero() -> Self {
        Self::NEG_ZERO
    }

    fn max_value() -> Self {
        Self::MAX
    }

    fn min_value() -> Self {
        Self::MIN
    }

    fn min_positive_value() -> Self {
        Self::MIN_POSITIVE
    }

    fn epsilon() -> Self {
        Self::EPSILON
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn abs_sub(self, other: Self) -> Self {
        (self - other).max(hf8::from_bits(0))
    }

    thin_op!(
        Float Self
        acos acosh asin asinh atan atanh cos cosh sin sinh tan tanh
        floor ceil trunc round
        exp exp2 exp_m1 log2 log10 ln ln_1p
        cbrt sqrt recip
        fract
        to_degrees to_radians
    );

    thin_op!(
        Float bool
        is_finite is_infinite is_nan is_sign_negative is_sign_positive is_normal is_subnormal
    );

    thin_op!(Float FpCategory classify);

    fn atan2(self, other: Self) -> Self {
        self.to_f32().atan2(other.to_f32()).into()
    }

    fn log(self, base: Self) -> Self {
        self.to_f32().log(base.to_f32()).into()
    }

    fn max(self, other: Self) -> Self {
        self.to_f32().max(other.to_f32()).into()
    }

    fn min(self, other: Self) -> Self {
        self.to_f32().min(other.to_f32()).into()
    }

    fn powf(self, n: Self) -> Self {
        self.to_f32().powf(n.to_f32()).into()
    }

    fn powi(self, n: i32) -> Self {
        self.to_f32().powi(n).into()
    }

    fn signum(self) -> Self {
        self.signum()
    }

    fn sin_cos(self) -> (Self, Self) {
        (self.sin(), self.cos())
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }

    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }

    fn hypot(self, other: Self) -> Self {
        self.to_f32().hypot(other.to_f32()).into()
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        self.to_f32().integer_decode()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        // self.fma(a, b)
        todo!()
    }
}

impl MulAdd for hf8 {
    type Output = Self;

    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        // self.mul_add(a, b)
        todo!()
    }
}

impl NumCast for hf8 {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        Some(hf8::from_f32(n.to_f32()?))
    }
}

impl ToPrimitive for hf8 {
    #[inline]
    fn to_i64(&self) -> Option<i64> {
        (*self).to_f32().to_i64()
    }

    #[inline]
    fn to_u64(&self) -> Option<u64> {
        (*self).to_f32().to_u64()
    }

    #[inline]
    fn to_f32(&self) -> Option<f32> {
        Some((*self).to_f32())
    }

    #[inline]
    fn to_f64(&self) -> Option<f64> {
        Some((*self).to_f64())
    }
}

impl AsPrimitive<hf8> for f32 {
    fn as_(self) -> hf8 {
        hf8::from_f32(self)
    }
}

impl AsPrimitive<hf8> for f64 {
    fn as_(self) -> hf8 {
        hf8::from_f64(self)
    }
}

#[cfg(feature = "half")]
impl<T: Copy + 'static> AsPrimitive<T> for hf8
where 
    f32: AsPrimitive<T>,
{
    fn as_(self) -> T {
        self.to_f32().as_()
    }
}

impl Num for hf8 {
    type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        f32::from_str_radix(str, radix).map(Into::into)
    }
}

impl Zero for hf8 {
    fn is_zero(&self) -> bool {
        self.abs().to_bits() == 0
    }

    fn zero() -> Self {
        Self::ZERO
    }

    fn set_zero(&mut self) {
        *self = Self::ZERO;
    }
}

impl One for hf8 {
    fn is_one(&self) -> bool {
        self.to_bits() == Self::ONE.to_bits()
    }

    fn one() -> Self {
        Self::ONE
    }

    fn set_one(&mut self) {
        *self = Self::ONE;
    }
}
