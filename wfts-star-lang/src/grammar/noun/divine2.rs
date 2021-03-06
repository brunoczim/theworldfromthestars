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
use indexmap::IndexMap;
use std::fmt;
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
            class: "Divine Class 2".to_owned(),
            id: self.id,
            inflections: {
                let mut map = IndexMap::new();
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
#[error("Invalid nominative singular {nom_sing:?} for noun divine class 2")]
pub struct Invalid {
    pub nom_sing: phonology::Word,
}

#[derive(Debug, Clone, Copy)]
pub enum Affix {
    Coda(Coda),
    Suffix(Syllable),
    Empty,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        match self {
            Affix::Coda(coda) => {
                for ph in coda.phonemes() {
                    write!(fmt, "{}", ph)?;
                }
            },
            Affix::Suffix(suffix) => {
                for ph in suffix.phonemes() {
                    write!(fmt, "{}", ph)?;
                }
            },
            Affix::Empty => (),
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
        let last = nom_sing.phonemes().next_back();
        match last {
            Some(Phoneme::N) | Some(Phoneme::M) | Some(Phoneme::X)
            | Some(Phoneme::Mg) | Some(Phoneme::Xw) => {
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

        match (case, number) {
            (Nominative, Singular) => Affix::Empty,
            (Accusative, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::N]).unwrap())
            },
            (Topical, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::M]).unwrap())
            },
            (Postpositional, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::X]).unwrap())
            },
            (Nominative, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::H]).unwrap())
            },
            (Accusative, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Rr, Phoneme::N]).unwrap())
            },
            (Topical, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Rr, Phoneme::M]).unwrap())
            },
            (Postpositional, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::H]).unwrap())
            },
            (Nominative, Nullar) => {
                Affix::Coda(Coda::parse(&[Phoneme::M]).unwrap())
            },
            (Accusative, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::N, Phoneme::Ee, Phoneme::M])
                    .unwrap(),
            ),
            (Topical, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::M, Phoneme::Ee, Phoneme::M])
                    .unwrap(),
            ),
            (Postpositional, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::X, Phoneme::Ee, Phoneme::M])
                    .unwrap(),
            ),
            (Nominative, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::X]).unwrap())
            },
            (Accusative, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::N]).unwrap())
            },
            (Topical, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::M]).unwrap())
            },
            (Postpositional, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::X]).unwrap())
            },
        }
    }

    pub fn inflect(&self, case: BasicCase, number: Number) -> noun::Inflected {
        let affix = Self::affix(case, number);
        let phonemes = match affix {
            Affix::Coda(coda) => {
                self.nom_sing.replace_final_coda(coda).unwrap()
            },
            Affix::Suffix(suffix) => self.nom_sing.append(suffix).unwrap(),
            Affix::Empty => self.nom_sing.clone(),
        };

        noun::Inflected { phonemes, gender: Gender::Divine, case, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Divine Class 2",
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
    vec![Definition {
        id: Id::new("wind").unwrap(),
        meanings: vec![Meaning::Wind],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("fwi").unwrap()).unwrap(),
    }]
}
