use super::{digest::define_digest, Word};

pub const N_DIGEST_WORDS: usize = 8; // 512 bits

define_digest!();

pub fn hash(message: &[u8]) -> Digest {
    Digest::from_bytes(super::keccak::keccak::<64>(72, message, 0x06).unwrap())
}

#[cfg(test)]
mod tests {
    use super::{super::tests::*, *};

    define_hash_tests!(
        [
            [
                0xa69f73cca23a9ac5,
                0xc8b567dc185a756e,
                0x97c982164fe25859,
                0xe0d1dcc1475c80a6,
                0x15b2123af1f5f94c,
                0x11e3e9402c3ac558,
                0xf500199d95b6d3e3,
                0x01758586281dcd26
            ],
            [
                0xb751850b1a57168a,
                0x5693cd924b6b096e,
                0x08f621827444f70d,
                0x884f5d0240d2712e,
                0x10e116e9192af3c9,
                0x1a7ec57647e39340,
                0x57340b4cf408d5a5,
                0x6592f8274eec53f0
            ],
            [
                0x04a371e84ecfb5b8,
                0xb77cb48610fca818,
                0x2dd457ce6f326a0f,
                0xd3d7ec2f1e91636d,
                0xee691fbe0c985302,
                0xba1b0d8dc78c0863,
                0x46b533b49c030d99,
                0xa27daf1139d6e75e
            ],
            [
                0xafebb2ef542e6579,
                0xc50cad06d2e578f9,
                0xf8dd6881d7dc824d,
                0x26360feebf18a4fa,
                0x73e3261122948efc,
                0xfd492e74e82e2189,
                0xed0fb440d187f382,
                0x270cb455f21dd185
            ],
            [
                0x3c3a876da14034ab,
                0x60627c077bb98f7e,
                0x120a2a5370212dff,
                0xb3385a18d4f38859,
                0xed311d0a9d5141ce,
                0x9cc5c66ee689b266,
                0xa8aa18ace8282a0e,
                0x0db596c90b0a7b87
            ],
        ],
        [
            0x235ffd53504ef836,
            0xa1342b488f483b39,
            0x6eabbfe642cf78ee,
            0x0d31feec788b23d0,
            0xd18d5c339550dd59,
            0x58a500d4b95363da,
            0x1b5fa18affc1bab2,
            0x292dc63b7d85097c
        ]
    );
}
