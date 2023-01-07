pub(crate) fn create_chunk<const N_CHUNK_BYTES: usize>(
    message: &[u8],
    chunk_offset: usize,
    message_len_bytes: &[u8],
) -> Option<[u8; N_CHUNK_BYTES]> {
    let message_len_bytes_len = message_len_bytes.len();

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
        message_len if chunk_offset - message_len < 1 + message_len_bytes_len => 0,

        // when all pad data already added on the previous iteration
        // i.e. when chunk_offset - message_len >= 1 + N_APPENDED_MESSAGE_LEN_BYTES
        _ => return None,
    };

    if n_used_bytes <= N_CHUNK_BYTES - message_len_bytes_len {
        chunk[N_CHUNK_BYTES - message_len_bytes_len..N_CHUNK_BYTES]
            .copy_from_slice(message_len_bytes)
    }

    Some(chunk)
}

pub(crate) fn flatten<
    const N_WORD_BYTES: usize,
    const N_WORDS: usize,
    const N_FLAT_BYTES: usize,
>(
    word_bytes: [[u8; N_WORD_BYTES]; N_WORDS],
) -> [u8; N_FLAT_BYTES] {
    let mut flat_bytes = [0u8; N_FLAT_BYTES];

    word_bytes
        .into_iter()
        .zip(flat_bytes.chunks_mut(N_WORD_BYTES))
        .for_each(|(w, f)| f.copy_from_slice(&w));

    flat_bytes
}
