use hashes::sha2::sha256::hash;

fn main() {
    assert_eq!(
        hash("abc".as_bytes()).into_words(),
        [
            0xba7816bf, 0x8f01cfea, 0x414140de, 0x5dae2223, 0xb00361a3, 0x96177a9c, 0xb410ff61,
            0xf20015ad
        ]
    );
}
