#[cfg(doctest)]
mod test_readme {
    macro_rules! doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    doc_test!(include_str!("../README.md"));
}
