use crate::{
    chunking_hasher::{ChunkingHasher, IntoDigest},
    digest, hash_utils,
};

type Word = u32;
type MessageLen = u64;

const N_DIGEST_BYTES: usize = 20; // 160 bits
const N_CHUNK_BYTES: usize = 64; // 512 bits
const N_INNER_DIGEST_WORDS: usize = 5;
const N_ROUNDS: usize = 80;

const N_WORD_BYTES: usize = core::mem::size_of::<Word>();
const N_MESSAGE_LEN_BYTES: usize = core::mem::size_of::<MessageLen>();

const INITIAL_DIGEST: [Word; N_INNER_DIGEST_WORDS] =
    [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];

struct Algorithm<'a> {
    message: &'a [u8],
    message_len_bytes: [u8; N_MESSAGE_LEN_BYTES],
}

impl<'a> Algorithm<'a> {
    fn new(message: &'a [u8]) -> Self {
        Self {
            message,
            message_len_bytes: (message.len() as u64 * 8u64).to_be_bytes(),
        }
    }

    #[inline(always)]
    fn create_message_schedule(chunk: [u8; N_CHUNK_BYTES]) -> [Word; N_ROUNDS] {
        let mut w = [0; N_ROUNDS];

        w.iter_mut()
            .zip(chunk.chunks(N_WORD_BYTES))
            .for_each(|(wi, wi_bytes)| *wi = Word::from_be_bytes(wi_bytes.try_into().unwrap()));

        (16..N_ROUNDS).for_each(|i| {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        });

        w
    }
}

struct InnerDigest([Word; N_INNER_DIGEST_WORDS]);

impl IntoDigest for InnerDigest {
    type Digest = digest::Digest<N_DIGEST_BYTES>;

    fn into_digest(self) -> digest::Digest<N_DIGEST_BYTES> {
        digest::Digest::from_bytes(hash_utils::flatten(self.0.map(|d| d.to_be_bytes())))
    }
}

impl<'a> ChunkingHasher for Algorithm<'a> {
    type Chunk = [u8; N_CHUNK_BYTES];
    type Digest = digest::Digest<N_DIGEST_BYTES>;
    type InnerDigest = InnerDigest;

    const N_CHUNK_BYTES: usize = N_CHUNK_BYTES;
    const INITIAL_DIGEST: InnerDigest = InnerDigest(INITIAL_DIGEST);

    fn create_chunk(&self, chunk_offset: usize) -> Option<Self::Chunk> {
        hash_utils::create_chunk::<N_CHUNK_BYTES>(
            self.message,
            chunk_offset,
            &self.message_len_bytes,
        )
    }

    #[inline(always)]
    fn compute_next_digest(&self, digest: InnerDigest, chunk: [u8; N_CHUNK_BYTES]) -> InnerDigest {
        let digest = digest.0;

        let w = Self::create_message_schedule(chunk);

        let chunk_digest = (0..N_ROUNDS).fold(digest, |[a, b, c, d, e], i| {
            let (f, k) = match i {
                0..=19 => ((b & c) ^ ((!b) & d), 0x5a827999),
                20..=39 => (b ^ c ^ d, 0x6ed9eba1),
                40..=59 => ((b & c) ^ (b & d) ^ (c & d), 0x8f1bbcdc),
                _ => (b ^ c ^ d, 0xca62c1d6), // 60..=79
            };
            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);

            [t, a, b.rotate_left(30), c, d]
        });

        InnerDigest([
            digest[0].wrapping_add(chunk_digest[0]),
            digest[1].wrapping_add(chunk_digest[1]),
            digest[2].wrapping_add(chunk_digest[2]),
            digest[3].wrapping_add(chunk_digest[3]),
            digest[4].wrapping_add(chunk_digest[4]),
        ])
    }
}

pub fn hash(message: &[u8]) -> digest::Digest<N_DIGEST_BYTES> {
    Algorithm::new(message).hash()
}
