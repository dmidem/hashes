// See https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf for a detailed description
// of the algorithms.
// Test values are taken from here: https://www.di-mgt.com.au/sha_testvectors.html

mod algorithm;

#[cfg(test)]
mod tests;

pub mod sha224;
pub mod sha256;
pub mod sha384;
pub mod sha512;
