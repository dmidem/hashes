macro_rules! define_digest_convert_tests {
    ($words:expr, $bytes:expr) => {
        #[test]
        fn test_digest_convert() {
            let words = $words;
            let bytes = $bytes;

            assert_eq!(Digest::from_words(words).into_bytes(), bytes);
            assert_eq!(Digest::from_bytes(bytes).into_words(), words);

            assert_eq!(
                Digest::from_words(words)
                    .to_string()
                    .parse::<Digest>()
                    .unwrap()
                    .into_words(),
                words
            );
        }
    };
}

pub(super) use define_digest_convert_tests;

macro_rules! define_hash_tests {
    ($small_data_results:expr, $big_data_result:expr) => {
        #[test]
        fn test_small_data() {
            assert_eq!(hash("".as_bytes()).into_words(), $small_data_results[0]);

            assert_eq!(hash("abc".as_bytes()).into_words(), $small_data_results[1]);

            assert_eq!(
                hash("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_bytes())
                    .into_words(),
                $small_data_results[2]
            );

            assert_eq!(
                hash(
                    "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
                    hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
                        .as_bytes()
                )
                .into_words(),
                $small_data_results[3]
            );

            assert_eq!(
                hash(&vec![0x61u8; 1000000]).into_words(),
                $small_data_results[4]
            );
        }

        #[test]
        #[cfg(feature = "test_big_data")]
        fn test_big_data() {
            assert_eq!(
                hash(
                    "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmno"
                        .repeat(16_777_216)
                        .as_bytes()
                )
                .into_words(),
                $big_data_result
            );
        }
    };
}

pub(super) use define_hash_tests;
