pub trait Functions {
    fn ch(x: Self, y: Self, z: Self) -> Self;
    fn maj(x: Self, y: Self, z: Self) -> Self;

    fn sum0(x: Self) -> Self;
    fn sum1(x: Self) -> Self;

    fn sigma0(x: Self) -> Self;
    fn sigma1(x: Self) -> Self;
}

macro_rules! define_hash_algorithm {
    () => {
        use crate::{digest::Digest, hash_macros::define_hash};

        use super::algorithm::Functions;

        pub const N_INNER_DIGEST_WORDS: usize = 8;
        pub const N_WORD_BYTES: usize = core::mem::size_of::<Word>();

        fn create_message_schedule(chunk: [u8; N_CHUNK_BYTES]) -> [Word; N_ROUNDS] {
            let mut w = [0; N_ROUNDS];

            w.iter_mut()
                .zip(chunk.chunks(N_WORD_BYTES))
                .for_each(|(wi, wi_bytes)| *wi = Word::from_be_bytes(wi_bytes.try_into().unwrap()));

            (16..N_ROUNDS).for_each(|i| {
                let s0 = Word::sigma0(w[i - 15]);
                let s1 = Word::sigma1(w[i - 2]);

                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1)
            });

            w
        }

        #[inline(always)]
        fn compute_next_digest(
            digest: [Word; N_INNER_DIGEST_WORDS],
            chunk: [u8; N_CHUNK_BYTES],
        ) -> [Word; N_INNER_DIGEST_WORDS] {
            let w = create_message_schedule(chunk);

            let chunk_digest = (0..N_ROUNDS).fold(digest, |[a, b, c, d, e, f, g, h], i| {
                let s0 = Word::sum0(a);
                let s1 = Word::sum1(e);
                let maj = Word::maj(a, b, c);
                let ch = Word::ch(e, f, g);

                let t1 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(K[i])
                    .wrapping_add(w[i]);

                let t2 = s0.wrapping_add(maj);

                [t1.wrapping_add(t2), a, b, c, d.wrapping_add(t1), e, f, g]
            });

            [
                digest[0].wrapping_add(chunk_digest[0]),
                digest[1].wrapping_add(chunk_digest[1]),
                digest[2].wrapping_add(chunk_digest[2]),
                digest[3].wrapping_add(chunk_digest[3]),
                digest[4].wrapping_add(chunk_digest[4]),
                digest[5].wrapping_add(chunk_digest[5]),
                digest[6].wrapping_add(chunk_digest[6]),
                digest[7].wrapping_add(chunk_digest[7]),
            ]
        }

        #[inline(always)]
        fn message_len_into_bytes<const N_MESSAGE_LEN_BYTES: usize>(
            message_len: usize,
        ) -> [u8; N_MESSAGE_LEN_BYTES] {
            (message_len as u128 * 8u128).to_be_bytes()[16 - N_MESSAGE_LEN_BYTES..]
                .try_into()
                .unwrap()
        }

        #[inline(always)]
        fn word_into_bytes(d: Word) -> [u8; N_WORD_BYTES] {
            d.to_be_bytes()
        }

        define_hash!();
    };
}

pub(super) use define_hash_algorithm;
