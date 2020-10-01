//! Simple tools to compute powers of a radix, or the radix decomposition of a number.
//!
//! # Examples
//! Compute the 5th power of 7:
//! ```rust
//! use radix_tools::powers::*;
//! assert_eq!(7*7*7*7*7, Powers::new(1u32, 7u32).nth(5).unwrap());
//! ```
//!
//! Compute the 2nd ternary (radix-3) digit of 42:
//! ```rust
//! use radix_tools::decomposition::*;
//! assert_eq!(2, RadixDecomposer::new(3u32, 42u32).nth(1).unwrap());
//! ```

#![no_std]
pub mod decomposition;
pub mod powers;

#[cfg(test)]
#[macro_use]
extern crate alloc;
