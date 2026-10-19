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
        unsafe impl Pod for $name {}

        impl $name {
            pub const NAN: Self = Self::from_bits($format.nan);
            pub const MAX: Self = Self::from_bits($format.max);
            pub const MIN: Self = Self::from_bits($format.max | 0x80);
            pub const MIN_POSITIVE: Self = Self::from_bits(1 << $format.frac);
            pub const MIN_EXPONENT: i32 = 1 - ($format.bias as i32);
            pub const MAX_EXPONENT: i32 = (((1i32 << $format.exp) - 2) - ($format.bias as i32));
            pub const ZERO: Self = Self::from_bits(0);
            pub const NEG_ZERO: Self = Self::from_bits(0x80);
            pub const ONE: Self = Self::from_bits((1 << $format.frac) + $format.bias as u8);
            pub const NEG_ONE: Self =
                Self::from_bits(0x80 | (($format.bias as u8) << $format.frac));

            #[inline(always)]
            pub const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }

            #[inline]
            pub const fn to_bits(self) -> u8 {
                self.0
            }

            /// Returns `true` if this value is NaN and `false` otherwise.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            ///
            #[doc = concat!("let nan = ", stringify!($name), "::NAN;")]
            #[doc = concat!("let f = ", stringify!($name), "::from_f32(7.0f32);")]
            ///
            /// assert!(nan.is_nan());
            /// assert!(!f.is_nan());
            /// ```
            #[inline]
            pub const fn is_nan(self) -> bool {
                #[allow(clippy::bad_bit_mask)]
                if const { $format.inf.is_some() } {
                    self.0 & 0x7f == $format.nan
                } else if const { $format.nan == 0x7f } {
                    self.0 == 0x7f || self.0 == 0xff
                } else {
                    self.0 == $format.nan
                }
            }

            /// Returns `true` if this value is ±∞ and `false` otherwise.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            ///
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
            ///
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

            #[inline]
            pub const fn is_sign_positive(self) -> bool {
                self.0 & 0x80u8 == 0
            }

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

            #[inline]
            pub const fn abs(self) -> Self {
                Self::from_bits(self.0 & 0x7f)
            }

            #[inline]
            pub const fn is_sign_negative(self) -> bool {
                self.0 & 0x80 != 0
            }

            #[inline]
            pub const fn copysign(self, sign: Self) -> Self {
                Self::from_bits((self.0 & 0x7f) | (sign.0 & 0x80))
            }

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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                >(x))
            }

            /// Converts this type into a [`f32`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 16-bit value.
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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                >(self.0)
            }

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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                >(x))
            }

            /// Converts this type into a [`f64`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 16-bit value.
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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                >(self.0)
            }

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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                    false,
                >(self.0, rhs.0))
            }

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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                    true,
                >(self.0, rhs.0))
            }

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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                >(self.0, rhs.0))
            }

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
                            None => $crate::bit::float8::NO_INF,
                        }
                    },
                >(self.0, rhs.0))
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
    };
}

use crate::float::format::{E4M3, E4M3FN, E4M3FNUZ, E5M2, E5M2FNUZ};

impl_float8!(hf8, E4M3);
impl_float8!(hf8fn, E4M3FN);
impl_float8!(hf8uz, E4M3FNUZ);
impl_float8!(bf8, E5M2);
impl_float8!(bf8uz, E5M2FNUZ);
