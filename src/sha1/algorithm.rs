macro_rules! define_hash_algorithm {
    () => {
        use crate::{digest::Digest, sha2::hash_macros::define_hash};

        pub const N_INNER_DIGEST_WORDS: usize = 5;

        fn create_message_schedule(chunk: [u8; N_CHUNK_BYTES]) -> [Word; N_ROUNDS] {
            const N_WORD_BYTES: usize = core::mem::size_of::<Word>();

            let mut w = [0; N_ROUNDS];

            w.iter_mut()
                .zip(chunk.chunks(N_WORD_BYTES))
                .for_each(|(wi, wi_bytes)| *wi = Word::from_be_bytes(wi_bytes.try_into().unwrap()));

            (16..N_ROUNDS).for_each(|i| {
                w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
            });

            w
        }

        #[inline(always)]
        fn compute_next_digest(
            digest: [Word; N_INNER_DIGEST_WORDS],
            w: [Word; N_ROUNDS],
        ) -> [Word; N_INNER_DIGEST_WORDS] {
            let chunk_digest = (0..N_ROUNDS).fold(digest, |[a, b, c, d, e], i| {
                let (f, k) = match i {
                    0..=19 => ((b & c) ^ ((!b) & d), 0x5a827999),
                    20..=39 => (b ^ c ^ d, 0x6ed9eba1),
                    40..=59 => ((b & c) ^ (b & d) ^ (c & d), 0x8f1bbcdc),
                    _ => (b ^ c ^ d, 0xca62c1d6), // 60..=79
                };
                let t = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(w[i]);

                [t, a, b.rotate_left(30), c, d]
            });

            [
                digest[0].wrapping_add(chunk_digest[0]),
                digest[1].wrapping_add(chunk_digest[1]),
                digest[2].wrapping_add(chunk_digest[2]),
                digest[3].wrapping_add(chunk_digest[3]),
                digest[4].wrapping_add(chunk_digest[4]),
            ]
        }

        define_hash!();
    };
}

pub(super) use define_hash_algorithm;
