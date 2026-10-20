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
                            None => $crate::bit::float8::NO_INF,
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
                            None => $crate::bit::float8::NO_INF,
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
                            None => $crate::bit::float8::NO_INF,
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
                            None => $crate::bit::float8::NO_INF,
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

mod float8;
pub use float8::*;

mod float4;
pub use float4::*;
