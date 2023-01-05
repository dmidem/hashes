use super::{digest::define_digest, Word};

pub const N_DIGEST_WORDS: usize = 6; // 384 bits

define_digest!();

pub fn hash(message: &[u8]) -> Digest {
    Digest::from_bytes(super::keccak::keccak::<48>(104, message, 0x06).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    define_hash_tests!(
        [
            [
                0x0c63a75b845e4f7d,
                0x01107d852e4c2485,
                0xc51a50aaaa94fc61,
                0x995e71bbee983a2a,
                0xc3713831264adb47,
                0xfb6bd1e058d5f004
            ],
            [
                0xec01498288516fc9,
                0x26459f58e2c6ad8d,
                0xf9b473cb0fc08c25,
                0x96da7cf0e49be4b2,
                0x98d88cea927ac7f5,
                0x39f1edf228376d25
            ],
            [
                0x991c665755eb3a4b,
                0x6bbdfb75c78a492e,
                0x8c56a22c5c4d7e42,
                0x9bfdbc32b9d4ad5a,
                0xa04a1f076e62fea1,
                0x9eef51acd0657c22
            ],
            [
                0x79407d3b5916b59c,
                0x3e30b09822974791,
                0xc313fb9ecc849e40,
                0x6f23592d04f625dc,
                0x8c709b98b43b3852,
                0xb337216179aa7fc7
            ],
            [
                0xeee9e24d78c18553,
                0x37983451df97c8ad,
                0x9eedf256c6334f8e,
                0x948d252d5e0e7684,
                0x7aa0774ddb90a842,
                0x190d2c558b4b8340
            ],
        ],
        [
            0xa04296f4fcaae148,
            0x71bb5ad33e28dcf6,
            0x9238b04204d9941b,
            0x8782e816d014bcb7,
            0x540e4af54f30d578,
            0xf1a1ca2930847a12
        ]
    );
}
