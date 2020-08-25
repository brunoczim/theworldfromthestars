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
            class: "Full-Inflection Class 2".to_owned(),
            id: self.id,
            inflections: {
                let mut map = HashMap::new();
                for &case in BasicCase::ALL {
                    for &gender in Gender::ALL {
                        for &number in Number::ALL {
                            map.insert(
                                format!("{} {} {}", case, gender, number),
                                self.word
                                    .inflect(case, gender, number)
                                    .phonemes
                                    .into(),
                            );
                        }
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
#[error(
    "Invalid nominative divine singular {nom_div_sing:?} for noun \
     full-inflection class 1"
)]
pub struct Invalid {
    pub nom_div_sing: phonology::Word,
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
    nom_div_sing: phonology::Word,
}

impl Word {
    pub fn new(nom_div_sing: phonology::Word) -> anyhow::Result<Self> {
        let last = nom_div_sing.phonemes().next_back();
        match last {
            Some(Phoneme::N) | Some(Phoneme::M) | Some(Phoneme::X)
            | Some(Phoneme::Mg) | Some(Phoneme::Xw) => {
                Err(Invalid { nom_div_sing })?
            },
            _ => Ok(Self { nom_div_sing }),
        }
    }

    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        component::bcase_gender_number_table(|case, gender, number| {
            let inflected =
                self.inflect(case, gender, number).phonemes.to_text();
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

    pub fn affix(case: BasicCase, gender: Gender, number: Number) -> Affix {
        use BasicCase::*;
        use Gender::*;
        use Number::*;

        match (case, gender, number) {
            (Nominative, Divine, Singular) => Affix::Empty,
            (Accusative, Divine, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::N]).unwrap())
            },
            (Topical, Divine, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::M]).unwrap())
            },
            (Postpositional, Divine, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::X]).unwrap())
            },
            (Nominative, Animate, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::Y]).unwrap())
            },
            (Accusative, Animate, Singular) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::I]).unwrap(),
            ),
            (Topical, Animate, Singular) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::I]).unwrap(),
            ),
            (Postpositional, Animate, Singular) => Affix::Suffix(
                Syllable::parse(&[Phoneme::X, Phoneme::I]).unwrap(),
            ),
            (Nominative, Inanimate, Singular) => {
                Affix::Coda(Coda::parse(&[Phoneme::W]).unwrap())
            },
            (Accusative, Inanimate, Singular) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I]).unwrap(),
            ),
            (Topical, Inanimate, Singular) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I]).unwrap(),
            ),
            (Postpositional, Inanimate, Singular) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Xw, Phoneme::I]).unwrap(),
            ),
            (Nominative, Divine, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Rr]).unwrap())
            },
            (Accusative, Divine, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Rr, Phoneme::N]).unwrap())
            },
            (Topical, Divine, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Rr, Phoneme::M]).unwrap())
            },
            (Postpositional, Divine, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Rr, Phoneme::X]).unwrap())
            },
            (Nominative, Animate, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::Y, Phoneme::H]).unwrap())
            },
            (Accusative, Animate, Plural) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::I, Phoneme::Rr])
                    .unwrap(),
            ),
            (Topical, Animate, Plural) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::Rr, Phoneme::I])
                    .unwrap(),
            ),
            (Postpositional, Animate, Plural) => Affix::Suffix(
                Syllable::parse(&[Phoneme::X, Phoneme::I, Phoneme::Rr])
                    .unwrap(),
            ),
            (Nominative, Inanimate, Plural) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::H]).unwrap())
            },
            (Accusative, Inanimate, Plural) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I, Phoneme::Rr])
                    .unwrap(),
            ),
            (Topical, Inanimate, Plural) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::Rr, Phoneme::I])
                    .unwrap(),
            ),
            (Postpositional, Inanimate, Plural) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Xw, Phoneme::I]).unwrap(),
            ),
            (Nominative, Divine, Nullar) => {
                Affix::Coda(Coda::parse(&[Phoneme::M]).unwrap())
            },
            (Accusative, Divine, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::N, Phoneme::Ee, Phoneme::M])
                    .unwrap(),
            ),
            (Topical, Divine, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::M, Phoneme::Ee, Phoneme::M])
                    .unwrap(),
            ),
            (Postpositional, Divine, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::X, Phoneme::Ee, Phoneme::M])
                    .unwrap(),
            ),
            (Nominative, Animate, Nullar) => {
                Affix::Coda(Coda::parse(&[Phoneme::Y, Phoneme::M]).unwrap())
            },
            (Accusative, Animate, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::I, Phoneme::M])
                    .unwrap(),
            ),
            (Topical, Animate, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::I, Phoneme::M])
                    .unwrap(),
            ),
            (Postpositional, Animate, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::X, Phoneme::I, Phoneme::M]).unwrap(),
            ),
            (Nominative, Inanimate, Nullar) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::M]).unwrap())
            },
            (Accusative, Inanimate, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I, Phoneme::M])
                    .unwrap(),
            ),
            (Topical, Inanimate, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I, Phoneme::M])
                    .unwrap(),
            ),
            (Postpositional, Inanimate, Nullar) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Xw, Phoneme::I, Phoneme::M])
                    .unwrap(),
            ),
            (Nominative, Divine, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::X]).unwrap())
            },
            (Accusative, Divine, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::N]).unwrap())
            },
            (Topical, Divine, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::M]).unwrap())
            },
            (Postpositional, Divine, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::X]).unwrap())
            },
            (Nominative, Animate, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::Y, Phoneme::X]).unwrap())
            },
            (Accusative, Animate, Collective) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::I, Phoneme::X])
                    .unwrap(),
            ),
            (Topical, Animate, Collective) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Nj, Phoneme::W, Phoneme::I])
                    .unwrap(),
            ),
            (Postpositional, Animate, Collective) => Affix::Suffix(
                Syllable::parse(&[Phoneme::X, Phoneme::I, Phoneme::X]).unwrap(),
            ),
            (Nominative, Inanimate, Collective) => {
                Affix::Coda(Coda::parse(&[Phoneme::W, Phoneme::X]).unwrap())
            },
            (Accusative, Inanimate, Collective) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I, Phoneme::X])
                    .unwrap(),
            ),
            (Topical, Inanimate, Collective) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Ng, Phoneme::I]).unwrap(),
            ),
            (Postpositional, Inanimate, Collective) => Affix::Suffix(
                Syllable::parse(&[Phoneme::Xw, Phoneme::I, Phoneme::X])
                    .unwrap(),
            ),
        }
    }

    pub fn inflect(
        &self,
        case: BasicCase,
        gender: Gender,
        number: Number,
    ) -> noun::Inflected {
        let affix = Self::affix(case, gender, number);
        let phonemes = match affix {
            Affix::Coda(coda) => {
                self.nom_div_sing.replace_final_coda(coda).unwrap()
            },
            Affix::Suffix(suffix) => self.nom_div_sing.append(suffix).unwrap(),
            Affix::Empty => self.nom_div_sing.clone(),
        };

        noun::Inflected { phonemes, case, gender, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Full-Inflection Class 2",
            entries: component::bcase_gender_number_table(
                |case, gender, number| {
                    let affix = Self::affix(case, gender, number);
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
        id: Id::new("tree").unwrap(),
        meanings: vec![Meaning::Tree],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("dse").unwrap()).unwrap(),
    }]
}
