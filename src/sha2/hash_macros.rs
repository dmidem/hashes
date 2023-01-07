macro_rules! define_hash {
    () => {
        use crate::sha2::hash_utils::{create_chunk, flatten};

        //const N_WORD_BYTES: usize = core::mem::size_of::<Word>();
        const N_DIGEST_BYTES: usize = N_DIGEST_WORDS * core::mem::size_of::<Word>();

        fn compute_hash_words(message: &[u8]) -> [Word; N_INNER_DIGEST_WORDS] {
            const N_MESSAGE_LEN_BYTES: usize = core::mem::size_of::<MessageLen>();

            let message_len_bytes = message_len_into_bytes::<N_MESSAGE_LEN_BYTES>(message.len());

            (0..)
                .step_by(N_CHUNK_BYTES)
                .map_while(|chunk_offset| {
                    create_chunk::<N_CHUNK_BYTES>(message, chunk_offset, &message_len_bytes)
                })
                .fold(INITIAL_DIGEST, |digest, chunk| {
                    compute_next_digest(digest, chunk)
                })
        }

        pub fn hash(message: &[u8]) -> Digest<N_DIGEST_BYTES> {
            Digest::from_bytes(flatten(
                compute_hash_words(message).map(|d| word_into_bytes(d)),
            ))
        }
    };
}

pub(crate) use define_hash;
