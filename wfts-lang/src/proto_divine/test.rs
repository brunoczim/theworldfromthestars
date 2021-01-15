use super::phonology::{
    pronounce_words,
    Composite,
    Obstruent,
    Onset,
    Root,
    Sonorant,
    Vowel,
};
use crate::phonetics::{Phone, Variation};

#[test]
fn narrow_pronunc() {
    let word = Composite {
        head: Root {
            onset: Onset {
                outer: Some(Obstruent::K),
                inner: Some(Sonorant::W),
            },
            nucleus: Vowel::Ae,
            coda: Some(Sonorant::Ng.into()),
        },
        tail: Vec::new(),
    };

    let mut expected = Variation::default();
    expected.add_phones(&[Phone::Stress]);
    expected.add_phones(&[Phone::K]);
    expected.add_phones(&[Phone::W, Phone::Bw, Phone::Vw]);
    expected.add_phones(&[Phone::Ae, Phone::A]);
    expected.add_phones(&[Phone::Nj]);

    assert_eq!(expected, word.narrow_pronunc());

    let word = Composite {
        head: Root {
            onset: Onset { outer: None, inner: Some(Sonorant::M) },
            nucleus: Vowel::O,
            coda: Some(Obstruent::F.into()),
        },
        tail: vec![
            Root {
                onset: Onset { outer: None, inner: Some(Sonorant::J) },
                nucleus: Vowel::I,
                coda: None,
            },
            Root {
                onset: Onset {
                    outer: Some(Obstruent::H),
                    inner: Some(Sonorant::J),
                },
                nucleus: Vowel::U,
                coda: None,
            },
        ],
    };

    let mut expected = Variation::default();
    expected.add_phones(&[Phone::Stress]);
    expected.add_phones(&[Phone::M]);
    expected.add_phones(&[Phone::O, Phone::OMid]);
    expected.add_phones(&[Phone::V, Phone::Bh]);
    expected.add_phones(&[Phone::SylBreak]);
    expected.add_phones(&[Phone::J]);
    expected.add_phones(&[Phone::IMidCent]);
    expected.add_phones(&[Phone::SecStress]);
    expected.add_phones(&[Phone::Jh]);
    expected.add_phones(&[Phone::J]);
    expected.add_phones(&[Phone::U]);

    assert_eq!(expected, word.narrow_pronunc());

    let words = [
        Composite {
            head: Root {
                onset: Onset { outer: None, inner: Some(Sonorant::M) },
                nucleus: Vowel::O,
                coda: Some(Obstruent::F.into()),
            },
            tail: vec![
                Root {
                    onset: Onset { outer: None, inner: Some(Sonorant::J) },
                    nucleus: Vowel::I,
                    coda: None,
                },
                Root {
                    onset: Onset {
                        outer: Some(Obstruent::H),
                        inner: Some(Sonorant::J),
                    },
                    nucleus: Vowel::U,
                    coda: None,
                },
            ],
        },
        Composite {
            head: Root {
                onset: Onset {
                    outer: Some(Obstruent::K),
                    inner: Some(Sonorant::W),
                },
                nucleus: Vowel::Ae,
                coda: Some(Sonorant::Ng.into()),
            },
            tail: Vec::new(),
        },
        Composite {
            head: Root {
                onset: Onset { outer: None, inner: Some(Sonorant::M) },
                nucleus: Vowel::E,
                coda: Some(Obstruent::P.into()),
            },
            tail: vec![Root {
                onset: Onset { outer: None, inner: Some(Sonorant::W) },
                nucleus: Vowel::U,
                coda: Some(Obstruent::S.into()),
            }],
        },
    ];

    let mut expected = Variation::default();
    expected.add_phones(&[Phone::Stress]);
    expected.add_phones(&[Phone::M]);
    expected.add_phones(&[Phone::O, Phone::OMid]);
    expected.add_phones(&[Phone::V, Phone::Bh]);
    expected.add_phones(&[Phone::SylBreak]);
    expected.add_phones(&[Phone::J]);
    expected.add_phones(&[Phone::IMidCent]);
    expected.add_phones(&[Phone::SecStress]);
    expected.add_phones(&[Phone::Jh]);
    expected.add_phones(&[Phone::J]);
    expected.add_phones(&[Phone::U]);
    expected.add_phones(&[Phone::Stress]);
    expected.add_phones(&[Phone::G]);
    expected.add_phones(&[Phone::W, Phone::Bw, Phone::Vw]);
    expected.add_phones(&[Phone::Ae, Phone::A]);
    expected.add_phones(&[Phone::Nj]);
    expected.add_phones(&[Phone::Stress]);
    expected.add_phones(&[Phone::M]);
    expected.add_phones(&[Phone::E, Phone::EMid]);
    expected.add_phones(&[Phone::B]);
    expected.add_phones(&[Phone::SylBreak]);
    expected.add_phones(&[Phone::W, Phone::Bw, Phone::Vw]);
    expected.add_phones(&[Phone::UMidCent]);
    expected.add_phones(&[Phone::S]);

    assert_eq!(expected, pronounce_words(&words));
}
