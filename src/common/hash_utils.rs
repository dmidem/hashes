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
