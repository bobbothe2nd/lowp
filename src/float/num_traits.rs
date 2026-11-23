use core::num::FpCategory;

use num_traits::{
    float::{FloatCore, TotalOrder},
    AsPrimitive, Bounded, ConstOne, ConstZero, FloatConst, MulAdd, Num, NumCast, One, ToPrimitive,
    Zero,
};

#[cfg(any(feature = "std", feature = "libm"))]
use num_traits::float::Float;

macro_rules! thin_op {
    ($trait:ident $ret:ident $($name:ident)*) => {$(
        fn $name(self) -> $ret {
            <f32 as $trait>::$name(self.to_f32()).into()
        }
    )*};
}

macro_rules! const_fn {
    ($($name:ident)*) => {$(
        fn $name() -> Self {
            Self::$name
        }
    )*};
}

macro_rules! impl_num_traits {
    ($ty:ident) => {
        impl Bounded for crate::$ty {
            fn max_value() -> Self {
                Self::MAX
            }

            fn min_value() -> Self {
                Self::MIN
            }
        }

        impl FloatConst for $crate::$ty {
            const_fn!(
                E
                FRAC_1_PI
                FRAC_1_SQRT_2
                FRAC_2_PI
                FRAC_2_SQRT_PI
                FRAC_PI_2
                FRAC_PI_3
                FRAC_PI_4
                FRAC_PI_6
                FRAC_PI_8
                LN_2
                LN_10
                LOG2_E
                LOG2_10
                LOG10_E
                LOG10_2
                PI
                SQRT_2
            );
        }

        impl MulAdd for $crate::$ty {
            type Output = Self;

            fn mul_add(self, a: Self, b: Self) -> Self::Output {
                self.mul_add(a, b)
            }
        }

        impl NumCast for $crate::$ty {
            fn from<T: ToPrimitive>(n: T) -> Option<Self> {
                Some(crate::$ty::from_f32(n.to_f32()?))
            }
        }

        impl ToPrimitive for $crate::$ty {
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

        impl AsPrimitive<$crate::$ty> for f32 {
            fn as_(self) -> $crate::$ty {
                $crate::$ty::from_f32(self)
            }
        }

        impl AsPrimitive<$crate::$ty> for f64 {
            fn as_(self) -> $crate::$ty {
                $crate::$ty::from_f64(self)
            }
        }

        #[cfg(feature = "half")]
        impl AsPrimitive<$crate::$ty> for half::f16 {
            fn as_(self) -> $crate::$ty {
                $crate::$ty::from_f16(self)
            }
        }

        #[cfg(feature = "half")]
        impl AsPrimitive<$crate::$ty> for half::bf16 {
            fn as_(self) -> $crate::$ty {
                $crate::$ty::from_bf16(self)
            }
        }

        impl<T: Copy + 'static> AsPrimitive<T> for $crate::$ty
        where
            f32: AsPrimitive<T>,
        {
            fn as_(self) -> T {
                self.to_f32().as_()
            }
        }

        impl Num for $crate::$ty {
            type FromStrRadixErr = <f32 as Num>::FromStrRadixErr;

            fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                f32::from_str_radix(str, radix).map(Into::into)
            }
        }

        impl ConstZero for $crate::$ty {
            const ZERO: Self = Self::ZERO;
        }

        impl ConstOne for $crate::$ty {
            const ONE: Self = Self::ONE;
        }

        impl Zero for $crate::$ty {
            fn is_zero(&self) -> bool {
                self.abs().to_bits() == 0
            }

            fn zero() -> Self {
                Self::ZERO
            }
        }

        impl One for $crate::$ty {
            fn is_one(&self) -> bool {
                self.to_bits() == Self::ONE.to_bits()
            }

            fn one() -> Self {
                Self::ONE
            }
        }
    };
}

impl_num_traits!(f4);
impl_num_traits!(hf6);
impl_num_traits!(bf6);
impl_num_traits!(hf8);
impl_num_traits!(hf8fn);
impl_num_traits!(hf8uz);
impl_num_traits!(bf8);
impl_num_traits!(bf8uz);

macro_rules! impl_float {
    ($ty:ident) => {
        impl FloatCore for $crate::$ty {
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

            thin_op!(
                FloatCore Self
                to_degrees to_radians
            );

            thin_op!(FloatCore FpCategory classify);

            fn integer_decode(self) -> (u64, i16, i8) {
                FloatCore::integer_decode(self.to_f32())
            }
        }

        #[cfg(any(feature = "std", feature = "libm"))]
        impl Float for $crate::$ty {
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
                (self - other).max($crate::$ty::from_bits(0))
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
                Float::powi(self.to_f32(), n).into()
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
                Float::integer_decode(self.to_f32())
            }

            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }
        }
    };
}

impl_float!(hf8);
impl_float!(bf8);

macro_rules! impl_nan {
    ($ty:ident) => {
        impl TotalOrder for crate::$ty {
            fn total_cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.total_cmp(other)
            }
        }
    };
}

impl_nan!(hf8);
impl_nan!(hf8fn);
impl_nan!(hf8uz);
impl_nan!(bf8);
impl_nan!(bf8uz);
