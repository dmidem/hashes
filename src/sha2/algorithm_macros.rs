macro_rules! define_algorithm {
    (
    ) => {
        mod algorithm {
            use super::{
                super::Functions,
                args::{
                    MessageLen, Word, INITIAL_DIGEST, K, N_CHUNK_BYTES, N_DIGEST_BYTES,
                    N_INNER_DIGEST_WORDS, N_ROUNDS,
                },
            };

            use crate::{chunking_hasher::ChunkingHasher, digest, hash_utils};

            #[inline(always)]
            fn create_message_schedule(chunk: [u8; N_CHUNK_BYTES]) -> [Word; N_ROUNDS] {
                const N_WORD_BYTES: usize = core::mem::size_of::<Word>();

                let mut w = [0; N_ROUNDS];

                w.iter_mut()
                    .zip(chunk.chunks(N_WORD_BYTES))
                    .for_each(|(wi, wi_bytes)| {
                        *wi = <Word>::from_be_bytes(wi_bytes.try_into().unwrap())
                    });

                (16..N_ROUNDS).for_each(|i| {
                    let s0 = <Word>::sigma0(w[i - 15]);
                    let s1 = <Word>::sigma1(w[i - 2]);

                    w[i] = w[i - 16]
                        .wrapping_add(s0)
                        .wrapping_add(w[i - 7])
                        .wrapping_add(s1)
                });

                w
            }

            pub(super) struct Algorithm;

            impl ChunkingHasher<N_CHUNK_BYTES> for Algorithm {
                type Digest = digest::Digest<N_DIGEST_BYTES>;
                type InnerDigest = [Word; N_INNER_DIGEST_WORDS];

                const INITIAL_DIGEST: Self::InnerDigest = INITIAL_DIGEST;
                const N_MESSAGE_LEN_BYTES: usize = core::mem::size_of::<MessageLen>();

                #[inline(always)]
                fn emit_message_len_bytes(message_len: usize, buffer: &mut [u8]) {
                    buffer.copy_from_slice(
                        &(message_len as u128 * 8u128).to_be_bytes()
                            [16 - Self::N_MESSAGE_LEN_BYTES..],
                    );
                }

                #[inline(always)]
                fn convert_inner_digest(inner_digest: Self::InnerDigest) -> Self::Digest {
                    digest::Digest::from_bytes(hash_utils::flatten(
                        inner_digest.map(|d| d.to_be_bytes()),
                    ))
                }

                #[inline(always)]
                fn compute_next_digest(
                    digest: Self::InnerDigest,
                    chunk: [u8; N_CHUNK_BYTES],
                ) -> Self::InnerDigest {
                    let w = create_message_schedule(chunk);

                    let chunk_digest = (0..N_ROUNDS).fold(digest, |[a, b, c, d, e, f, g, h], i| {
                        let s0 = <Word>::sum0(a);
                        let s1 = <Word>::sum1(e);
                        let maj = <Word>::maj(a, b, c);
                        let ch = <Word>::ch(e, f, g);

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
            }
        }

        use crate::chunking_hasher::ChunkingHasher;

        pub fn hash(message: &[u8]) -> crate::digest::Digest<{ args::N_DIGEST_BYTES }> {
            algorithm::Algorithm::hash(message)
        }
    };
}

pub(super) use define_algorithm;
