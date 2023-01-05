use crate::digest::Digest;

pub fn hash(message: &[u8]) -> Digest<8> {
    Digest::from_bytes(u64::to_be_bytes(message.iter().fold(0, |hash, c| {
        (*c as u64)
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash)
    })))
}

#[test]
fn test() {
    assert_eq!(
        hash("hello".as_bytes()).into_bytes(),
        [0x66, 0xeb, 0x1b, 0xb3, 0x28, 0xd1, 0x99, 0x32]
    );

    assert_eq!(
        hash("world".as_bytes()).into_bytes(),
        [0x75, 0xbe, 0x97, 0x5b, 0xf7, 0xe3, 0xae, 0xb2]
    );
}
