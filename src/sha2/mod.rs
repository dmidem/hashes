// See https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf for a detailed description
// of the algorithms.
// Test values are taken from here: https://www.di-mgt.com.au/sha_testvectors.html

mod algorithm;

pub mod sha224;
pub mod sha256;
pub mod sha384;
pub mod sha512;

use super::{digest, hash, N_BYTE_BITS};

#[cfg(test)]
use super::tests;
