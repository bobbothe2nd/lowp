use core::{hint::assert_unchecked, ops::{Add, Div, Mul, Sub}};

use crate::bit::float4;

#[cfg(feature = "half")]
use half::{bf16, f16};

/// Two 4-bit E2M1 floats packed into 1 byte
///
/// Use this when you need contiguous storage or want to optimize 2 floats into 1 operation.
#[derive(Clone, Copy, Default)]
pub struct f4x2(u8);

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
}

/// 4-bit E2M1 float with 4 padding bits
#[derive(Clone, Copy, Default)]
pub struct f4(u8);

#[cfg(feature = "briny")]
unsafe impl briny::traits::Layout<u8> for f4 {}

impl f4 {
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

impl_io!(f4);
