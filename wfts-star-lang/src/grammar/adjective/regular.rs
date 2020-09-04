use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        adjective,
        grammemes::{BasicCase, Gender, Number},
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
            class: "Regular Class".to_owned(),
            id: self.id,
            inflections: {
                let mut map = IndexMap::new();
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
    "Invalid nominative divine singular {nom_div_sing:?} for adjective \
     regular class"
)]
pub struct Invalid {
    pub nom_div_sing: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub nucleus: Phoneme,
    pub coda: Coda,
    pub suffix: Option<Syllable>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-{}", self.nucleus.to_text())?;
        for ph in self.coda.phonemes() {
            write!(fmt, "{}", ph.to_text())?;
        }
        if let Some(suffix) = self.suffix {
            for ph in suffix.phonemes() {
                write!(fmt, "{}", ph.to_text())?;
            }
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
        let last = nom_div_sing.phonemes().next_back().unwrap();
        match last {
            Phoneme::Ii => Ok(Self { nom_div_sing }),
            _ => Err(Invalid { nom_div_sing })?,
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

        let nominative = Coda::parse(&[]).unwrap();
        let accusative = Coda::parse(&[Phoneme::N]).unwrap();
        let accusative2 = Coda::parse(&[Phoneme::Mg]).unwrap();
        let accusative3 = Coda::parse(&[Phoneme::W]).unwrap();
        let topical = Coda::parse(&[Phoneme::F]).unwrap();
        let topical2 = Coda::parse(&[Phoneme::W]).unwrap();
        let topical3 = Coda::parse(&[Phoneme::Y]).unwrap();
        let postpositional = Coda::parse(&[Phoneme::S]).unwrap();
        let postpositional2 = Coda::parse(&[Phoneme::W]).unwrap();
        let divine = Phoneme::Ii;
        let animate = Phoneme::Aa;
        let animate2 = Phoneme::Ee;
        let inanimate = Phoneme::I;

        let plural = Syllable::parse(&[Phoneme::Rr, Phoneme::Ee]).unwrap();
        let nullar = Syllable::parse(&[Phoneme::M, Phoneme::Ee]).unwrap();
        let collective = Syllable::parse(&[Phoneme::A, Phoneme::X]).unwrap();
        let collective2 = Syllable::parse(&[Phoneme::X, Phoneme::I]).unwrap();

        let coda = match (case, gender, number) {
            (Nominative, ..) => nominative,
            (Accusative, _, Nullar) => accusative2,
            (Accusative, _, Collective) => accusative3,
            (Accusative, ..) => accusative,
            (Topical, Animate, _) => topical2,
            (Topical, Inanimate, Nullar) => topical3,
            (Topical, ..) => topical,
            (Postpositional, Animate, _) => postpositional2,
            (Postpositional, ..) => postpositional,
        };

        let nucleus = match (case, gender, number) {
            (_, Divine, _) => divine,
            (Accusative, Animate, Nullar)
            | (Accusative, Animate, Collective) => animate2,
            (_, Animate, _) => animate,
            (_, Inanimate, _) => inanimate,
        };

        let suffix = match (case, number) {
            (_, Singular) => None,
            (_, Plural) => Some(plural),
            (_, Nullar) => Some(nullar),
            (Postpositional, Collective) => Some(collective2),
            (_, Collective) => Some(collective),
        };

        Affix { nucleus, coda, suffix }
    }

    pub fn inflect(
        &self,
        case: BasicCase,
        gender: Gender,
        number: Number,
    ) -> adjective::Inflected {
        let affix = Self::affix(case, gender, number);
        let mut phonemes = self
            .nom_div_sing
            .replace_final_rhyme(affix.nucleus, affix.coda)
            .unwrap();
        if let Some(suffix) = affix.suffix {
            phonemes = phonemes.append(suffix).unwrap();
        }

        adjective::Inflected { phonemes, case, gender, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Regular Class",
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
        id: Id::new("big").unwrap(),
        meanings: vec![Meaning::Big],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("mací").unwrap()).unwrap(),
    }]
}
