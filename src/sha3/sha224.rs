use crate::digest::Digest;

use super::keccak::keccak;

pub fn hash(message: &[u8]) -> Digest<28> {
    Digest::from_bytes(keccak::<144, 28, 0x06>(message))
}

crate::test_macros::define_hash_tests!(
    [
        [
            0x6b, 0x4e, 0x03, 0x42, 0x36, 0x67, 0xdb, 0xb7, 0x3b, 0x6e, 0x15, 0x45, 0x4f, 0x0e,
            0xb1, 0xab, 0xd4, 0x59, 0x7f, 0x9a, 0x1b, 0x07, 0x8e, 0x3f, 0x5b, 0x5a, 0x6b, 0xc7,
        ],
        [
            0xe6, 0x42, 0x82, 0x4c, 0x3f, 0x8c, 0xf2, 0x4a, 0xd0, 0x92, 0x34, 0xee, 0x7d, 0x3c,
            0x76, 0x6f, 0xc9, 0xa3, 0xa5, 0x16, 0x8d, 0x0c, 0x94, 0xad, 0x73, 0xb4, 0x6f, 0xdf,
        ],
        [
            0x8a, 0x24, 0x10, 0x8b, 0x15, 0x4a, 0xda, 0x21, 0xc9, 0xfd, 0x55, 0x74, 0x49, 0x44,
            0x79, 0xba, 0x5c, 0x7e, 0x7a, 0xb7, 0x6e, 0xf2, 0x64, 0xea, 0xd0, 0xfc, 0xce, 0x33,
        ],
        [
            0x54, 0x3e, 0x68, 0x68, 0xe1, 0x66, 0x6c, 0x1a, 0x64, 0x36, 0x30, 0xdf, 0x77, 0x36,
            0x7a, 0xe5, 0xa6, 0x2a, 0x85, 0x07, 0x0a, 0x51, 0xc1, 0x4c, 0xbf, 0x66, 0x5c, 0xbc,
        ],
        [
            0xd6, 0x93, 0x35, 0xb9, 0x33, 0x25, 0x19, 0x2e, 0x51, 0x6a, 0x91, 0x2e, 0x6d, 0x19,
            0xa1, 0x5c, 0xb5, 0x1c, 0x6e, 0xd5, 0xc1, 0x52, 0x43, 0xe7, 0xa7, 0xfd, 0x65, 0x3c,
        ],
    ],
    [
        0xc6, 0xd6, 0x6e, 0x77, 0xae, 0x28, 0x95, 0x66, 0xaf, 0xb2, 0xce, 0x39, 0x27, 0x77, 0x52,
        0xd6, 0xda, 0x2a, 0x3c, 0x46, 0x01, 0x0f, 0x1e, 0x0a, 0x09, 0x70, 0xff, 0x60,
    ]
);
