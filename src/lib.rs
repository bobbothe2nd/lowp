//! Low-precision floating point types.
//!
//! This crate provides 5 types:
//! * [`hf8`]: 4-bit exponent, 3-bit mantissa, signed zero, infinities
//! * [`hf8fn`]: 4-bit exponent, 3-bit mantissa, signed zero, finite
//! * [`hf8uz`]: 4-bit exponent, 3-bit mantissa, unsigned zero, finite
//! * [`bf8`]: 5-bit exponent, 2-bit mantissa, signed zero, infinities
//! * [`bf8uz`]: 5-bit exponent, 2-bit mantissa, unsigned zero, finite

#![no_std]

#![forbid(missing_docs)]
#![allow(non_camel_case_types)]

mod bit;

mod float;

pub use float::*;
