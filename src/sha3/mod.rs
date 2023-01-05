mod keccak;

pub mod sha256;
pub mod sha384;
pub mod sha512;

type Word = u64;

use super::digest;

#[cfg(test)]
use super::tests;
