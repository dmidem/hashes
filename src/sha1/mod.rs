mod algorithm;

use algorithm::define_hash_algorithm;

pub type Word = u32;
pub type MessageLen = u64;

pub const N_DIGEST_WORDS: usize = 5; // 256 bits
pub const N_CHUNK_BYTES: usize = 64; // 512 bits
pub const N_ROUNDS: usize = 80;

pub const INITIAL_DIGEST: [Word; N_DIGEST_WORDS] =
    [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

define_hash_algorithm!();

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    define_digest_convert_tests!(
        [0x34aa973c, 0xd4c4daa4, 0xf61eeb2b, 0xdbad2731, 0x6534016f],
        [
            0x34, 0xaa, 0x97, 0x3c, 0xd4, 0xc4, 0xda, 0xa4, 0xf6, 0x1e, 0xeb, 0x2b, 0xdb, 0xad,
            0x27, 0x31, 0x65, 0x34, 0x01, 0x6f
        ]
    );

    define_hash_tests!(
        [
            [0xda39a3ee, 0x5e6b4b0d, 0x3255bfef, 0x95601890, 0xafd80709],
            [0xa9993e36, 0x4706816a, 0xba3e2571, 0x7850c26c, 0x9cd0d89d],
            [0x84983e44, 0x1c3bd26e, 0xbaae4aa1, 0xf95129e5, 0xe54670f1],
            [0xa49b2446, 0xa02c645b, 0xf419f995, 0xb6709125, 0x3a04a259],
            [0x34aa973c, 0xd4c4daa4, 0xf61eeb2b, 0xdbad2731, 0x6534016f]
        ],
        [0x7789f0c9, 0xef7bfc40, 0xd9331114, 0x3dfbe69e, 0x2017f592]
    );
}
