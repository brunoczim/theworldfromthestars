use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        conjunction,
        grammemes::{BasicCase, Case, ClauseCase},
    },
    phonology::{self, Coda, Parse, Phoneme},
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
            class: "Additive Class".to_owned(),
            id: self.id,
            inflections: {
                let mut map = IndexMap::new();
                for &case in ClauseCase::ALL {
                    map.insert(
                        format!("{}", case),
                        self.word.inflect(case).phonemes.into(),
                    );
                }
                map
            },
            meanings: self.meanings,
            notes: self.notes,
        }
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid coordinative {coord:?} for conjunction additive class")]
pub struct Invalid {
    pub coord: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub nucleus: Option<Phoneme>,
    pub coda: Option<Coda>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        let mut needs_dash = false;
        if let Some(nucleus) = self.nucleus {
            write!(fmt, "{}", nucleus)?;
            needs_dash = true;
        }
        if let Some(coda) = self.coda {
            for ph in coda.phonemes() {
                write!(fmt, "{}", ph)?;
            }
        } else if needs_dash {
            write!(fmt, "-")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    coord: phonology::Word,
}

impl Word {
    pub fn new(coord: phonology::Word) -> Self {
        Self { coord }
    }

    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        component::ccase_table(|case| {
            let inflected = self.inflect(case).phonemes.to_text();
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

    pub fn affix(case: ClauseCase) -> Affix {
        use BasicCase::*;
        use Case::*;
        use ClauseCase::*;

        let (nucleus, coda) = match case {
            Coordinative => (None, None),
            Subordinative(Basic(Nominative)) => {
                (Some(Phoneme::I), Some(Coda::parse(&[Phoneme::Rr]).unwrap()))
            },
            Subordinative(Basic(Accusative)) => {
                (Some(Phoneme::E), Some(Coda::parse(&[Phoneme::M]).unwrap()))
            },
            Subordinative(Basic(Topical)) => {
                (Some(Phoneme::I), Some(Coda::parse(&[Phoneme::F]).unwrap()))
            },
            Subordinative(Basic(Postpositional)) => (Some(Phoneme::Ee), None),
            Subordinative(Passive) => (Some(Phoneme::Aa), None),
        };

        Affix { nucleus, coda }
    }

    pub fn inflect(&self, case: ClauseCase) -> conjunction::Inflected {
        let affix = Self::affix(case);

        let phonemes = match (affix.nucleus, affix.coda) {
            (Some(nucleus), Some(coda)) => {
                self.coord.replace_final_rhyme(nucleus, coda).unwrap()
            },
            (Some(nucleus), None) => {
                self.coord.replace_final_nucleus(nucleus).unwrap()
            },
            (None, Some(coda)) => self.coord.replace_final_coda(coda).unwrap(),
            (None, None) => self.coord.clone(),
        };

        conjunction::Inflected { phonemes, case }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Additive Class",
            entries: component::ccase_table(|case| {
                let affix = Self::affix(case);
                UnmarkedList(vec![
                    WithStarAlphabet(affix.to_string()).to_dyn(),
                    affix.to_string().to_dyn(),
                ])
                .to_dyn()
            }),
        }
    }
}

pub fn definitions() -> Vec<Definition> {
    vec![Definition {
        id: Id::new("and").unwrap(),
        meanings: vec![Meaning::And],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("eta").unwrap()),
    }]
}
