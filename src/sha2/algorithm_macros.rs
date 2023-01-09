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

            use crate::{
                chunking_hasher::{ChunkingHasher, IntoDigest},
                digest, hash_utils,
            };

            const N_MESSAGE_LEN_BYTES: usize = core::mem::size_of::<MessageLen>();
            const N_WORD_BYTES: usize = core::mem::size_of::<Word>();

            pub(crate) struct Algorithm<'a> {
                message: &'a [u8],
                message_len_bytes: [u8; N_MESSAGE_LEN_BYTES],
            }

            impl<'a> Algorithm<'a> {
                pub(crate) fn new(message: &'a [u8]) -> Self {
                    Self {
                        message,
                        message_len_bytes: (message.len() as u128 * 8u128).to_be_bytes()
                            [16 - N_MESSAGE_LEN_BYTES..]
                            .try_into()
                            .unwrap(),
                    }
                }

                #[inline(always)]
                fn create_message_schedule(chunk: [u8; N_CHUNK_BYTES]) -> [Word; N_ROUNDS] {
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
            }

            pub(crate) struct InnerDigest([Word; N_INNER_DIGEST_WORDS]);

            impl IntoDigest for InnerDigest {
                type Digest = digest::Digest<N_DIGEST_BYTES>;

                fn into_digest(self) -> digest::Digest<N_DIGEST_BYTES> {
                    digest::Digest::from_bytes(hash_utils::flatten(self.0.map(|d| d.to_be_bytes())))
                }
            }

            impl<'a> ChunkingHasher for Algorithm<'a> {
                type Chunk = [u8; N_CHUNK_BYTES];
                type Digest = digest::Digest<N_DIGEST_BYTES>;
                type InnerDigest = InnerDigest;

                const N_CHUNK_BYTES: usize = N_CHUNK_BYTES;
                const INITIAL_DIGEST: InnerDigest = InnerDigest(INITIAL_DIGEST);

                fn create_chunk(&self, chunk_offset: usize) -> Option<Self::Chunk> {
                    hash_utils::create_chunk::<N_CHUNK_BYTES>(
                        self.message,
                        chunk_offset,
                        &self.message_len_bytes,
                    )
                }

                #[inline(always)]
                fn compute_next_digest(
                    &self,
                    digest: InnerDigest,
                    chunk: [u8; N_CHUNK_BYTES],
                ) -> InnerDigest {
                    let digest = digest.0;

                    let w = Self::create_message_schedule(chunk);

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

                    InnerDigest([
                        digest[0].wrapping_add(chunk_digest[0]),
                        digest[1].wrapping_add(chunk_digest[1]),
                        digest[2].wrapping_add(chunk_digest[2]),
                        digest[3].wrapping_add(chunk_digest[3]),
                        digest[4].wrapping_add(chunk_digest[4]),
                        digest[5].wrapping_add(chunk_digest[5]),
                        digest[6].wrapping_add(chunk_digest[6]),
                        digest[7].wrapping_add(chunk_digest[7]),
                    ])
                }
            }
        }

        use crate::chunking_hasher::ChunkingHasher;

        pub fn hash(message: &[u8]) -> crate::digest::Digest<{ args::N_DIGEST_BYTES }> {
            algorithm::Algorithm::new(message).hash()
        }
    };
}

pub(super) use define_algorithm;
