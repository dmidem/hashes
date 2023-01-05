macro_rules! define_hash {
    () => {
        fn create_chunk(message: &[u8], chunk_offset: usize) -> Option<[u8; N_CHUNK_BYTES]> {
            use super::N_BYTE_BITS;

            const N_MESSAGE_LEN_BYTES: usize = std::mem::size_of::<MessageLen>();

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

        fn make_digest_from_inner(inner_digest: [Word; N_INNER_DIGEST_WORDS]) -> Digest {
            Digest(inner_digest[0..N_DIGEST_WORDS].try_into().unwrap())
        }

        pub fn hash(message: &[u8]) -> Digest {
            make_digest_from_inner(
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

pub(super) use define_hash;
