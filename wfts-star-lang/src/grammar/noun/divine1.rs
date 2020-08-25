use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        grammemes::{BasicCase, Gender, Number},
        noun,
    },
    phonology::{self, Coda, Parse, Phoneme, Syllable},
    StarLang,
};
use std::{
    collections::HashMap,
    fmt::{self},
};
use thiserror::Error;
use wfts_lang::{semantics::Meaning, Lang};
use wfts_pedia_ssg::{
    component::{
        list::UnmarkedList,
        table::{self, Table},
        text::Link,
        Component,
        DynComponent,
    },
    location::{Id, Location},
};

#[derive(Debug, Clone)]
pub struct Definition {
    pub id: Id,
    pub word: Word,
    pub meanings: Vec<Meaning>,
    pub notes: DynComponent,
}

impl Definition {
    pub fn to_dict_entry(self) -> dictionary::Entry {
        dictionary::Entry {
            inflection_table: self.word.table(&self.id),
            class: "Divine Class 1".to_owned(),
            id: self.id,
            inflections: {
                let mut map = HashMap::new();
                for &case in BasicCase::ALL {
                    for &number in Number::ALL {
                        map.insert(
                            format!("{} {} {}", case, Gender::Divine, number),
                            self.word.inflect(case, number).phonemes.into(),
                        );
                    }
                }
                map
            },
            meanings: self.meanings,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid nominative singular {nom_sing:?} for noun divine class 1")]
pub struct Invalid {
    pub nom_sing: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub coda: Option<Coda>,
    pub suffix: Option<Syllable>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        if let Some(coda) = self.coda {
            for ph in coda.phonemes() {
                write!(fmt, "{}", ph)?;
            }
        }
        if let Some(suffix) = self.suffix {
            for ph in suffix.phonemes() {
                write!(fmt, "{}", ph)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    nom_sing: phonology::Word,
}

impl Word {
    pub fn new(nom_sing: phonology::Word) -> anyhow::Result<Self> {
        let last = nom_sing.syllables().last().unwrap();
        let no_coda = last.coda().phonemes().next().is_none();
        match last.nucleus() {
            Phoneme::E | Phoneme::Ee | Phoneme::I if no_coda => {
                Err(Invalid { nom_sing })?
            },
            _ => Ok(Self { nom_sing }),
        }
    }

    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        component::bcase_fgender_number_table(Gender::Divine, |case, number| {
            let inflected = self.inflect(case, number).phonemes.to_text();
            let link = Link {
                location: Location::internal(format!(
                    "{}/dictionary/{}#{}",
                    StarLang.path(),
                    inflected,
                    entry_id,
                )),
                text: WithStarAlphabet(inflected.clone()),
            };
            let component =
                UnmarkedList(vec![link.to_dyn(), inflected.to_dyn()]);
            component.blocking().to_dyn()
        })
    }

    pub fn affix(case: BasicCase, number: Number) -> Affix {
        use BasicCase::*;
        use Number::*;

        let accusative = Coda::parse(&[Phoneme::N]).unwrap();
        let accusative2 = Coda::parse(&[Phoneme::Nj]).unwrap();
        let accusative3 = Coda::parse(&[Phoneme::Y]).unwrap();
        let topical = Coda::parse(&[Phoneme::F]).unwrap();
        let postpositional = Coda::parse(&[Phoneme::Y, Phoneme::S]).unwrap();

        let plural = Syllable::parse(&[Phoneme::Ee]).unwrap();
        let nullar = Syllable::parse(&[Phoneme::E, Phoneme::N]).unwrap();
        let collective = Syllable::parse(&[Phoneme::I, Phoneme::Xw]).unwrap();
        let collective2 = Syllable::parse(&[Phoneme::I, Phoneme::W]).unwrap();

        let coda = match (case, number) {
            (Nominative, _) => None,
            (Accusative, Nullar) => Some(accusative2),
            (Accusative, Collective) => Some(accusative3),
            (Accusative, _) => Some(accusative),
            (Topical, _) => Some(topical),
            (Postpositional, _) => Some(postpositional),
        };

        let suffix = match (case, number) {
            (_, Singular) => None,
            (_, Plural) => Some(plural),
            (_, Nullar) => Some(nullar),
            (Postpositional, Collective) => Some(collective2),
            (_, Collective) => Some(collective),
        };

        Affix { coda, suffix }
    }

    pub fn inflect(&self, case: BasicCase, number: Number) -> noun::Inflected {
        let affix = Self::affix(case, number);
        let mut phonemes = match affix.coda {
            Some(coda) => self.nom_sing.replace_final_coda(coda).unwrap(),
            None => self.nom_sing.clone(),
        };
        if let Some(suffix) = affix.suffix {
            phonemes = phonemes.append(suffix).unwrap();
        }

        noun::Inflected { phonemes, gender: Gender::Divine, case, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Divine Class 1",
            entries: component::bcase_fgender_number_table(
                Gender::Divine,
                |case, number| {
                    let affix = Self::affix(case, number);
                    UnmarkedList(vec![
                        WithStarAlphabet(affix.to_string()).to_dyn(),
                        affix.to_string().to_dyn(),
                    ])
                    .to_dyn()
                },
            ),
        }
    }
}

pub fn definitions() -> Vec<Definition> {
    vec![
        Definition {
            id: Id::new("star").unwrap(),
            meanings: vec![Meaning::Star],
            notes: "".blocking().to_dyn(),
            word: Word::new(phonology::Word::parse_str("saŋ").unwrap())
                .unwrap(),
        },
        Definition {
            id: Id::new("fire").unwrap(),
            meanings: vec![Meaning::Fire],
            notes: "".blocking().to_dyn(),
            word: Word::new(phonology::Word::parse_str("kef").unwrap())
                .unwrap(),
        },
    ]
}
