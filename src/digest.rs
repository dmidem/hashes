macro_rules! define_digest {
    () => {
        #[derive(PartialEq, Eq, Clone, Copy, Hash)]
        pub struct Digest([Word; N_DIGEST_WORDS]);

        impl Digest {
            pub const N_WORD_BYTES: usize = std::mem::size_of::<Word>();
            pub const N_DIGEST_BYTES: usize = N_DIGEST_WORDS * Self::N_WORD_BYTES;

            pub fn from_inner_digest(inner_digest: [Word; N_INNER_DIGEST_WORDS]) -> Self {
                Self(inner_digest[0..N_DIGEST_WORDS].try_into().unwrap())
            }

            pub fn into_words(self) -> [Word; N_DIGEST_WORDS] {
                self.0
            }

            pub fn from_words(words: [Word; N_DIGEST_WORDS]) -> Self {
                Self(words)
            }

            pub fn into_bytes(self) -> [u8; Self::N_DIGEST_BYTES] {
                let mut bytes = [0u8; Self::N_DIGEST_BYTES];

                self.0
                    .map(|d| d.to_be_bytes())
                    .into_iter()
                    .zip(bytes.chunks_mut(Self::N_WORD_BYTES))
                    .for_each(|(d, b)| b.copy_from_slice(&d));

                bytes
            }

            pub fn from_bytes(bytes: [u8; Self::N_DIGEST_BYTES]) -> Self {
                let mut words = [0; N_DIGEST_WORDS];

                words
                    .iter_mut()
                    .zip(
                        bytes
                            .chunks(Self::N_WORD_BYTES)
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

                if s.len() != N_DIGEST_WORDS * 2 * Self::N_WORD_BYTES {
                    return Err("Invalid digest string length".into());
                }

                words
                    .iter_mut()
                    .zip(
                        s.as_bytes()
                            .chunks(2 * Self::N_WORD_BYTES)
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
    };
}

pub(super) use define_digest;
