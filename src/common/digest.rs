fn flatten<const N_WORD_BYTES: usize, const N_WORDS: usize, const N_FLAT_BYTES: usize>(
    word_bytes: [[u8; N_WORD_BYTES]; N_WORDS],
) -> [u8; N_FLAT_BYTES] {
    let mut flat_bytes = [0u8; N_FLAT_BYTES];

    word_bytes
        .into_iter()
        .zip(flat_bytes.chunks_mut(N_WORD_BYTES))
        .for_each(|(w, f)| f.copy_from_slice(&w));

    flat_bytes
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Digest<const N_BYTES: usize>([u8; N_BYTES]);

impl<const N_BYTES: usize> Digest<N_BYTES> {
    pub fn from_bytes(bytes: [u8; N_BYTES]) -> Self {
        Self(bytes)
    }

    pub fn from_word_bytes<const N_WORD_BYTES: usize, const N_WORDS: usize>(
        word_bytes: [[u8; N_WORD_BYTES]; N_WORDS],
    ) -> Self {
        Self::from_bytes(flatten(word_bytes))
    }

    pub fn into_bytes(self) -> [u8; N_BYTES] {
        self.0
    }
}

#[cfg(any(feature = "std", doctest))]
mod with_std {
    use super::Digest;

    impl<const N_BYTES: usize> std::fmt::Display for Digest<N_BYTES> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for b in self.0 {
                write!(f, "{:02x}", b)?
            }
            Ok(())
        }
    }

    impl<const N_BYTES: usize> std::fmt::Debug for Digest<N_BYTES> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "\"{}\"", self)
        }
    }

    impl<const N_BYTES: usize> std::str::FromStr for Digest<N_BYTES> {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut bytes = [0; N_BYTES];

            if s.len() != 2 * N_BYTES {
                return Err("Invalid digest string length".into());
            }

            bytes
                .iter_mut()
                .zip(s.as_bytes().chunks(2).map(std::str::from_utf8))
                .try_for_each(|(byte, chunk)| {
                    chunk
                        .map_err(|_| "Invalid utf8 character in digest string".to_string())
                        .and_then(|chunk| {
                            u8::from_str_radix(chunk, 16)
                                .map_err(|_| "Invalid hex character in digest string".into())
                                .map(|value| {
                                    *byte = value;
                                })
                        })
                })?;

            Ok(Self(bytes))
        }
    }
}
