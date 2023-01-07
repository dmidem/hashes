use hashes::{fast, md5, sha1, sha2, sha3};

fn main() {
    let s = "abc";

    println!(
        "djb2 hash of \"{}\" string is: {}",
        s,
        fast::djb2::hash(s.as_bytes())
    );

    println!(
        "sdbm hash of \"{}\" string is: {}",
        s,
        fast::sdbm::hash(s.as_bytes())
    );

    println!(
        "md5 hash of \"{}\" string is: {}",
        s,
        md5::hash(s.as_bytes())
    );

    println!(
        "sha-1 hash of \"{}\" string is: {}",
        s,
        sha1::hash(s.as_bytes())
    );

    println!(
        "sha-2-256 hash of \"{}\" string is: {}",
        s,
        sha2::sha256::hash(s.as_bytes())
    );

    println!(
        "sha-3-512 hash of \"{}\" string is: {}",
        s,
        sha3::sha512::hash(s.as_bytes())
    );
}
