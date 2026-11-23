macro_rules! impl_constants {
    ($name:ty) => {
        impl $name {
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

impl_constants!(f4);
impl_constants!(hf6);
impl_constants!(bf6);
impl_constants!(hf8);
impl_constants!(hf8fn);
impl_constants!(hf8uz);
impl_constants!(bf8);
impl_constants!(bf8uz);

macro_rules! impl_io {
    ($name:ident) => {
        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::LowerExp for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerExp::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::UpperExp for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperExp::fmt(&self.to_f32(), f)
            }
        }
        impl ::core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::LowerHex::fmt(&self.to_bits(), f)
            }
        }
        impl ::core::fmt::UpperHex for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::UpperHex::fmt(&self.to_bits(), f)
            }
        }
        impl ::core::str::FromStr for $name {
            type Err = ::core::num::ParseFloatError;
            fn from_str(src: &str) -> Result<Self, Self::Err> {
                f32::from_str(src).map($name::from_f32)
            }
        }
    };
}

#[cfg(feature = "half")]
macro_rules! impl_half_cvt {
    ($name:ident) => {
        impl $name {
            /// Convert a [`struct@f16`] type into [`Self`].
            ///
            /// This operation is lossy.
            ///
            /// - If the 16-bit value is too large to fit in 8-bits, ±∞ will result.
            /// - NaN values are preserved.
            /// - 16-bit subnormal values are too tiny to be represented in 8-bits and result in ±0.
            /// - Exponents that underflow the minimum 8-bit exponent will result in 8-bit subnormals or ±0.
            /// - All other values are truncated and rounded to the nearest representable  8-bit value.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*; use half::*;
            #[doc = concat!("let a = ", stringify!($name), "::from_f16(f16::from_f32(1.5));")]
            /// let b = a.to_f16();
            /// assert_eq!(b, f16::from_f32(1.5));
            /// ```
            pub const fn from_f16(x: ::half::f16) -> Self {
                Self::from_bits($crate::bit::half::f8_from_f16::<
                    { Self::FORMAT.frac },
                    { Self::FORMAT.exp },
                    { Self::FORMAT.bias },
                    { Self::FORMAT.width },
                    { Self::FORMAT.max },
                    { Self::FORMAT.nan },
                    {
                        match Self::FORMAT.inf {
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
            /// - If the 16-bit value is too large to fit in 8-bits, ±∞ will result.
            /// - NaN values are preserved.
            /// - 16-bit subnormal values are too tiny to be represented in 8-bits and result in ±0.
            /// - Exponents that underflow the minimum 8-bit exponent will result in 8-bit subnormals or ±0.
            /// - All other values are truncated and rounded to the nearest representable  8-bit value.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*; use half::*;
            #[doc = concat!("let a = ", stringify!($name), "::from_bf16(bf16::from_f32(1.5));")]
            /// let b = a.to_bf16();
            /// assert_eq!(b, bf16::from_f32(1.5));
            /// ```
            pub const fn from_bf16(x: ::half::bf16) -> Self {
                Self::from_bits($crate::bit::half::f8_from_bf16::<
                    { Self::FORMAT.frac },
                    { Self::FORMAT.exp },
                    { Self::FORMAT.bias },
                    { Self::FORMAT.width },
                    { Self::FORMAT.max },
                    { Self::FORMAT.nan },
                    {
                        match Self::FORMAT.inf {
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
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*; use half::*;
            #[doc = concat!("let a = ", stringify!($name), "::from_f16(f16::from_f32(1.5));")]
            /// let b = a.to_f16();
            /// assert_eq!(b, f16::from_f32(1.5));
            /// ```
            pub const fn to_f16(self) -> ::half::f16 {
                crate::bit::half::f8_to_f16::<
                    { Self::FORMAT.frac },
                    { Self::FORMAT.exp },
                    { Self::FORMAT.bias },
                    { Self::FORMAT.width },
                    { Self::FORMAT.nan },
                    {
                        match Self::FORMAT.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.to_bits())
            }

            /// Converts this type into a [`struct@bf16`] type.
            ///
            /// This operation may be lossy.
            ///
            /// - NaN and zero values are preserved.
            /// - Subnormal values are normalized.
            /// - Otherwise, the values are mapped to the appropriate 16-bit value.
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*; use half::*;
            #[doc = concat!("let a = ", stringify!($name), "::from_bf16(bf16::from_f32(1.5));")]
            /// let b = a.to_bf16();
            /// assert_eq!(b, bf16::from_f32(1.5));
            /// ```
            pub const fn to_bf16(self) -> ::half::bf16 {
                crate::bit::half::f8_to_bf16::<
                    { Self::FORMAT.frac },
                    { Self::FORMAT.exp },
                    { Self::FORMAT.bias },
                    { Self::FORMAT.width },
                    { Self::FORMAT.nan },
                    {
                        match Self::FORMAT.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(self.to_bits())
            }
        }

        impl From<::half::f16> for $name {
            fn from(x: ::half::f16) -> Self {
                Self::from_f16(x)
            }
        }
        impl From<::half::bf16> for $name {
            fn from(x: ::half::bf16) -> Self {
                Self::from_bf16(x)
            }
        }
        impl From<$name> for ::half::f16 {
            fn from(x: $name) -> Self {
                x.to_f16()
            }
        }
        impl From<$name> for ::half::bf16 {
            fn from(x: $name) -> Self {
                x.to_bf16()
            }
        }
    };
}

#[cfg(feature = "half")]
impl_half_cvt!(f4);
#[cfg(feature = "half")]
impl_half_cvt!(hf6);
#[cfg(feature = "half")]
impl_half_cvt!(bf6);
#[cfg(feature = "half")]
impl_half_cvt!(hf8);
#[cfg(feature = "half")]
impl_half_cvt!(hf8fn);
#[cfg(feature = "half")]
impl_half_cvt!(hf8uz);
#[cfg(feature = "half")]
impl_half_cvt!(bf8);
#[cfg(feature = "half")]
impl_half_cvt!(bf8uz);

macro_rules! impl_cvt {
    ($($ty:ident -> $from:ident $to:ident -> $out_ty:ident)*) => {$(
        impl $ty {
            #[doc = concat!("Converts to ", stringify!($out_ty), ".")]
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let a = ", stringify!($ty), "::from_f32(2.0);")]
            #[doc = concat!("let b = ", stringify!($out_ty), "::from_f32(2.0);")]
            #[doc = concat!("assert_eq!(a.", stringify!($to), "(), b);")]
            /// ```
            #[inline]
            pub const fn $to(self) -> $out_ty {
                $out_ty::from_bits($crate::bit::cvt::float_to_float::<
                    { Self::FORMAT.frac },
                    { Self::FORMAT.exp },
                    { Self::FORMAT.bias },
                    { Self::FORMAT.width },
                    { Self::FORMAT.max },
                    { Self::FORMAT.nan },
                    {
                        match Self::FORMAT.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },

                    { $out_ty::FORMAT.frac },
                    { $out_ty::FORMAT.exp },
                    { $out_ty::FORMAT.bias },
                    { $out_ty::FORMAT.width },
                    { $out_ty::FORMAT.max },
                    { $out_ty::FORMAT.nan },
                    {
                        match $out_ty::FORMAT.inf {
                            Some(v) => v,
                            None => $crate::bit::NO_INF,
                        }
                    },
                >(
                    self.to_bits()
                ))
            }
        }
        impl $out_ty {
            /// Converts
            ///
            /// # Examples
            ///
            /// ```rust
            /// # use lowp::*;
            #[doc = concat!("let a = ", stringify!($ty), "::from_f32(2.0);")]
            #[doc = concat!("let b = ", stringify!($out_ty), "::from_", stringify!($ty), "(a);")]
            #[doc = concat!("assert_eq!(a.to_", stringify!($out_ty), "(), b);")]
            /// ```
            #[inline]
            pub const fn $from(val: $ty) -> Self {
                val.$to()
            }

        }
        impl From<$ty> for $out_ty {
            #[inline]
            fn from(val: $ty) -> Self {
                val.$to()
            }
        }
    )*};
}

impl_cvt!(
    hf8 -> from_hf8 to_f4 -> f4
    hf8 -> from_hf8 to_hf6 -> hf6
    hf8 -> from_hf8 to_bf6 -> bf6
    hf8 -> from_hf8 to_hf8fn -> hf8fn
    hf8 -> from_hf8 to_hf8uz -> hf8uz
    hf8 -> from_hf8 to_bf8 -> bf8
    hf8 -> from_hf8 to_bf8uz -> bf8uz

    hf8fn -> from_hf8fn to_f4 -> f4
    hf8fn -> from_hf8fn to_hf6 -> hf6
    hf8fn -> from_hf8fn to_bf6 -> bf6
    hf8fn -> from_hf8fn to_hf8 -> hf8
    hf8fn -> from_hf8fn to_hf8uz -> hf8uz
    hf8fn -> from_hf8fn to_bf8 -> bf8
    hf8fn -> from_hf8fn to_bf8uz -> bf8uz

    hf8uz -> from_hf8uz to_f4 -> f4
    hf8uz -> from_hf8uz to_hf6 -> hf6
    hf8uz -> from_hf8uz to_bf6 -> bf6
    hf8uz -> from_hf8uz to_hf8 -> hf8
    hf8uz -> from_hf8uz to_hf8fn -> hf8fn
    hf8uz -> from_hf8uz to_bf8 -> bf8
    hf8uz -> from_hf8uz to_bf8uz -> bf8uz

    bf8 -> from_bf8 to_f4 -> f4
    bf8 -> from_bf8 to_hf6 -> hf6
    bf8 -> from_bf8 to_bf6 -> bf6
    bf8 -> from_bf8 to_hf8 -> hf8
    bf8 -> from_bf8 to_hf8fn -> hf8fn
    bf8 -> from_bf8 to_hf8uz -> hf8uz
    bf8 -> from_bf8 to_bf8uz -> bf8uz

    bf8uz -> from_bf8uz to_f4 -> f4
    bf8uz -> from_bf8uz to_hf6 -> hf6
    bf8uz -> from_bf8uz to_bf6 -> bf6
    bf8uz -> from_bf8uz to_hf8 -> hf8
    bf8uz -> from_bf8uz to_hf8fn -> hf8fn
    bf8uz -> from_bf8uz to_bf8 -> bf8
    bf8uz -> from_bf8uz to_hf8uz -> hf8uz

    f4 -> from_f4 to_hf6 -> hf6
    f4 -> from_f4 to_bf6 -> bf6
    f4 -> from_f4 to_hf8 -> hf8
    f4 -> from_f4 to_hf8fn -> hf8fn
    f4 -> from_f4 to_hf8uz -> hf8uz
    f4 -> from_f4 to_bf8 -> bf8
    f4 -> from_f4 to_bf8uz -> bf8uz

    bf6 -> from_bf6 to_f4 -> f4
    bf6 -> from_bf6 to_hf6 -> hf6
    bf6 -> from_bf6 to_hf8 -> hf8
    bf6 -> from_bf6 to_hf8fn -> hf8fn
    bf6 -> from_bf6 to_hf8uz -> hf8uz
    bf6 -> from_bf6 to_bf8 -> bf8
    bf6 -> from_bf6 to_bf8uz -> bf8uz

    hf6 -> from_hf6 to_f4 -> f4
    hf6 -> from_hf6 to_bf6 -> bf6
    hf6 -> from_hf6 to_hf8 -> hf8
    hf6 -> from_hf6 to_hf8fn -> hf8fn
    hf6 -> from_hf6 to_hf8uz -> hf8uz
    hf6 -> from_hf6 to_bf8 -> bf8
    hf6 -> from_hf6 to_bf8uz -> bf8uz
);

pub(crate) struct FloatFormat {
    pub width: u8,
    pub exp: u8,
    pub frac: u8,
    pub bias: i8,
    pub max: u8,
    pub nan: u8,
    pub inf: Option<u8>,
}

mod float8;
pub use float8::{bf8, bf8uz, hf8, hf8fn, hf8uz};

mod float6;
pub use float6::{bf6, bf6x4, hf6, hf6x4};

mod float4;
pub use float4::{f4, f4x2};

#[cfg(feature = "num-traits")]
mod num_traits;
