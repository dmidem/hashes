#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(std), test))]
extern crate alloc;

mod digest;

mod test_macros;

pub mod sha1;
pub mod sha2;
pub mod sha3;
