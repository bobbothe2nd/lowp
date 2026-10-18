mod format;

macro_rules! impl_float8 {
    ($name:ident, $format:ident$(,)?) => {
        #[doc = concat!("8-bit floating point value with the ", stringify!($format), " format")]
        #[derive(Clone, Copy, Default)]
        pub struct $name(u8);

        impl $name {
            pub const NAN: Self = Self::from_bits($format.nan);
            pub const MAX: Self = Self::from_bits($format.max);

            #[inline(always)]
            pub const fn from_bits(bits: u8) -> Self {
                Self(bits)
            }

            #[inline(always)]
            pub const fn to_bits(self) -> u8 {
                self.0
            }

            #[inline(always)]
            pub const fn from_f32(x: f32) -> Self {
                Self($crate::bit::float8::from_f32::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    { match $format.inf {
                        Some(v) => v,
                        None => $crate::bit::float8::NO_INF,
                    } },
                >(x))
            }

            #[inline(always)]
            pub const fn to_f32(self) -> f32 {
                $crate::bit::float8::to_f32::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.nan },
                    { match $format.inf {
                        Some(v) => v,
                        None => $crate::bit::float8::NO_INF,
                    } },
                >(self.0)
            }

            #[inline(always)]
            pub const fn from_f64(x: f64) -> Self {
                Self($crate::bit::float8::from_f64::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    { match $format.inf {
                        Some(v) => v,
                        None => $crate::bit::float8::NO_INF,
                    } },
                >(x))
            }

            #[inline(always)]
            pub const fn to_f64(self) -> f64 {
                $crate::bit::float8::to_f64::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.nan },
                    { match $format.inf {
                        Some(v) => v,
                        None => $crate::bit::float8::NO_INF,
                    } },
                >(self.0)
            }

            pub const fn add_const(self, rhs: Self) -> Self {
                Self($crate::bit::float8::addsub::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    { match $format.inf {
                        Some(v) => v,
                        None => $crate::bit::float8::NO_INF,
                    } },
                    false,
                >(self.0, rhs.0))
            }

            pub const fn sub_const(self, rhs: Self) -> Self {
                Self($crate::bit::float8::addsub::<
                    { $format.frac },
                    { $format.exp },
                    { $format.bias },
                    { $format.max },
                    { $format.nan },
                    { match $format.inf {
                        Some(v) => v,
                        None => $crate::bit::float8::NO_INF,
                    } },
                    true,
                >(self.0, rhs.0))
            }
        }

        impl From<f32> for $name {
            fn from(val: f32) -> Self {
                Self::from_f32(val)
            }
        }

        impl From<f64> for $name {
            fn from(val: f64) -> Self {
                Self::from_f64(val)
            }
        }

        impl From<$name> for f32 {
            fn from(val: $name) -> Self {
                val.to_f32()
            }
        }

        impl From<$name> for f64 {
            fn from(val: $name) -> Self {
                val.to_f64()
            }
        }

        impl ::core::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                self.add_const(rhs)
            }
        }

        impl ::core::ops::Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                self.sub_const(rhs)
            }
        }
    };
}

use crate::float::format::{E4M3, E4M3FN, E4M3FNUZ, E5M2, E5M2FN, E5M2FNUZ};

impl_float8!(hf8, E4M3);
impl_float8!(hf8fn, E4M3FN);
impl_float8!(hf8uz, E4M3FNUZ);
impl_float8!(bf8, E5M2);
impl_float8!(bf8fn, E5M2FN);
impl_float8!(bf8uz, E5M2FNUZ);
