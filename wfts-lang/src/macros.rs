#[macro_export]
macro_rules! proto_div_word {
    [$($($phonemes:expr),*);*] => {
        $crate::proto_divine::phonology::Word::parse(
            &[$(&[$($phonemes.into(),)*],)*]
        ).unwrap()
    };
}
