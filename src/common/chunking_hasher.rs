pub(crate) trait IntoDigest {
    type Digest;

    fn into_digest(self) -> Self::Digest;
}

pub(crate) trait ChunkingHasher<const N_CHUNK_BYTES: usize> {
    type Digest;
    type InnerDigest: IntoDigest<Digest = Self::Digest>;

    const INITIAL_DIGEST: Self::InnerDigest;
    const N_MESSAGE_LEN_BYTES: usize;

    fn compute_next_digest(
        digest: Self::InnerDigest,
        chunk: [u8; N_CHUNK_BYTES],
    ) -> Self::InnerDigest;

    fn emit_message_len_bytes(message_len: usize, buffer: &mut [u8]);

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
            message_len if chunk_offset - message_len < 1 + Self::N_MESSAGE_LEN_BYTES => 0,

            // when all pad data already added on the previous iteration
            // i.e. when chunk_offset - message_len >= 1 + N_APPENDED_MESSAGE_LEN_BYTES
            _ => return None,
        };

        if n_used_bytes <= N_CHUNK_BYTES - Self::N_MESSAGE_LEN_BYTES {
            Self::emit_message_len_bytes(
                message.len(),
                &mut chunk[N_CHUNK_BYTES - Self::N_MESSAGE_LEN_BYTES..],
            );
        }

        Some(chunk)
    }

    fn hash(message: &[u8]) -> Self::Digest {
        (0..)
            .step_by(N_CHUNK_BYTES)
            .map_while(|chunk_offset| Self::create_chunk(message, chunk_offset))
            .fold(Self::INITIAL_DIGEST, |digest, chunk| {
                Self::compute_next_digest(digest, chunk)
            })
            .into_digest()
    }
}
