// See https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf for details of the algorithms.

mod algorithm_functions;
mod algorithm_macros;

use algorithm_functions::Functions;

pub(crate) const N_INNER_DIGEST_WORDS: usize = 8;

pub mod sha224;
pub mod sha256;
pub mod sha384;
pub mod sha512;
