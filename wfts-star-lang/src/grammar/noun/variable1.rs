use crate::{
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
    "Invalid nominative divine singular {nom_div_sing:?} for noun variable \
     class 1"
)]
pub struct Invalid {
    pub nom_div_sing: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub nucleus: Option<Phoneme>,
    pub coda: Option<Coda>,
    pub suffix: Option<Syllable>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        let mut needs_dash = false;
        if let Some(nucleus) = self.nucleus {
            write!(fmt, "{}", nucleus.to_text())?;
            needs_dash = true;
        }
        if let Some(coda) = self.coda {
            for ph in coda.iter() {
                write!(fmt, "{}", ph.to_text())?;
            }
        } else if needs_dash {
            write!(fmt, "-")?;
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
        let last = nom_div_sing.syllables().last().unwrap();
        let no_coda = last.coda().iter().next().is_none();
        match last.nucleus() {
            Phoneme::E | Phoneme::Ee | Phoneme::I if no_coda => {
                Err(Invalid { nom_div_sing })?
            },
            _ => Ok(Self { nom_div_sing }),
        }
    }

    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        noun::full_inflection_table(|case, gender, number| {
            let inflected =
                self.inflect(case, gender, number).phonemes.to_text();
            let component = Link {
                location: Location::internal(format!(
                    "{}/dictionary/{}#{}",
                    StarLang.path(),
                    inflected,
                    entry_id,
                )),
                text: WithStarAlphabet(inflected),
            };
            component.blocking().to_dyn()
        })
    }

    pub fn affix(case: BasicCase, gender: Gender, number: Number) -> Affix {
        use BasicCase::*;
        use Gender::*;
        use Number::*;

        let accusative = Coda::new(None, Some(Phoneme::N)).unwrap();
        let accusative2 = Coda::new(None, Some(Phoneme::Mg)).unwrap();
        let accusative3 = Coda::new(Some(Phoneme::W), None).unwrap();
        let topical = Coda::new(None, Some(Phoneme::F)).unwrap();
        let topical2 = Coda::new(Some(Phoneme::W), None).unwrap();
        let topical3 = Coda::new(Some(Phoneme::Y), None).unwrap();
        let postpositional =
            Coda::new(Some(Phoneme::Y), Some(Phoneme::S)).unwrap();
        let postpositional2 = Coda::new(Some(Phoneme::Y), None).unwrap();
        let animate = Phoneme::Aa;
        let animate2 = Phoneme::Ee;
        let inanimate = Phoneme::I;

        let plural = Syllable::parse(&[Phoneme::Ee]).unwrap();
        let nullar = Syllable::parse(&[Phoneme::E, Phoneme::N]).unwrap();
        let collective = Syllable::parse(&[Phoneme::I, Phoneme::Xw]).unwrap();
        let collective2 = Syllable::parse(&[Phoneme::I, Phoneme::W]).unwrap();

        let coda = match (case, gender, number) {
            (Nominative, ..) => None,
            (Accusative, _, Nullar) => Some(accusative2),
            (Accusative, _, Collective) => Some(accusative3),
            (Accusative, ..) => Some(accusative),
            (Topical, Animate, _) => Some(topical2),
            (Topical, Inanimate, Nullar) => Some(topical3),
            (Topical, ..) => Some(topical),
            (Postpositional, Animate, _) => Some(postpositional2),
            (Postpositional, ..) => Some(postpositional),
        };

        let nucleus = match (case, gender, number) {
            (_, Divine, _) => None,
            (Accusative, Animate, Nullar)
            | (Accusative, Animate, Collective) => Some(animate2),
            (_, Animate, _) => Some(animate),
            (_, Inanimate, _) => Some(inanimate),
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
    ) -> noun::Inflected {
        let affix = Self::affix(case, gender, number);
        let mut phonemes = match (affix.nucleus, affix.coda) {
            (Some(nucleus), Some(coda)) => {
                self.nom_div_sing.replace_final_rhyme(nucleus, coda).unwrap()
            },
            (Some(nucleus), None) => {
                self.nom_div_sing.replace_final_nucleus(nucleus).unwrap()
            },
            (None, Some(coda)) => {
                self.nom_div_sing.replace_final_coda(coda).unwrap()
            },
            (None, None) => self.nom_div_sing.clone(),
        };
        if let Some(suffix) = affix.suffix {
            phonemes = phonemes.append(suffix).unwrap();
        }

        noun::Inflected { phonemes, case, gender, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Variable Class 1",
            entries: noun::full_inflection_table(|case, gender, number| {
                let affix = Self::affix(case, gender, number);
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
        id: Id::new("eye").unwrap(),
        meanings: vec![Meaning::Eye],
        notes: "".blocking().to_dyn(),
        word: Word::new(phonology::Word::parse_str("gas").unwrap()).unwrap(),
    }]
}
