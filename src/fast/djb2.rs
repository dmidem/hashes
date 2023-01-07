use crate::digest::Digest;

pub fn hash(message: &[u8]) -> Digest<8> {
    Digest::from_bytes(u64::to_be_bytes(message.iter().fold(5381, |hash, c| {
        (hash << 5).wrapping_add(hash).wrapping_add(*c as u64) // hash * 33 + c
    })))
}

#[test]
fn test() {
    assert_eq!(
        hash("hello".as_bytes()).into_bytes(),
        [0x00, 0x00, 0x00, 0x31, 0x0f, 0x92, 0x30, 0x99]
    );

    assert_eq!(
        hash("world".as_bytes()).into_bytes(),
        [0x00, 0x00, 0x00, 0x31, 0x10, 0xa7, 0x35, 0x6D]
    );
}
