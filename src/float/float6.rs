use core::{
    cmp::Ordering,
    hint::{assert_unchecked, unreachable_unchecked},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::{
    bit::float6::{ValidIndex, Vec6x4Idx},
    float::FloatFormat,
};

const E2M3: FloatFormat = FloatFormat {
    width: 6,
    exp: 2,
    frac: 3,
    bias: 1,
    max: 0x1f,
    nan: 0x1f,
    inf: None,
};

const E3M2: FloatFormat = FloatFormat {
    width: 6,
    exp: 3,
    frac: 2,
    bias: 3,
    max: 0x1f,
    nan: 0x1f,
    inf: None,
};

macro_rules! impl_float6 {
    ($name:ident $vec:ident $format:ident) => {
        #[doc = concat!("Four contiguous 6-bit floating point values with the ", stringify!($format), " format")]
        #[derive(Clone, Copy, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
        #[cfg_attr(feature = "zerocopy", derive(zerocopy::AsBytes, zerocopy::FromBytes))]
        #[cfg_attr(feature = "stable_auto", derive(stable_auto::Auto))]
        #[repr(transparent)]
        pub struct $vec([u8; 3]);

        impl $vec {
            #[doc = concat!("Packs 4 [`", stringify!($name), "`] values into a [`", stringify!($vec), "`].")]
            #[inline(always)]
            pub const fn new(a: $name, b: $name, c: $name, d: $name) -> Self {
                let bits = (a.to_bits() as u32)
                    | ((b.to_bits() as u32) << 6)
                    | ((c.to_bits() as u32) << 12)
                    | ((d.to_bits() as u32) << 18);

                Self([
                    bits as u8,
                    (bits >> 8) as u8,
                    (bits >> 16) as u8,
                ])
            }

            /// Constructs a packed vector from an array of 6-bit floats.
            #[inline(always)]
            pub const fn from_array(values: [$name; 4]) -> Self {
                Self::new(values[0], values[1], values[2], values[3])
            }

            #[doc = concat!("Gets the associated 6-bit packed [`", stringify!($name), "`] at index `IDX`.")]
            #[inline(always)]
            pub const fn get<const IDX: u8>(&self) -> $name
            where
                Vec6x4Idx<IDX>: ValidIndex,
            {
                match IDX {
                    0 => $name(self.0[0] & 0x3f),
                    1 => $name(
                        ((self.0[0] >> 6) | (self.0[1] << 2)) & 0x3f
                    ),
                    2 => $name(
                        ((self.0[1] >> 4) | (self.0[2] << 4)) & 0x3f
                    ),
                    3 => $name(self.0[2] >> 2),
                    _ => unsafe { unreachable_unchecked() },
                }
            }

            #[doc = concat!("Packs individual [`", stringify!($name), "`] values into [`", stringify!($vec), "`] groups.")]
            ///
            /// If `values.len()` is not divisible by four, the final group is padded with
            #[doc = concat!("[`", stringify!($name), "::ZERO`].")]
            #[inline]
            #[cfg(feature = "alloc")]
            pub fn pack(values: &[$name]) -> Box<[Self]> {
                values
                    .chunks(4)
                    .map(|chunk| {
                        let a = chunk[0];
                        let b = if chunk.len() > 1 { chunk[1] } else { $name::ZERO };
                        let c = if chunk.len() > 2 { chunk[2] } else { $name::ZERO };
                        let d = if chunk.len() > 3 { chunk[3] } else { $name::ZERO };
                        bf6x4::new(a, b, c, d)
                    })
                    .collect()
            }

            #[doc = concat!("Unpacks [`", stringify!($vec), "`] groups into individual [`", stringify!($name), "`] values.")]
            #[inline]
            #[cfg(feature = "alloc")]
            pub fn unpack(packed: &[Self]) -> Box<[$name]> {
                let mut result = Box::new_uninit_slice(packed.len() * 4);
                for (pos, &group) in packed.iter().enumerate() {
                    result[pos * 4].write(group.get::<0>());
                    result[pos * 4 + 1].write(group.get::<1>());
                    result[pos * 4 + 2].write(group.get::<2>());
                    result[pos * 4 + 3].write(group.get::<3>());
                }
                unsafe { result.assume_init() }
            }
        }

        #[doc = concat!("6-bit floating point value with the ", stringify!($format), " format")]
        #[derive(Clone, Copy, Default, Eq)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize))]
        #[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
        #[cfg_attr(feature = "zerocopy", derive(zerocopy::AsBytes, zerocopy::FromBytes))]
        #[cfg_attr(feature = "stable_auto", stable_auto::Auto)]
        #[repr(transparent)]
        pub struct $name(u8);

        impl $name {
            pub(super) const FORMAT: FloatFormat = $format;

            /// Construct an 6-bit floating point value from the low bits of the byte.
            pub const fn from_bits(bits: u8) -> Self {
                Self(bits & 0x3f)
            }

            /// Construct an 6-bit floating point value from the high bits of the byte.
            pub const fn from_hi_bits(bits: u8) -> Self {
                Self(bits >> 2)
            }

            /// Construct an 4-bit floating point value from the raw bits.
            ///
            /// # Safety
            ///
            /// Undefined behavior if any of the high 4 bits of `bits` are set
            pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
                unsafe {
                    assert_unchecked(bits & 0xc0 == 0);
                }

                Self(bits)
            }

            /// Return the raw bits.
            pub const fn to_bits(self) -> u8 {
                self.0
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                if (self.0 & 0x1f) == 0 && (other.0 & 0x1f) == 0 {
                    true
                } else {
                    self.0 == other.0
                }
            }
        }

        impl PartialOrd for $name {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $name {
            fn cmp(&self, other: &Self) -> Ordering {
                #[inline]
                const fn cmp_key(x: u8) -> u8 {
                    if x & 0x20 != 0 {
                        (!x) & 0x3f
                    } else {
                        x | 0x20
                    }
                }

                cmp_key(self.0).cmp(&cmp_key(other.0))
            }
        }
    };
}

impl_float6!(hf6 hf6x4 E2M3);
impl_float6!(bf6 bf6x4 E3M2);
