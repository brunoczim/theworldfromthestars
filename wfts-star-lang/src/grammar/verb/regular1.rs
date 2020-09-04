use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        grammemes::{
            ImperativeTense as Imp,
            IndicativeTense as Ind,
            Person,
            Tense,
        },
        verb,
    },
    phonology::{self, Parse, Phoneme, Syllable},
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
            class: "Regular Class 1".to_owned(),
            id: self.id,
            inflections: {
                let mut map = HashMap::new();
                for &person in Person::ALL {
                    for &tense in Tense::ALL {
                        map.insert(
                            format!("{} {}", person, tense),
                            self.word.inflect(person, tense).phonemes.into(),
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
#[error(
    "Invalid 1st-person indicative present {fst_ind_pres:?} for verb regular \
     class 1"
)]
pub struct Invalid {
    pub fst_ind_pres: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    nucleus: Option<Phoneme>,
    suffix: Option<Syllable>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        if let Some(nucleus) = self.nucleus {
            write!(fmt, "{}", nucleus)?;
        } else {
            write!(fmt, "{}", Phoneme::A)?;
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
    fst_ind_pres: phonology::Word,
}

impl Word {
    pub fn new(fst_ind_pres: phonology::Word) -> anyhow::Result<Self> {
        if fst_ind_pres.phonemes().next_back() != Some(Phoneme::A) {
            Err(Invalid { fst_ind_pres })?
        } else {
            Ok(Self { fst_ind_pres })
        }
    }

    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        component::person_tense_table(|person, tense| {
            let inflected = self.inflect(person, tense).phonemes.to_text();
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

    pub fn affix(person: Person, tense: Tense) -> Affix {
        use Person::*;
        use Tense::*;

        let nucleus = match person {
            First => None,
            Second => Some(Phoneme::Ee),
            Third => Some(Phoneme::Ii),
        };

        let suffix = match tense {
            Indicative(Ind::Present) => None,
            Indicative(Ind::Past) => {
                Some(Syllable::parse(&[Phoneme::B, Phoneme::I]).unwrap())
            },
            Indicative(Ind::NearFuture) => {
                Some(Syllable::parse(&[Phoneme::K, Phoneme::E]).unwrap())
            },
            Indicative(Ind::FarFuture) => Some(
                Syllable::parse(&[Phoneme::P, Phoneme::K, Phoneme::E]).unwrap(),
            ),
            Imperative(Imp::Present) => {
                Some(Syllable::parse(&[Phoneme::F, Phoneme::A]).unwrap())
            },
            Imperative(Imp::Future) => {
                Some(Syllable::parse(&[Phoneme::X, Phoneme::A]).unwrap())
            },
        };

        Affix { nucleus, suffix }
    }

    pub fn inflect(&self, person: Person, tense: Tense) -> verb::Inflected {
        let affix = Self::affix(person, tense);

        let mut phonemes = self.fst_ind_pres.clone();

        if let Some(nucleus) = affix.nucleus {
            phonemes = phonemes.replace_final_nucleus(nucleus).unwrap();
        }

        if let Some(suffix) = affix.suffix {
            phonemes = phonemes.append(suffix).unwrap();
        }

        verb::Inflected { person, tense, phonemes }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Regular Class 1",
            entries: component::person_tense_table(|person, tense| {
                let affix = Self::affix(person, tense);
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
        id: Id::new("to-walk").unwrap(),
        meanings: vec![Meaning::ToWalk],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("wiya").unwrap()).unwrap(),
    }]
}
