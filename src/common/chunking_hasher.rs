pub(crate) trait IntoDigest {
    type Digest;

    fn into_digest(self) -> Self::Digest;
}

pub(crate) trait ChunkingHasher<const N_CHUNK_BYTES: usize> {
    type Digest;
    type InnerDigest: IntoDigest<Digest = Self::Digest>;

    const INITIAL_DIGEST: Self::InnerDigest;

    fn create_chunk(&self, chunk_offset: usize) -> Option<[u8; N_CHUNK_BYTES]>;

    fn compute_next_digest(
        &self,
        digest: Self::InnerDigest,
        chunk: [u8; N_CHUNK_BYTES],
    ) -> Self::InnerDigest;

    fn hash(&self) -> Self::Digest {
        (0..)
            .step_by(N_CHUNK_BYTES)
            .map_while(|chunk_offset| self.create_chunk(chunk_offset))
            .fold(Self::INITIAL_DIGEST, |digest, chunk| {
                self.compute_next_digest(digest, chunk)
            })
            .into_digest()
    }
}
