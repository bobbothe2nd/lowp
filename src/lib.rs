//! Low-precision floating point types.
//!
//! In this crate:
//! * [`hf8`]: 4-bit exponent, 3-bit mantissa, signed zero, infinities
//! * [`hf8fn`]: 4-bit exponent, 3-bit mantissa, signed zero, finite
//! * [`hf8uz`]: 4-bit exponent, 3-bit mantissa, unsigned zero, finite
//! * [`bf8`]: 5-bit exponent, 2-bit mantissa, signed zero, infinities
//! * [`bf8uz`]: 5-bit exponent, 2-bit mantissa, unsigned zero, finite
//! * [`f4`]: 2-bit exponent, 1-bit mantissa, signed zero, finite
//! * [`f4x2`]: 2 packed [`f4`] into a byte
//! * [`hf6`]: 2-bit exponent, 3-bit mantissa, signed zero, finite
//! * [`hf6x4`]: 2 packed [`hf6`] into a byte
//! * [`bf6`]: 3-bit exponent, 2-bit mantissa, signed zero, finite
//! * [`bf6x4`]: 2 packed [`bf6`] into a byte

#![no_std]
#![forbid(missing_docs, unconditional_recursion)]
#![allow(non_camel_case_types)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod bit;

mod float;

pub use float::*;
