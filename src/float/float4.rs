use core::{hint::assert_unchecked, ops::{Add, Div, Mul, Neg, Rem, Sub}};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::bit::float4;

#[cfg(feature = "half")]
use half::{bf16, f16};

/// Two 4-bit E2M1 floats packed into 1 byte
///
/// Use this when you need contiguous storage or want to optimize 2 floats into 1 operation.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable, bytemuck::Pod))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::AsBytes, zerocopy::FromBytes))]
#[cfg_attr(feature = "auto", stable_auto::auto_opaque)]
pub struct f4x2(u8);

/// Packs individual [`f4x2`] values into [`f4`] pairs.
///
/// If `values.len()` is odd, the last element is paired with
/// [`f4x2::ZERO`] as padding.
///
/// # Examples
///
/// ```
/// use lowp::{f4x2, f4, pack, unpack};
///
/// let values = vec![
///     f4x2::from_f64(1.5),
///     f4x2::from_f64(-2.0),
///     f4x2::from_f64(3.0),
/// ];
/// let packed = pack(&values);
/// assert_eq!(packed.len(), 2); // 3 values → 2 bytes (last padded)
/// assert_eq!(packed[0].lo().to_f64(), 1.5);
/// assert_eq!(packed[0].hi().to_f64(), -2.0);
/// assert_eq!(packed[1].lo().to_f64(), 3.0);
/// assert_eq!(packed[1].hi().to_f64(), 0.0); // zero-padded
/// ```
#[cfg(feature = "alloc")]
pub fn pack(values: &[f4]) -> Box<[f4x2]> {
    values
        .chunks(2)
        .map(|chunk| {
            let lo = chunk[0];
            let hi = if chunk.len() == 2 {
                chunk[1]
            } else {
                f4::ZERO
            };
            f4x2::new(lo, hi)
        })
        .collect()
}

/// Unpacks [`f4`] pairs into individual [`f4x2`] values.
///
/// Each packed byte produces exactly two values. To recover the original
/// count when the input had an odd length, the caller should truncate the
/// result as needed.
///
/// # Examples
///
/// ```
/// use lowp::{f4x2, f4, unpack};
///
/// let packed = vec![
///     f4::new(f4x2::from_f64(1.0), f4x2::from_f64(2.0)),
/// ];
/// let values = unpack(&packed);
/// assert_eq!(values.len(), 2);
/// assert_eq!(values[0].to_f64(), 1.0);
/// assert_eq!(values[1].to_f64(), 2.0);
/// ```
#[cfg(feature = "alloc")]
pub fn unpack(packed: &[f4x2]) -> Box<[f4]> {
    let mut result = Box::new_uninit_slice(packed.len() * 2);
    let mut pos = 0;
    for &pair in packed {
        result[pos].write(pair.lo());
        result[pos +1].write(pair.hi());
        pos += 1;
    }
    unsafe { result.assume_init() }
}

#[cfg(feature = "briny")]
unsafe impl briny::traits::Pod for f4x2 {}

impl f4x2 {
    /// Construct an 8-bit floating point value from the raw bits.
    pub fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    /// Return the raw bits.
    pub const fn to_bits(self) -> u8 {
        self.0
    }

    /// Return the raw bits.
    pub const fn new(lo: f4, hi: f4) -> Self {
        Self(lo.0 | (hi.0 << 4))
    }

    /// Return both the low and high 4-bit floats
    pub const fn lo_hi(self) -> (f4, f4) {
        (self.lo(), self.hi())
    }

    /// Return the low 4-bit float
    pub const fn lo(self) -> f4 {
        f4(self.0 & 0xf)
    }

    /// Return the high 4-bit float
    pub const fn hi(self) -> f4 {
        f4(self.0 >> 4)
    }
}

/// 4-bit E2M1 float with 4 padding bits
///
/// This format can represent 16 distinct values ranging from -6.0 to 6.0, including:
/// - Normal numbers: ±1.0, ±1.5, ±2.0, ±3.0, ±4.0, ±6.0
/// - Subnormal numbers: ±0.5
/// - Zero: ±0.0
#[derive(Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::Zeroable))]
#[cfg_attr(feature = "zerocopy", derive(zerocopy::AsBytes))]
#[cfg_attr(feature = "auto", stable_auto::auto_opaque)]
pub struct f4(u8);

#[cfg(feature = "briny")]
unsafe impl briny::traits::Layout<u8> for f4 {}

impl f4 {
    /// Maximum possible value
    pub const MAX: Self = Self::from_bits(0b0111);
    /// Minimum possible value
    pub const MIN: Self = Self::from_bits(0b1111);
    /// Smallest possible positive value
    pub const MIN_POSITIVE: Self = Self::from_bits(0b0001);
    /// Largest possible subnormal value
    pub const MAX_SUBNORMAL: Self = Self::from_bits((1 << 1) - 1);
    /// Smallest possible subnormal value
    pub const MIN_POSITIVE_SUBNORMAL: Self = Self::from_bits(1);
    /// One greater than the minimum possible normal power of 2 exponent
    pub const MIN_EXP: i32 = 2 - (1 as i32);
    /// Minimum possible normal power of 2 exponent
    pub const MAX_EXP: i32 = (((1i32 << 2) - 2) - 1);
    /// 0
    pub const ZERO: Self = Self::from_bits(0);
    /// -0
    pub const NEG_ZERO: Self = Self::from_bits(0b1000);
    /// 1
    pub const ONE: Self = Self::from_bits(0b0010);
    /// -1
    pub const NEG_ONE: Self = Self::from_bits(0b1010);

    /// Construct an 8-bit floating point value from the raw bits.
    #[inline]
    pub const fn from_bits(bits: u8) -> Self {
        Self(bits & 0xf)
    }

    /// Construct an 8-bit floating point value from the raw bits.
    ///
    /// # Safety
    ///
    /// Undefined behavior if any of the high 4 bits of `bits` are set
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u8) -> Self {
        unsafe { assert_unchecked(bits & 0xf0 == 0); }

        Self(bits)
    }

    /// Return the raw bits.
    #[inline]
    pub const fn to_bits(self) -> u8 {
        self.0
    }

    /// Losslessly converts this type into a [`struct@f16`] type.
    #[inline]
    #[cfg(feature = "half")]
    pub const fn from_f16(x: f16) -> Self {
        Self(float4::from_f16(x))
    }

    /// Losslessly converts this type into a [`struct@f16`] type.
    #[inline]
    #[cfg(feature = "half")]
    pub const fn to_f16(self) -> f16 {
        unsafe { float4::to_f16(self.0) }
    }

    /// Losslessly converts this type into a [`struct@bf16`] type.
    #[inline]
    #[cfg(feature = "half")]
    pub const fn from_bf16(x: bf16) -> Self {
        Self(float4::from_bf16(x))
    }

    /// Losslessly converts this type into a [`struct@bf16`] type.
    #[inline]
    #[cfg(feature = "half")]
    pub const fn to_bf16(self) -> bf16 {
        unsafe { float4::to_bf16(self.0) }
    }

    /// Losslessly converts this type into a [`prim@f32`] type.
    #[inline]
    pub const fn from_f32(x: f32) -> Self {
        Self(float4::from_f32(x))
    }

    /// Losslessly converts this type into a [`prim@f32`] type.
    #[inline]
    pub const fn to_f32(self) -> f32 {
        unsafe { float4::to_f32(self.0) }
    }

    /// Losslessly converts this type into a [`prim@f64`] type.
    #[inline]
    pub const fn from_f64(x: f64) -> Self {
        Self(float4::from_f64(x))
    }

    /// Losslessly converts this type into a [`prim@f64`] type.
    #[inline]
    pub const fn to_f64(self) -> f64 {
        unsafe { float4::to_f64(self.0) }
    }

    /// Returns `true` if and only if `self` has a positive sign, including +0.0, NaNs with a positive sign bit and +∞.
    #[inline]
    pub const fn is_sign_positive(self) -> bool {
        self.0 & 0x8 == 0
    }

    /// Returns `true` if and only if `self` has a negative sign, including −0.0, NaNs with a negative sign bit and −∞.
    #[inline]
    pub const fn is_sign_negative(self) -> bool {
        self.0 & 0x8 != 0
    }

    /// Computes the absolute value of `self`.
    ///
    /// This function always returns the precise result.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use lowp::*;
    /// let x = f4::from_f32(3.5);
    /// let y = f4::from_f32(-3.5);
    ///
    /// assert_eq!(x.abs(), x);
    /// assert_eq!(y.abs(), -y);
    /// ```
    #[inline]
    pub const fn abs(self) -> Self {
        Self::from_bits(self.0 & 0x7)
    }

    /// `self + rhs`
    #[inline]
    pub const fn add_const(self, rhs: Self) -> Self {
        Self(unsafe { float4::add(self.0, rhs.0) })
    }

    /// `self * rhs`
    #[inline]
    pub const fn mul_const(self, rhs: Self) -> Self {
        Self(unsafe { float4::mul(self.0, rhs.0) })
    }

    /// `self - rhs`
    #[inline]
    pub const fn sub_const(self, rhs: Self) -> Self {
        Self(unsafe { float4::sub(self.0, rhs.0) })
    }

    /// `self / rhs`
    #[inline]
    pub const fn div_const(self, rhs: Self) -> Self {
        Self(unsafe { float4::div(self.0, rhs.0) })
    }

    /// `self % rhs`
    #[inline]
    pub const fn rem_const(self, rhs: Self) -> Self {
        Self(unsafe { float4::rem(self.0, rhs.0) })
    }

    /// `-self`
    #[inline]
    pub const fn neg_const(self) -> Self {
        Self(self.0 ^ 0x8)
    }
}

impl Add for f4 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self.add_const(rhs)
    }
}

impl Mul for f4 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_const(rhs)
    }
}

impl Sub for f4 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self.sub_const(rhs)
    }
}

impl Div for f4 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self.div_const(rhs)
    }
}

impl Rem for f4 {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        self.rem_const(rhs)
    }
}

impl Neg for f4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self.neg_const()
    }
}

impl From<f4> for f32 {
    fn from(value: f4) -> Self {
        value.to_f32()
    }
}

impl From<f4> for f64 {
    fn from(value: f4) -> Self {
        value.to_f64()
    }
}

impl From<f32> for f4 {
    fn from(value: f32) -> Self {
        f4::from_f32(value)
    }
}

impl From<f64> for f4 {
    fn from(value: f64) -> Self {
        f4::from_f64(value)
    }
}

impl PartialOrd for f4 {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.to_f32().partial_cmp(&other.to_f32())
    }
}

impl_io!(f4);
