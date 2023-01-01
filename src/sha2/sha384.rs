use super::{
    algorithm::define_hash_algorithm,
    sha512::{MessageLen, Word, K, N_CHUNK_BYTES, N_ROUNDS},
};

pub const N_DIGEST_WORDS: usize = 6; // 384 bits

pub const INITIAL_DIGEST: [Word; N_INNER_DIGEST_WORDS] = [
    0xcbbb9d5dc1059ed8,
    0x629a292a367cd507,
    0x9159015a3070dd17,
    0x152fecd8f70e5939,
    0x67332667ffc00b31,
    0x8eb44a8768581511,
    0xdb0c2e0d64f98fa7,
    0x47b5481dbefa4fa4,
];

define_hash_algorithm!();

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    define_hash_tests!(
        [
            [
                0x38b060a751ac9638,
                0x4cd9327eb1b1e36a,
                0x21fdb71114be0743,
                0x4c0cc7bf63f6e1da,
                0x274edebfe76f65fb,
                0xd51ad2f14898b95b
            ],
            [
                0xcb00753f45a35e8b,
                0xb5a03d699ac65007,
                0x272c32ab0eded163,
                0x1a8b605a43ff5bed,
                0x8086072ba1e7cc23,
                0x58baeca134c825a7
            ],
            [
                0x3391fdddfc8dc739,
                0x3707a65b1b470939,
                0x7cf8b1d162af05ab,
                0xfe8f450de5f36bc6,
                0xb0455a8520bc4e6f,
                0x5fe95b1fe3c8452b
            ],
            [
                0x09330c33f71147e8,
                0x3d192fc782cd1b47,
                0x53111b173b3b05d2,
                0x2fa08086e3b0f712,
                0xfcc7c71a557e2db9,
                0x66c3e9fa91746039
            ],
            [
                0x9d0e1809716474cb,
                0x086e834e310a4a1c,
                0xed149e9c00f24852,
                0x7972cec5704c2a5b,
                0x07b8b3dc38ecc4eb,
                0xae97ddd87f3d8985
            ]
        ],
        [
            0x5441235cc0235341,
            0xed806a64fb354742,
            0xb5e5c02a3c5cb71b,
            0x5f63fb793458d8fd,
            0xae599c8cd8884943,
            0xc04f11b31b89f023
        ]
    );
}
