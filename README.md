# Cryptographic Hash Functions in Rust

This is a Rust implementation of various cryptographic hash functions. The following algorithms are currently supported:

* SHA-1

* SHA-2-224
* SHA-2-256
* SHA-2-384
* SHA-2-512

* SHA-3-224
* SHA-3-256
* SHA-3-384
* SHA-3-512

## Example

Here's an example of how to use the `sha256` hash function:

```rust
use hashes::sha2::sha256::hash;

fn main() {
    let s = "abc";

    let digest = hash(s.as_bytes());

    println!("Hash of \"{}\" string is: {}", s, digest);

    assert_eq!(
        digest.into_bytes(),
        [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad
        ]
    );
}
```

Enable "std" feature if you want to use Display, Debug of FromStr for the output of hash functions (i.e. for Digest struct).

## License

This project is dual-licensed under the <a href="LICENSE-APACHE">Apache License 2.0</a> and the <a href="LICENSE-MIT">MIT License</a>.
You may choose either license to use this software.
