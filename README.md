# Cryptographic Hash Functions in Rust

This is a Rust implementation of various cryptographic hash functions. The following algorithms are currently supported:

* SHA-224
* SHA-256
* SHA-384
* SHA-512

## Example

Here's an example of how to use the `sha256` hash function:

```rust
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
```

## License

This project is dual-licensed under the <a href="LICENSE-APACHE">Apache License 2.0</a> and the <a href="LICENSE-MIT">MIT License</a>.
You may choose either license to use this software.
