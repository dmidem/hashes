// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

pub trait Functions {
    fn ch(x: Self, y: Self, z: Self) -> Self;
    fn maj(x: Self, y: Self, z: Self) -> Self;

    fn sum0(x: Self) -> Self;
    fn sum1(x: Self) -> Self;

    fn sigma0(x: Self) -> Self;
    fn sigma1(x: Self) -> Self;
}

pub const N_BYTE_BITS: usize = 8;

macro_rules! define_hash_algorithm {
    //($mod:ident, $mod_args:ident) => {
    () => {
        //mod $mod {
        use super::algorithm::{Functions, N_BYTE_BITS};
        //use $mod_args::*;

        pub const N_WORD_BYTES: usize = std::mem::size_of::<Word>();
        pub const N_MESSAGE_LEN_BYTES: usize = std::mem::size_of::<MessageLen>();
        pub const N_INNER_DIGEST_WORDS: usize = 8; // FIXME: 8 words are hardcoded here - is that correct?
        pub const N_DIGEST_BYTES: usize = N_DIGEST_WORDS * N_WORD_BYTES;

        fn create_chunk(message: &[u8], chunk_offset: usize) -> Option<[u8; N_CHUNK_BYTES]> {
            let mut chunk = [0u8; N_CHUNK_BYTES];

            let n_used_bytes = match message.len() {
                message_len if chunk_offset < message_len => {
                    let n = (message_len - chunk_offset).min(N_CHUNK_BYTES);
                    chunk[0..n].copy_from_slice(&message[chunk_offset..chunk_offset + n]);
                    if n < N_CHUNK_BYTES {
                        chunk[n] = 0x80;
                        n + 1
                    } else {
                        n
                    }
                }

                message_len if chunk_offset == message_len => {
                    chunk[0] = 0x80;
                    1
                }

                // when 0x80 byte added but length bytes could not be added on the previous iteration
                message_len if chunk_offset - message_len < 1 + N_MESSAGE_LEN_BYTES => 0,

                // when all pad data already added on the previous iteration
                // i.e. when chunk_offset - message_len >= 1 + N_APPENDED_MESSAGE_LEN_BYTES
                _ => return None,
            };

            if n_used_bytes <= N_CHUNK_BYTES - N_MESSAGE_LEN_BYTES {
                chunk[N_CHUNK_BYTES - N_MESSAGE_LEN_BYTES..N_CHUNK_BYTES].copy_from_slice(
                    &(message.len() as MessageLen * N_BYTE_BITS as MessageLen).to_be_bytes(),
                );
            }

            Some(chunk)
        }

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
            w: [Word; N_ROUNDS],
        ) -> [Word; N_INNER_DIGEST_WORDS] {
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

        #[derive(PartialEq, Eq, Clone, Copy, Hash)]
        pub struct Digest([Word; N_DIGEST_WORDS]);

        impl Digest {
            pub fn from_inner_digest(inner_digest: [Word; N_INNER_DIGEST_WORDS]) -> Self {
                assert!(N_DIGEST_WORDS <= N_INNER_DIGEST_WORDS);
                Self(unsafe {
                    std::mem::transmute_copy::<[Word; N_INNER_DIGEST_WORDS], [Word; N_DIGEST_WORDS]>(
                        &inner_digest
                    )
                })
            }

            pub fn into_words(self) -> [Word; N_DIGEST_WORDS] {
                self.0
            }

            pub fn from_words(words: [Word; N_DIGEST_WORDS]) -> Self {
                Self(words)
            }

            pub fn into_bytes(self) -> [u8; N_DIGEST_BYTES] {
                let mut bytes = [0u8; N_DIGEST_BYTES];

                self.0
                    .map(|d| d.to_be_bytes())
                    .into_iter()
                    .zip(bytes.chunks_mut(N_WORD_BYTES))
                    .for_each(|(d, b)| b.copy_from_slice(&d));

                bytes
            }

            pub fn from_bytes(bytes: [u8; N_DIGEST_BYTES]) -> Self {
                let mut words = [0; N_DIGEST_WORDS];

                words
                    .iter_mut()
                    .zip(
                        bytes
                            .chunks(N_WORD_BYTES)
                            .map(|chunk| Word::from_be_bytes(chunk.try_into().unwrap())),
                    )
                    .for_each(|(w, b)| *w = b);

                Self(words)
            }
        }

        impl std::fmt::Display for Digest {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for b in self.into_bytes() {
                    write!(f, "{:02x}", b)?
                }
                Ok(())
            }
        }

        impl std::fmt::Debug for Digest {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "\"{}\"", self)
            }
        }

        impl std::str::FromStr for Digest {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut words = [0; N_DIGEST_WORDS];

                if s.len() != N_DIGEST_WORDS * 2 * N_WORD_BYTES {
                    return Err("Invalid digest string length".into());
                }

                words
                    .iter_mut()
                    .zip(
                        s.as_bytes()
                            .chunks(2 * N_WORD_BYTES)
                            .map(std::str::from_utf8),
                    )
                    .try_for_each(|(word, chunk)| {
                        chunk
                            .map_err(|_| "Invalid utf8 character in digest string".to_string())
                            .and_then(|chunk| {
                                Word::from_str_radix(chunk, 16)
                                    .map_err(|_| "Invalid hex character in digest string".into())
                                    .map(|value| {
                                        *word = value;
                                    })
                            })
                    })?;

                Ok(Self(words))
            }
        }

        pub fn hash(message: &[u8]) -> Digest {
            Digest::from_inner_digest(
                (0..)
                    .step_by(N_CHUNK_BYTES)
                    .map_while(|chunk_offset| create_chunk(message, chunk_offset))
                    .fold(INITIAL_DIGEST, |digest, chunk| {
                        compute_next_digest(digest, create_message_schedule(chunk))
                    }),
            )
        }
        //}
    };
}

pub(super) use define_hash_algorithm;
