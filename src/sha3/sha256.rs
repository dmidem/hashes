use super::{digest::define_digest, Word};

pub const N_DIGEST_WORDS: usize = 4; // 256 bits

define_digest!();

pub fn hash(message: &[u8]) -> Digest {
    Digest::from_bytes(super::keccak::keccak::<32>(136, message, 0x06).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    define_hash_tests!(
        [
            [
                0xa7ffc6f8bf1ed766,
                0x51c14756a061d662,
                0xf580ff4de43b49fa,
                0x82d80a4b80f8434a
            ],
            [
                0x3a985da74fe225b2,
                0x045c172d6bd390bd,
                0x855f086e3e9d525b,
                0x46bfe24511431532
            ],
            [
                0x41c0dba2a9d62408,
                0x49100376a8235e2c,
                0x82e1b9998a999e21,
                0xdb32dd97496d3376
            ],
            [
                0x916f6061fe879741,
                0xca6469b43971dfdb,
                0x28b1a32dc36cb325,
                0x4e812be27aad1d18
            ],
            [
                0x5c8875ae474a3634,
                0xba4fd55ec85bffd6,
                0x61f32aca75c6d699,
                0xd0cdcb6c115891c1
            ],
        ],
        [
            0xecbbc42cbf296603,
            0xacb2c6bc0410ef43,
            0x78bafb24b710357f,
            0x12df607758b33e2b
        ]
    );
}
