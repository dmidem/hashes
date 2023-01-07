#![cfg_attr(not(any(feature = "std", doctest)), no_std)]

mod common;

use common::{digest, hash_macros, hash_utils, test_macros};

pub mod fast;

pub mod md5;

pub mod sha1;
pub mod sha2;
pub mod sha3;

mod test_readme;
