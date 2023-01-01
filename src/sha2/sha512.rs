use super::algorithm::define_hash_algorithm;

impl Functions for u64 {
    #[inline(always)]
    fn ch(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ ((!x) & z)
    }

    #[inline(always)]
    fn maj(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    #[inline(always)]
    fn sum0(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }

    #[inline(always)]
    fn sum1(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }

    #[inline(always)]
    fn sigma0(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ x.wrapping_shr(7)
    }

    #[inline(always)]
    fn sigma1(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ x.wrapping_shr(6)
    }
}

pub type Word = u64;
pub type MessageLen = u128;

pub const N_DIGEST_WORDS: usize = 8; // 512 bits
pub const N_ROUNDS: usize = 80;
pub const N_CHUNK_BYTES: usize = 128; // 1024 bits

pub const K: [Word; N_ROUNDS] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

pub const INITIAL_DIGEST: [Word; N_DIGEST_WORDS] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

define_hash_algorithm!();

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    define_digest_convert_tests!(
        [
            0xe718483d0ce76964,
            0x4e2e42c7bc15b463,
            0x8e1f98b13b204428,
            0x5632a803afa973eb,
            0xde0ff244877ea60a,
            0x4cb0432ce577c31b,
            0xeb009c5c2c49aa2e,
            0x4eadb217ad8cc09b,
        ],
        [
            0xe7, 0x18, 0x48, 0x3d, 0x0c, 0xe7, 0x69, 0x64, 0x4e, 0x2e, 0x42, 0xc7, 0xbc, 0x15,
            0xb4, 0x63, 0x8e, 0x1f, 0x98, 0xb1, 0x3b, 0x20, 0x44, 0x28, 0x56, 0x32, 0xa8, 0x03,
            0xaf, 0xa9, 0x73, 0xeb, 0xde, 0x0f, 0xf2, 0x44, 0x87, 0x7e, 0xa6, 0x0a, 0x4c, 0xb0,
            0x43, 0x2c, 0xe5, 0x77, 0xc3, 0x1b, 0xeb, 0x00, 0x9c, 0x5c, 0x2c, 0x49, 0xaa, 0x2e,
            0x4e, 0xad, 0xb2, 0x17, 0xad, 0x8c, 0xc0, 0x9b,
        ]
    );

    define_hash_tests!(
        [
            [
                0xcf83e1357eefb8bd,
                0xf1542850d66d8007,
                0xd620e4050b5715dc,
                0x83f4a921d36ce9ce,
                0x47d0d13c5d85f2b0,
                0xff8318d2877eec2f,
                0x63b931bd47417a81,
                0xa538327af927da3e,
            ],
            [
                0xddaf35a193617aba,
                0xcc417349ae204131,
                0x12e6fa4e89a97ea2,
                0x0a9eeee64b55d39a,
                0x2192992a274fc1a8,
                0x36ba3c23a3feebbd,
                0x454d4423643ce80e,
                0x2a9ac94fa54ca49f,
            ],
            [
                0x204a8fc6dda82f0a,
                0x0ced7beb8e08a416,
                0x57c16ef468b228a8,
                0x279be331a703c335,
                0x96fd15c13b1b07f9,
                0xaa1d3bea57789ca0,
                0x31ad85c7a71dd703,
                0x54ec631238ca3445,
            ],
            [
                0x8e959b75dae313da,
                0x8cf4f72814fc143f,
                0x8f7779c6eb9f7fa1,
                0x7299aeadb6889018,
                0x501d289e4900f7e4,
                0x331b99dec4b5433a,
                0xc7d329eeb6dd2654,
                0x5e96e55b874be909,
            ],
            [
                0xe718483d0ce76964,
                0x4e2e42c7bc15b463,
                0x8e1f98b13b204428,
                0x5632a803afa973eb,
                0xde0ff244877ea60a,
                0x4cb0432ce577c31b,
                0xeb009c5c2c49aa2e,
                0x4eadb217ad8cc09b,
            ],
        ],
        [
            0xb47c933421ea2db1,
            0x49ad6e10fce6c7f9,
            0x3d0752380180ffd7,
            0xf4629a712134831d,
            0x77be6091b819ed35,
            0x2c2967a2e2d4fa50,
            0x50723c9630691f1a,
            0x05a7281dbe6c1086
        ]
    );
}
