pub(crate) trait IntoDigest {
    type Digest;

    fn into_digest(self) -> Self::Digest;
}

pub(crate) trait ChunkingHasher {
    type Chunk;
    type Digest;
    type InnerDigest: IntoDigest<Digest = Self::Digest>;

    const N_CHUNK_BYTES: usize;
    const INITIAL_DIGEST: Self::InnerDigest;

    fn create_chunk(&self, chunk_offset: usize) -> Option<Self::Chunk>;

    fn compute_next_digest(
        &self,
        digest: Self::InnerDigest,
        chunk: Self::Chunk,
    ) -> Self::InnerDigest;

    fn hash(&self) -> Self::Digest {
        (0..)
            .step_by(Self::N_CHUNK_BYTES)
            .map_while(|chunk_offset| self.create_chunk(chunk_offset))
            .fold(Self::INITIAL_DIGEST, |digest, chunk| {
                self.compute_next_digest(digest, chunk)
            })
            .into_digest()
    }
}
