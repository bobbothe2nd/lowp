mod format;

macro_rules! impl_float8 {
    ($name:ident, $format:ident$(,)?) => {
        #[doc = concat!("8-bit floating point value with the ", stringify!($format), " format")]
        #[derive(Clone, Copy, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
        #[cfg_attr(feature = "zerocopy", derive(zerocopy::AsBytes, zerocopy::FromBytes))]
        #[repr(transparent)]
        pub struct $name(u8);

        #[cfg(feature = "briny")]
        unsafe impl briny::traits::Pod for $name {}

        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                if self.is_nan() || other.is_nan() {
                    false
                } else if (self.0 & 0x7f) == 0 && (other.0 & 0x7f) == 0 {
                    true // +0 == -0
                } else {
                    self.0 == other.0
                }
            }
        }

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                #[inline]
                const fn cmp_key(x: u8) -> u8 {
                    if x & 0x80 != 0 {
                        !x
                    } else {
                        x | 0x80
                    }
                }

                if self.is_nan() || other.is_nan() {
                    None
                } else {
                    Some(cmp_key(self.0).cmp(&cmp_key(other.0)))
                }
            }
        }

        impl $name {
            /// NaN value
            pub const NAN: Self = Self::from_bits($format.nan);
            /// Maximum possible value
            pub const MAX: Self = Self::from_bits($format.max);
            /// Largest possible subnormal value
            pub const MAX_SUBNORMAL: Self = Self::from_bits((1 << $format.frac) - 1);
            /// Minimum possible value
            pub const MIN: Self = Self::from_bits($format.max | 0x80);
            /// Smallest possible positive value
            pub const MIN_POSITIVE: Self = Self::from_bits(1 << $format.frac);
            /// Smallest possible subnormal value
            pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(1);
            /// One greater than the minimum possible normal power of 2 exponent
            pub const MIN_EXP: i32 = 2 - ($format.bias as i32);
            /// Minimum possible normal power of 2 exponent
            pub const MAX_EXP: i32 = (((1i32 << $format.exp) - 2) - ($format.bias as i32));
            /// 0
            pub const ZERO: Self = Self::from_bits(0);
            /// 1
            pub const ONE: Self = Self::from_bits(($format.bias as u8) << $format.frac);
            /// -1
            pub const NEG_ONE: Self = Self::from_bits(0x80 | (($format.bias as u8) << $format.frac));
            /// This is the difference between 1.0 and the next largest representable number.
            pub const EPSILON: Self = Self::from_bits(((($format.bias as u8) - $format.frac) << $format.frac));

            /// Construct an 8-bit floating point value from the raw bits.
            #[inline(always)]
            pub const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }

            /// Return the raw bits.
            #[inline]
            pub const fn to_bits(self) -> u8 {
                self.0
            }

            /// Convert a [`prim@f32`] type into [`Self`].
            ///
            /// This operation is lossy.
            ///
            /// - If the 32-bit value is to large to fit in 8-bits, ±∞ will result.
            /// - NaN values are preserved.
            /// - 64-bit subnormal values are too tiny to be represented in 8-bits and result in ±0.
            /// - Exponents that underflow the minimum 8-bit exponent will result in 8-bit subnormals or ±0.
            /// - All other values are truncated and rounded to the nearest representable  8-bit value.
            #[inline]
            pub const fn from_f32(x: f32) -> Self {
                Self($crate::bit::float8::from_f32::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(x))
            }

            /// Converts this type into a [`prim@f32`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 32-bit value.
            #[inline]
            pub const fn to_f32(self) -> f32 {
                $crate::bit::float8::to_f32::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0)
            }

            /// Convert a [`prim@f64`] type into [`Self`].
            ///
            /// This operation is lossy.
            ///
            /// - If the 64-bit value is to large to fit in 8-bits, ±∞ will result.
            /// - NaN values are preserved.
            /// - 64-bit subnormal values are too tiny to be represented in 8-bits and result in ±0.
            /// - Exponents that underflow the minimum 8-bit exponent will result in 8-bit subnormals or ±0.
            /// - All other values are truncated and rounded to the nearest representable  8-bit value.
            #[inline]
            pub const fn from_f64(x: f64) -> Self {
                Self($crate::bit::float8::from_f64::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(x))
            }

            /// Converts this type into a [`prim@f64`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 64-bit value.
            #[inline]
            pub const fn to_f64(self) -> f64 {
                $crate::bit::float8::to_f64::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0)
            }

            /// Returns `true` if this value is NaN and `false` otherwise.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let nan = ", stringify!($name), "::NAN;")]
            #[doc = concat!("let f = ", stringify!($name), "::from_f32(7.0f32);")]
            ///
            /// assert!(nan.is_nan());
            /// assert!(!f.is_nan());
            /// ```
            #[inline]
            pub const fn is_nan(self) -> bool {
                $crate::bit::is_nan::<
                    { $format.frac },
                    { $format.exp },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0)
            }

            /// Returns `true` if this value is ±∞ and `false` otherwise.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let f = ", stringify!($name), "::from_f32(7.0f32);")]
            #[doc = concat!("let nan = ", stringify!($name), "::NAN;")]
            ///
            /// assert!(!f.is_infinite());
            /// assert!(!nan.is_infinite());
            /// ```
            #[inline]
            pub const fn is_infinite(self) -> bool {
                match $format.inf {
                    Some(inf) => self.0 & 0x7f == inf,
                    None => false,
                }
            }

            /// Returns true if this number is neither infinite nor NaN.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let f = ", stringify!($name), "::from_f32(7.0f32);")]
            #[doc = concat!("let nan = ", stringify!($name), "::NAN;")]
            ///
            /// assert!(f.is_finite());
            /// assert!(!nan.is_finite());
            /// ```
            #[inline]
            pub const fn is_finite(self) -> bool {
                !(self.is_infinite() || self.is_nan())
            }

            /// Returns `true` if and only if `self` has a positive sign, including +0.0, NaNs with a positive sign bit and +∞.
            #[inline]
            pub const fn is_sign_positive(self) -> bool {
                self.0 & 0x80 == 0
            }

            /// Returns `true` if and only if `self` has a negative sign, including −0.0, NaNs with a negative sign bit and −∞.
            #[inline]
            pub const fn is_sign_negative(self) -> bool {
                self.0 & 0x80 != 0
            }

            /// Returns `true` if the number is neither zero, infinite, subnormal, or `NaN` and `false` otherwise.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let min = ", stringify!($name), "::MIN_POSITIVE;")]
            #[doc = concat!("let max = ", stringify!($name), "::MAX;")]
            #[doc = concat!("let lower_than_min = ", stringify!($name), "::from_f32(1.0e-10);")]
            #[doc = concat!("let zero = ", stringify!($name), "::ZERO;")]
            ///
            /// assert!(min.is_normal());
            /// assert!(max.is_normal());
            ///
            /// assert!(!zero.is_normal());
            #[doc = concat!("assert!(!", stringify!($name), "::NAN.is_normal());")]
            /// // Values between `0` and `min` are Subnormal.
            /// assert!(!lower_than_min.is_normal());
            /// ```
            pub const fn is_normal(&self) -> bool {
                let exp = self.0 & const { ((1 << $format.exp) - 1) << $format.frac };
                exp != 0 && self.is_finite()
            }

            /// Returns a number that represents the sign of `self`.
            ///
            /// * `1.0` if the number is positive, `+0.0` or `INFINITY`
            /// * `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
            /// * [`NAN`][Self::NAN] if the number is `NaN`
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let f = ", stringify!($name), "::from_f32(3.5_f32);")]
            ///
            #[doc = concat!("assert_eq!(f.signum(), ", stringify!($name), "::from_f32(1.0));")]
            ///
            #[doc = concat!("assert!(", stringify!($name), "::NAN.signum().is_nan());")]
            /// ```
            #[inline]
            pub const fn signum(self) -> Self {
                if self.is_nan() {
                    Self::NAN
                } else if self.0 & 0x7f == 0 {
                    self
                } else if self.0 & 0x80 != 0 {
                    Self::NEG_ONE
                } else {
                    Self::ONE
                }
            }

            /// Computes the absolute value of `self`.
            ///
            /// This function always returns the precise result.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let x = ", stringify!($name), "::from_f32(3.5);")]
            #[doc = concat!("let y = ", stringify!($name), "::from_f32(-3.5);")]
            ///
            /// assert_eq!(x.abs(), x);
            /// assert_eq!(y.abs(), -y);
            ///
            /// assert!(f64::NAN.abs().is_nan());
            /// ```
            #[inline]
            pub const fn abs(self) -> Self {
                Self::from_bits(self.0 & 0x7f)
            }

            /// Returns a number composed of the magnitude of `self` and the sign of `sign`.
            ///
            /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise equal to `-self`.
            /// If `self` is NaN, then NaN with the sign of `sign` is returned.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let f = ", stringify!($name), "::from_f32(3.5);")]
            ///
            #[doc = concat!("assert_eq!(f.copysign(", stringify!($name), "::from_f32(0.42)), ", stringify!($name), "::from_f32(3.5));")]
            #[doc = concat!("assert_eq!(f.copysign(", stringify!($name), "::from_f32(-0.42)), ", stringify!($name), "::from_f32(-3.5));")]
            #[doc = concat!("assert_eq!((-f).copysign(", stringify!($name), "::from_f32(0.42)), ", stringify!($name), "::from_f32(3.5));")]
            #[doc = concat!("assert_eq!((-f).copysign(", stringify!($name), "::from_f32(-0.42)), ", stringify!($name), "::from_f32(-3.5));")]
            ///
            #[doc = concat!("assert!(", stringify!($name), "::NAN.copysign(", stringify!($name), "::from_f32(1.0)).is_nan());")]
            /// ```
            #[inline]
            pub const fn copysign(self, sign: Self) -> Self {
                if self.is_nan() {
                    Self::from_bits(Self::NAN.to_bits() | (sign.0 & 0x80))
                } else {
                    Self::from_bits((self.0 & 0x7f) | (sign.0 & 0x80))
                }
            }

            /// Returns the minimum of the two numbers.
            ///
            /// If one of the arguments is NaN, then the other argument is returned.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let x = ", stringify!($name), "::from_f32(1.0);")]
            #[doc = concat!("let y = ", stringify!($name), "::from_f32(2.0);")]
            ///
            /// assert_eq!(x.min(y), x);
            /// ```
            pub fn min(self, other: Self) -> Self {
                if other < self && !other.is_nan() {
                    other
                } else {
                    self
                }
            }

            /// Returns the maximum of the two numbers.
            ///
            /// If one of the arguments is NaN, then the other argument is returned.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let x = ", stringify!($name), "::from_f32(1.0);")]
            #[doc = concat!("let y = ", stringify!($name), "::from_f32(2.0);")]
            ///
            /// assert_eq!(x.max(y), y);
            /// ```
            pub fn max(self, other: Self) -> Self {
                if other > self && !other.is_nan() {
                    other
                } else {
                    self
                }
            }

            /// Restrict a value to a certain interval unless it is NaN.
            ///
            /// Returns `max` if `self` is greater than `max`, and `min` if `self` is less than `min`.
            /// Otherwise this returns `self`.
            ///
            /// Note that this function returns NaN if the initial value was NaN as well.
            ///
            /// # Panics
            /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("assert_eq!(", stringify!($name), "::from_f32(-3.0).clamp(", stringify!($name), "::from_f32(-2.0), ", stringify!($name), "::from_f32(1.0)),", stringify!($name), "::from_f32(-2.0));")]
            #[doc = concat!("assert_eq!(", stringify!($name), "::from_f32(0.0).clamp(", stringify!($name), "::from_f32(-2.0), ", stringify!($name), "::from_f32(1.0)),", stringify!($name), "::from_f32(0.0));")]
            #[doc = concat!("assert_eq!(", stringify!($name), "::from_f32(2.0).clamp(", stringify!($name), "::from_f32(-2.0), ", stringify!($name), "::from_f32(1.0)),", stringify!($name), "::from_f32(1.0));")]
            #[doc = concat!("assert!(", stringify!($name), "::NAN.clamp(", stringify!($name), "::from_f32(-2.0), ", stringify!($name), "::from_f32(1.0)).is_nan());")]
            /// ```
            pub fn clamp(self, min: Self, max: Self) -> Self {
                assert!(min <= max);
                let mut x = self;
                if x < min {
                    x = min;
                }
                if x > max {
                    x = max;
                }
                x
            }

            /// Returns the ordering between `self` and `other`.
            ///
            /// - negative quiet NaN
            /// - negative signaling NaN
            /// - negative infinity
            /// - negative numbers
            /// - negative subnormal numbers
            /// - negative zero
            /// - positive zero
            /// - positive subnormal numbers
            /// - positive numbers
            /// - positive infinity
            /// - positive signaling NaN
            /// - positive quiet NaN.
            ///
            /// The ordering established by this function does not always agree with the
            /// [`PartialOrd`] and [`PartialEq`] implementations. For example,
            /// they consider negative and positive zero equal, while `total_cmp`
            /// doesn't.
            ///
            /// # Example
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let mut v: Vec<", stringify!($name), "> = vec![];")]
            #[doc = concat!("v.push(", stringify!($name), "::ONE);")]
            #[doc = concat!("v.push(", stringify!($name), "::NAN);")]
            #[doc = concat!("v.push(", stringify!($name), "::MAX_SUBNORMAL);")]
            #[doc = concat!("v.push(-", stringify!($name), "::MAX_SUBNORMAL);")]
            #[doc = concat!("v.push(", stringify!($name), "::ZERO);")]
            #[doc = concat!("v.push(", stringify!($name), "::NEG_ONE);")]
            #[doc = concat!("v.push(", stringify!($name), "::MIN_POSITIVE);")]
            #[doc = ""]
            #[doc = "v.sort_by(|a, b| a.total_cmp(&b));"]
            #[doc = ""]
            #[doc = "assert!(v"]
            #[doc = "    .into_iter()"]
            #[doc = "    .zip("]
            #[doc = "        ["]
            #[doc = concat!("            ", stringify!($name), "::NEG_ONE,")]
            #[doc = concat!("            -", stringify!($name), "::MAX_SUBNORMAL,")]
            #[doc = concat!("            ", stringify!($name), "::ZERO,")]
            #[doc = concat!("            ", stringify!($name), "::MAX_SUBNORMAL,")]
            #[doc = concat!("            ", stringify!($name), "::MIN_POSITIVE,")]
            #[doc = concat!("            ", stringify!($name), "::ONE,")]
            #[doc = concat!("            ", stringify!($name), "::NAN")]
            #[doc = "        ]"]
            #[doc = "        .iter()"]
            #[doc = "    )"]
            #[doc = "    .all(|(a, b)| a.to_bits() == b.to_bits()));"]
            /// ```
            pub fn total_cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                match (self.is_nan(), other.is_nan()) {
                    (true, true) => ::core::cmp::Ordering::Equal,
                    (true, false) => ::core::cmp::Ordering::Greater,
                    (false, true) => ::core::cmp::Ordering::Less,
                    (false, false) => {
                        let mut left = self.to_bits() as i8;
                        let mut right = other.to_bits() as i8;
                        left ^= (((left >> 7) as u8) >> 1) as i8;
                        right ^= (((right >> 7) as u8) >> 1) as i8;
                        left.cmp(&right)
                    }
                }
            }

            /// `self + rhs`
            #[inline]
            pub const fn add_const(self, rhs: Self) -> Self {
                Self($crate::bit::float8::addsub::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                    false,
                >(self.0, rhs.0))
            }

            /// `self - rhs`
            #[inline]
            pub const fn sub_const(self, rhs: Self) -> Self {
                Self($crate::bit::float8::addsub::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                    true,
                >(self.0, rhs.0))
            }

            /// `self * rhs`
            #[inline]
            pub const fn mul_const(self, rhs: Self) -> Self {
                Self($crate::bit::float8::mul::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0, rhs.0))
            }

            /// `self / rhs`
            #[inline]
            pub const fn div_const(self, rhs: Self) -> Self {
                Self($crate::bit::float8::div::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0, rhs.0))
            }

            /// `-self`
            #[inline]
            pub const fn neg_const(self) -> Self {
                Self(self.0 ^ 0x80)
            }
        }

        impl From<f32> for $name {
            #[inline]
            fn from(val: f32) -> Self {
                Self::from_f32(val)
            }
        }

        impl From<f64> for $name {
            #[inline]
            fn from(val: f64) -> Self {
                Self::from_f64(val)
            }
        }

        impl From<$name> for f32 {
            #[inline]
            fn from(val: $name) -> Self {
                val.to_f32()
            }
        }

        impl From<$name> for f64 {
            #[inline]
            fn from(val: $name) -> Self {
                val.to_f64()
            }
        }

        impl ::core::ops::Add for $name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                self.add_const(rhs)
            }
        }

        impl ::core::ops::Sub for $name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                self.sub_const(rhs)
            }
        }

        impl ::core::ops::Mul for $name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self {
                self.mul_const(rhs)
            }
        }

        impl ::core::ops::Div for $name {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self {
                self.div_const(rhs)
            }
        }

        impl ::core::ops::Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                self.neg_const()
            }
        }
    };
}

use crate::float::format::{E4M3, E4M3FN, E4M3FNUZ, E5M2, E5M2FNUZ};

impl_float8!(hf8, E4M3);
impl_float8!(hf8fn, E4M3FN);
impl_float8!(hf8uz, E4M3FNUZ);
impl_float8!(bf8, E5M2);
impl_float8!(bf8uz, E5M2FNUZ);

impl hf8 {
    /// Positive infinity ∞
    pub const INFINITY: Self = Self(E4M3.inf.unwrap());
    /// Negative infinity -∞
    pub const NEG_INFINITY: Self = Self(E4M3.inf.unwrap()).neg_const();
    /// -0
    pub const NEG_ZERO: Self = Self(0x80);
}

impl hf8fn {
    /// -0
    pub const NEG_ZERO: Self = Self(0x80);
}

impl bf8 {
    /// Positive infinity ∞
    pub const INFINITY: Self = Self(E5M2.inf.unwrap());
    /// Negative infinity -∞
    pub const NEG_INFINITY: Self = Self(E5M2.inf.unwrap()).neg_const();
    /// -0
    pub const NEG_ZERO: Self = Self(0x80);
}

macro_rules! impl_constants {
    ($t:ty) => {
        impl $t {
            /// π
            pub const PI: Self = Self::from_f64(::core::f64::consts::PI);
            /// The full circle constant (τ)
            ///
            /// Equal to 2π.
            pub const TAU: Self = Self::from_f64(::core::f64::consts::TAU);
            /// π/2
            pub const FRAC_PI_2: Self = Self::from_f64(::core::f64::consts::FRAC_PI_2);
            /// π/3
            pub const FRAC_PI_3: Self = Self::from_f64(::core::f64::consts::FRAC_PI_3);
            /// π/4
            pub const FRAC_PI_4: Self = Self::from_f64(::core::f64::consts::FRAC_PI_4);
            /// π/6
            pub const FRAC_PI_6: Self = Self::from_f64(::core::f64::consts::FRAC_PI_6);
            /// π/8
            pub const FRAC_PI_8: Self = Self::from_f64(::core::f64::consts::FRAC_PI_8);
            /// 1/π
            pub const FRAC_1_PI: Self = Self::from_f64(::core::f64::consts::FRAC_1_PI);
            /// 2/π
            pub const FRAC_2_PI: Self = Self::from_f64(::core::f64::consts::FRAC_2_PI);
            /// 2/sqrt(π)
            pub const FRAC_2_SQRT_PI: Self = Self::from_f64(::core::f64::consts::FRAC_2_SQRT_PI);
            /// sqrt(2)
            pub const SQRT_2: Self = Self::from_f64(::core::f64::consts::SQRT_2);
            /// 1/sqrt(2)
            pub const FRAC_1_SQRT_2: Self = Self::from_f64(::core::f64::consts::FRAC_1_SQRT_2);
            /// Euler's number (e)
            pub const E: Self = Self::from_f64(::core::f64::consts::E);
            /// log<sub>2</sub>(10)
            pub const LOG2_10: Self = Self::from_f64(::core::f64::consts::LOG2_10);
            /// log<sub>2</sub>(e)
            pub const LOG2_E: Self = Self::from_f64(::core::f64::consts::LOG2_E);
            /// log<sub>10</sub>(2)
            pub const LOG10_2: Self = Self::from_f64(::core::f64::consts::LOG10_2);
            /// log<sub>10</sub>(e)
            pub const LOG10_E: Self = Self::from_f64(::core::f64::consts::LOG10_E);
            /// ln(2)
            pub const LN_2: Self = Self::from_f64(::core::f64::consts::LN_2);
            /// ln(10)
            pub const LN_10: Self = Self::from_f64(::core::f64::consts::LN_10);
        }
    };
}

impl_constants!(hf8);
impl_constants!(hf8fn);
impl_constants!(hf8uz);
impl_constants!(bf8);
impl_constants!(bf8uz);

macro_rules! impl_io {
    ($t:ident) => {
        impl ::core::fmt::Display for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::Debug for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::str::FromStr for $t {
            type Err = ::core::num::ParseFloatError;
            fn from_str(src: &str) -> Result<Self, Self::Err> {
                f32::from_str(src).map($t::from_f32)
            }
        }
        impl ::core::fmt::LowerExp for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerExp::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::LowerHex for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.0, f)
            }
        }
        impl ::core::fmt::UpperExp for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperExp::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::UpperHex for $t {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.0, f)
            }
        }
    };
}

impl_io!(hf8);
impl_io!(hf8fn);
impl_io!(hf8uz);
impl_io!(bf8);
impl_io!(bf8uz);

#[cfg(feature = "half")]
macro_rules! impl_half_cvt {
    ($t:ident $format:ident) => {
        impl $t {
            /// Convert a [`struct@f16`] type into [`Self`].
            ///
            /// This operation is lossy.
            ///
            /// - If the 16-bit value is to large to fit in 8-bits, ±∞ will result.
            /// - NaN values are preserved.
            /// - 16-bit subnormal values are too tiny to be represented in 8-bits and result in ±0.
            /// - Exponents that underflow the minimum 8-bit exponent will result in 8-bit subnormals or ±0.
            /// - All other values are truncated and rounded to the nearest representable  8-bit value.
            pub const fn from_f16(x: ::half::f16) -> Self {
                Self($crate::bit::half::f8_from_f16::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(x))
            }

            /// Convert a [`struct@bf16`] type into [`Self`].
            ///
            /// This operation is lossy.
            ///
            /// - If the 16-bit value is to large to fit in 8-bits, ±∞ will result.
            /// - NaN values are preserved.
            /// - 16-bit subnormal values are too tiny to be represented in 8-bits and result in ±0.
            /// - Exponents that underflow the minimum 8-bit exponent will result in 8-bit subnormals or ±0.
            /// - All other values are truncated and rounded to the nearest representable  8-bit value.
            pub const fn from_bf16(x: ::half::bf16) -> Self {
                Self($crate::bit::half::f8_from_bf16::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(x))
            }

            /// Converts this type into a [`struct@f16`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 16-bit value.
            pub const fn to_f16(self) -> ::half::f16 {
                crate::bit::half::f8_to_f16::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0)
            }

            /// Converts this type into a [`struct@bf16`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 16-bit value.
            pub const fn to_bf16(self) -> ::half::bf16 {
                crate::bit::half::f8_to_bf16::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.nan },
                    {
                        match $format.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.0)
            }
        }

        impl From<::half::f16> for $t {
            fn from(x: ::half::f16) -> Self {
                Self::from_f16(x)
            }
        }
        impl From<::half::bf16> for $t {
            fn from(x: ::half::bf16) -> Self {
                Self::from_bf16(x)
            }
        }
        impl From<$t> for ::half::f16 {
            fn from(x: $t) -> Self {
                x.to_f16()
            }
        }
        impl From<$t> for ::half::bf16 {
            fn from(x: $t) -> Self {
                x.to_bf16()
            }
        }
    };
}

#[cfg(feature = "half")]
impl_half_cvt!(hf8 E4M3);
#[cfg(feature = "half")]
impl_half_cvt!(hf8fn E4M3FN);
#[cfg(feature = "half")]
impl_half_cvt!(hf8uz E4M3FNUZ);
#[cfg(feature = "half")]
impl_half_cvt!(bf8 E5M2);
#[cfg(feature = "half")]
impl_half_cvt!(bf8uz E5M2FNUZ);
