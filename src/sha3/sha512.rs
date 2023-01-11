use crate::digest::Digest;

use super::keccak::keccak;

pub fn hash(message: &[u8]) -> Digest<64> {
    Digest::from_bytes(keccak::<72, 64, 0x06>(message).unwrap())
}

crate::test_macros::define_hash_tests!(
    [
        [
            0xa6, 0x9f, 0x73, 0xcc, 0xa2, 0x3a, 0x9a, 0xc5, 0xc8, 0xb5, 0x67, 0xdc, 0x18, 0x5a,
            0x75, 0x6e, 0x97, 0xc9, 0x82, 0x16, 0x4f, 0xe2, 0x58, 0x59, 0xe0, 0xd1, 0xdc, 0xc1,
            0x47, 0x5c, 0x80, 0xa6, 0x15, 0xb2, 0x12, 0x3a, 0xf1, 0xf5, 0xf9, 0x4c, 0x11, 0xe3,
            0xe9, 0x40, 0x2c, 0x3a, 0xc5, 0x58, 0xf5, 0x00, 0x19, 0x9d, 0x95, 0xb6, 0xd3, 0xe3,
            0x01, 0x75, 0x85, 0x86, 0x28, 0x1d, 0xcd, 0x26,
        ],
        [
            0xb7, 0x51, 0x85, 0x0b, 0x1a, 0x57, 0x16, 0x8a, 0x56, 0x93, 0xcd, 0x92, 0x4b, 0x6b,
            0x09, 0x6e, 0x08, 0xf6, 0x21, 0x82, 0x74, 0x44, 0xf7, 0x0d, 0x88, 0x4f, 0x5d, 0x02,
            0x40, 0xd2, 0x71, 0x2e, 0x10, 0xe1, 0x16, 0xe9, 0x19, 0x2a, 0xf3, 0xc9, 0x1a, 0x7e,
            0xc5, 0x76, 0x47, 0xe3, 0x93, 0x40, 0x57, 0x34, 0x0b, 0x4c, 0xf4, 0x08, 0xd5, 0xa5,
            0x65, 0x92, 0xf8, 0x27, 0x4e, 0xec, 0x53, 0xf0,
        ],
        [
            0x04, 0xa3, 0x71, 0xe8, 0x4e, 0xcf, 0xb5, 0xb8, 0xb7, 0x7c, 0xb4, 0x86, 0x10, 0xfc,
            0xa8, 0x18, 0x2d, 0xd4, 0x57, 0xce, 0x6f, 0x32, 0x6a, 0x0f, 0xd3, 0xd7, 0xec, 0x2f,
            0x1e, 0x91, 0x63, 0x6d, 0xee, 0x69, 0x1f, 0xbe, 0x0c, 0x98, 0x53, 0x02, 0xba, 0x1b,
            0x0d, 0x8d, 0xc7, 0x8c, 0x08, 0x63, 0x46, 0xb5, 0x33, 0xb4, 0x9c, 0x03, 0x0d, 0x99,
            0xa2, 0x7d, 0xaf, 0x11, 0x39, 0xd6, 0xe7, 0x5e,
        ],
        [
            0xaf, 0xeb, 0xb2, 0xef, 0x54, 0x2e, 0x65, 0x79, 0xc5, 0x0c, 0xad, 0x06, 0xd2, 0xe5,
            0x78, 0xf9, 0xf8, 0xdd, 0x68, 0x81, 0xd7, 0xdc, 0x82, 0x4d, 0x26, 0x36, 0x0f, 0xee,
            0xbf, 0x18, 0xa4, 0xfa, 0x73, 0xe3, 0x26, 0x11, 0x22, 0x94, 0x8e, 0xfc, 0xfd, 0x49,
            0x2e, 0x74, 0xe8, 0x2e, 0x21, 0x89, 0xed, 0x0f, 0xb4, 0x40, 0xd1, 0x87, 0xf3, 0x82,
            0x27, 0x0c, 0xb4, 0x55, 0xf2, 0x1d, 0xd1, 0x85,
        ],
        [
            0x3c, 0x3a, 0x87, 0x6d, 0xa1, 0x40, 0x34, 0xab, 0x60, 0x62, 0x7c, 0x07, 0x7b, 0xb9,
            0x8f, 0x7e, 0x12, 0x0a, 0x2a, 0x53, 0x70, 0x21, 0x2d, 0xff, 0xb3, 0x38, 0x5a, 0x18,
            0xd4, 0xf3, 0x88, 0x59, 0xed, 0x31, 0x1d, 0x0a, 0x9d, 0x51, 0x41, 0xce, 0x9c, 0xc5,
            0xc6, 0x6e, 0xe6, 0x89, 0xb2, 0x66, 0xa8, 0xaa, 0x18, 0xac, 0xe8, 0x28, 0x2a, 0x0e,
            0x0d, 0xb5, 0x96, 0xc9, 0x0b, 0x0a, 0x7b, 0x87,
        ],
    ],
    [
        0x23, 0x5f, 0xfd, 0x53, 0x50, 0x4e, 0xf8, 0x36, 0xa1, 0x34, 0x2b, 0x48, 0x8f, 0x48, 0x3b,
        0x39, 0x6e, 0xab, 0xbf, 0xe6, 0x42, 0xcf, 0x78, 0xee, 0x0d, 0x31, 0xfe, 0xec, 0x78, 0x8b,
        0x23, 0xd0, 0xd1, 0x8d, 0x5c, 0x33, 0x95, 0x50, 0xdd, 0x59, 0x58, 0xa5, 0x00, 0xd4, 0xb9,
        0x53, 0x63, 0xda, 0x1b, 0x5f, 0xa1, 0x8a, 0xff, 0xc1, 0xba, 0xb2, 0x29, 0x2d, 0xc6, 0x3b,
        0x7d, 0x85, 0x09, 0x7c,
    ]
);
