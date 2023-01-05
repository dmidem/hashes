macro_rules! define_hash_tests {
    ($small_data_results:expr, $big_data_result:expr) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_small_data() {
                assert_eq!(hash("".as_bytes()).into_bytes(), $small_data_results[0]);

                assert_eq!(hash("abc".as_bytes()).into_bytes(), $small_data_results[1]);

                assert_eq!(
                    hash("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq".as_bytes())
                        .into_bytes(),
                    $small_data_results[2]
                );

                assert_eq!(
                    hash(
                        "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
                    hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
                            .as_bytes()
                    )
                    .into_bytes(),
                    $small_data_results[3]
                );

                assert_eq!(
                    hash(&vec![0x61u8; 1000000]).into_bytes(),
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
                    .into_bytes(),
                    $big_data_result
                );
            }
        }
    };
}

pub(crate) use define_hash_tests;
