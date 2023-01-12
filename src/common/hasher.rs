pub trait IterativeHasher {
    type Digest;

    fn write(&mut self, bytes: &[u8]);
    fn finish(self) -> Self::Digest;
}

pub trait Hasher {
    type Digest;

    fn hash(message: &[u8]) -> Self::Digest;
}

pub(crate) trait ChunkingHasher<const N_CHUNK_BYTES: usize> {
    type Digest;
    type InnerDigest;

    const INITIAL_DIGEST: Self::InnerDigest;
    const N_MESSAGE_LEN_BYTES: usize;

    fn emit_message_len_bytes(message_len: usize, buffer: &mut [u8]);

    fn convert_inner_digest(inner_digest: Self::InnerDigest) -> Self::Digest;

    fn compute_next_digest(
        digest: &Self::InnerDigest,
        chunk: [u8; N_CHUNK_BYTES],
    ) -> Self::InnerDigest;
}

#[derive(Clone, Copy)]
pub(crate) struct BufHasher<const N_CHUNK_BYTES: usize, Inner: ChunkingHasher<N_CHUNK_BYTES>> {
    buffer: [u8; N_CHUNK_BYTES],
    buffer_index: usize,
    inner_digest: Inner::InnerDigest,
    message_len: usize,
}

impl<const N_CHUNK_BYTES: usize, Inner: ChunkingHasher<N_CHUNK_BYTES>>
    BufHasher<N_CHUNK_BYTES, Inner>
{
    pub fn new() -> Self {
        Self {
            buffer: [0u8; N_CHUNK_BYTES],
            buffer_index: 0,
            inner_digest: Inner::INITIAL_DIGEST,
            message_len: 0,
        }
    }
}

impl<const N_CHUNK_BYTES: usize, Inner: ChunkingHasher<N_CHUNK_BYTES>> Hasher
    for BufHasher<N_CHUNK_BYTES, Inner>
{
    type Digest = Inner::Digest;

    fn hash(message: &[u8]) -> Self::Digest {
        let mut hasher = Self::new();
        hasher.write(message);
        hasher.finish()
    }
}

impl<const N_CHUNK_BYTES: usize, Inner: ChunkingHasher<N_CHUNK_BYTES>> IterativeHasher
    for BufHasher<N_CHUNK_BYTES, Inner>
{
    type Digest = Inner::Digest;

    fn finish(mut self) -> Self::Digest {
        debug_assert!(self.buffer_index < self.buffer.len());

        // Add a byte with "1" bit set (i.e. 0x80)
        self.buffer[self.buffer_index] = 0x80;
        self.buffer_index += 1;

        // Add message length bytes
        let buffer_len = self.buffer.len();
        if buffer_len - self.buffer_index < Inner::N_MESSAGE_LEN_BYTES {
            self.buffer[self.buffer_index..].fill(0);
            self.inner_digest = Inner::compute_next_digest(&self.inner_digest, self.buffer);
            self.buffer_index = 0;
        }
        self.buffer[self.buffer_index..buffer_len - Inner::N_MESSAGE_LEN_BYTES].fill(0);
        Inner::emit_message_len_bytes(
            self.message_len,
            &mut self.buffer[buffer_len - Inner::N_MESSAGE_LEN_BYTES..],
        );
        self.inner_digest = Inner::compute_next_digest(&self.inner_digest, self.buffer);
        self.buffer_index = 0;

        Inner::convert_inner_digest(self.inner_digest)
    }

    fn write(&mut self, data: &[u8]) {
        let mut data_index = 0;

        while data_index < data.len() {
            debug_assert!(self.buffer_index < self.buffer.len());

            let copy_count = (data.len() - data_index).min(self.buffer.len() - self.buffer_index);

            let new_buffer_index = self.buffer_index + copy_count;
            let new_data_index = data_index + copy_count;

            self.buffer[self.buffer_index..new_buffer_index]
                .copy_from_slice(&data[data_index..new_data_index]);

            self.buffer_index = if new_buffer_index == self.buffer.len() {
                self.inner_digest = Inner::compute_next_digest(&self.inner_digest, self.buffer);
                0
            } else {
                new_buffer_index
            };

            data_index = new_data_index;
        }

        self.message_len += data.len();
    }
}
