macro_rules! define_hash_algorithm {
    () => {
        use crate::{digest::Digest, hash_macros::define_hash};

        pub const N_INNER_DIGEST_WORDS: usize = 4;
        const N_WORD_BYTES: usize = core::mem::size_of::<Word>();

        fn s(round: usize) -> u32 {
            match round {
                0..=15 => [7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22][round],
                16..=31 => [5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20][round - 16],
                32..=47 => [4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23][round - 32],
                _ => [6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21][round - 48], // 46..=64
            }
        }

        fn k(round: usize) -> u32 {
            match round {
                0..=3 => [0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee][round],
                4..=7 => [0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501][round - 4],
                8..=11 => [0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be][round - 8],
                12..=15 => [0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821][round - 12],
                16..=19 => [0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa][round - 16],
                20..=23 => [0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8][round - 20],
                24..=27 => [0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed][round - 24],
                28..=31 => [0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a][round - 28],
                32..=35 => [0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c][round - 32],
                36..=39 => [0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70][round - 36],
                40..=43 => [0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05][round - 40],
                44..=47 => [0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665][round - 44],
                48..=51 => [0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039][round - 48],
                52..=55 => [0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1][round - 52],
                56..=59 => [0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1][round - 56],
                _ => [0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391][round - 60], // 60..=63
            }
        }

        fn break_chunk_into_words(chunk: [u8; N_CHUNK_BYTES]) -> [Word; 16] {
            let mut m = [0; 16];

            m.iter_mut()
                .zip(chunk.chunks(N_WORD_BYTES))
                .for_each(|(wi, wi_bytes)| *wi = Word::from_le_bytes(wi_bytes.try_into().unwrap()));

            m
        }

        #[inline(always)]
        fn compute_next_digest(
            digest: [Word; N_INNER_DIGEST_WORDS],
            chunk: [u8; N_CHUNK_BYTES],
        ) -> [Word; N_INNER_DIGEST_WORDS] {
            let m = break_chunk_into_words(chunk);

            let chunk_digest = (0..N_ROUNDS).fold(digest, |[a, b, c, d], i| {
                let (f, g) = match i {
                    0..=15 => ((b & c) | ((!b) & d), i),
                    16..=31 => ((d & b) | ((!d) & c), (5 * i + 1) & 0x0f),
                    32..=47 => (b ^ c ^ d, (3 * i + 5) & 0x0f),
                    _ => (c ^ (b | (!d)), (7 * i) & 0x0f), // 48..=63
                };

                let f = f.wrapping_add(a).wrapping_add(k(i)).wrapping_add(m[g]);

                [d, b.wrapping_add(f.rotate_left(s(i))), b, c]
            });

            [
                digest[0].wrapping_add(chunk_digest[0]),
                digest[1].wrapping_add(chunk_digest[1]),
                digest[2].wrapping_add(chunk_digest[2]),
                digest[3].wrapping_add(chunk_digest[3]),
            ]
        }

        #[inline(always)]
        fn message_len_into_bytes<const N_MESSAGE_LEN_BYTES: usize>(
            message_len: usize,
        ) -> [u8; N_MESSAGE_LEN_BYTES] {
            (message_len as u128 * 8u128).to_le_bytes()[..N_MESSAGE_LEN_BYTES]
                .try_into()
                .unwrap()
        }

        #[inline(always)]
        fn word_into_bytes(d: Word) -> [u8; N_WORD_BYTES] {
            d.to_le_bytes()
        }

        define_hash!();
    };
}

pub(super) use define_hash_algorithm;
