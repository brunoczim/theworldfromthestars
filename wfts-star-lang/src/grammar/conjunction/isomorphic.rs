use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        conjunction,
        grammemes::{BasicCase, Case, ClauseCase},
    },
    phonology::{self, Coda, Onset, Parse, Phoneme, Syllable},
    StarLang,
};
use std::{collections::HashMap, fmt};
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
                let mut map = HashMap::new();
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
#[error("Invalid nominative {nom:?} for conjunction isomorphic class")]
pub struct Invalid {
    pub nom: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub before: Option<Phoneme>,
    pub after: Option<Phoneme>,
    pub coda_outer: Option<Phoneme>,
}
impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        if let Some(nucleus) = self.before {
            write!(fmt, "{}", nucleus)?;
        }
        write!(fmt, "{}", Phoneme::R)?;
        if let Some(nucleus) = self.after {
            write!(fmt, "{}", nucleus)?;
        }
        if let Some(coda_outer) = self.coda_outer {
            write!(fmt, "{}", coda_outer)?;
        } else {
            write!(fmt, "-")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    nom: phonology::Word,
}

impl Word {
    pub fn new(nom: phonology::Word) -> anyhow::Result<Self> {
        let (&last, init) = nom.syllables().split_last().unwrap();

        let last_but_one = init.last().map(|syl| syl.nucleus());
        if last.nucleus() == Phoneme::R && last_but_one != Some(Phoneme::A) {
            Ok(Self { nom })
        } else {
            Err(Invalid { nom })?
        }
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

        let (before, after, coda_outer) = match case {
            Subordinative(Basic(Nominative))
            | Subordinative(Basic(Postpositional)) => (None, None, None),
            Subordinative(Basic(Accusative)) => (None, Some(Phoneme::E), None),
            Subordinative(Basic(Topical)) => (Some(Phoneme::A), None, None),
            Subordinative(Passive) => {
                (None, Some(Phoneme::A), Some(Phoneme::F))
            },
            Coordinative => (Some(Phoneme::A), None, Some(Phoneme::S)),
        };

        Affix { before, after, coda_outer }
    }

    pub fn inflect(&self, case: ClauseCase) -> conjunction::Inflected {
        let affix = Self::affix(case);

        let phonemes = match (affix.before, affix.after) {
            (None, None) => self.nom.clone(),
            (Some(nucleus), None) => {
                let mut syllables = self.nom.syllables().to_vec();
                let last = syllables.last_mut().unwrap();
                let coda =
                    Coda::new(Some(Phoneme::R), last.coda().outer()).unwrap();
                *last = Syllable::new(last.onset(), nucleus, coda).unwrap();

                phonology::Word::new(syllables).unwrap()
            },
            (None, Some(nucleus)) => {
                let mut syllables = self.nom.syllables().to_vec();
                let last = syllables.last_mut().unwrap();
                let onset = Onset::new(
                    last.onset().outer(),
                    last.onset().medial(),
                    Some(Phoneme::R),
                );
                *last = Syllable::new(onset.unwrap(), nucleus, last.coda())
                    .unwrap();
                phonology::Word::new(syllables).unwrap()
            },

            (Some(before), Some(after)) => {
                let mut syllables = self.nom.syllables().to_vec();
                let last = syllables.pop().unwrap();
                let last_but_one = Syllable::new(
                    last.onset(),
                    before,
                    Coda::parse(&[]).unwrap(),
                );
                let new_last = Syllable::new(
                    Onset::parse(&[Phoneme::R]).unwrap(),
                    after,
                    last.coda(),
                );
                syllables.push(last_but_one.unwrap());
                syllables.push(new_last.unwrap());
                phonology::Word::new(syllables).unwrap()
            },
        };

        conjunction::Inflected { phonemes, case }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Isomorphic Class",
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
        id: Id::new("equals").unwrap(),
        meanings: vec![Meaning::CompoundConj],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("r").unwrap()).unwrap(),
    }]
}
