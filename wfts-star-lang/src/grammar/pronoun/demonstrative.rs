use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        grammemes::{BasicCase, Case, Gender, Number, Person},
        pronoun,
    },
    phonology::{self, Coda, Onset, Parse, Phoneme, Syllable},
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
            class: "Demonstrative Class".to_owned(),
            id: self.id,
            inflections: {
                let mut map = HashMap::new();
                for &person in Person::ALL {
                    for &case in Case::ALL {
                        for &gender in Gender::ALL {
                            for &number in Number::ALL {
                                map.insert(
                                    format!(
                                        "{} {} {} {}",
                                        person, case, gender, number
                                    ),
                                    self.word
                                        .inflect(person, case, gender, number)
                                        .phonemes
                                        .into(),
                                );
                            }
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
    "Invalid nominative divine singular {fst_nom_div_sing:?} for pronoun \
     demonstrative class"
)]
pub struct Invalid {
    pub fst_nom_div_sing: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub prefix: Option<Syllable>,
    pub nucleus: Option<Phoneme>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(prefix) = self.prefix {
            for ph in prefix.phonemes() {
                write!(fmt, "{}", ph)?;
            }
        }
        write!(fmt, "-")?;
        if let Some(nucleus) = self.nucleus {
            write!(fmt, "{}-", nucleus)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    fst_nom_div_sing: phonology::Word,
}

impl Word {
    pub fn new(fst_nom_div_sing: phonology::Word) -> anyhow::Result<Self> {
        let first = fst_nom_div_sing.phonemes().next().unwrap();
        match first {
            Phoneme::A
            | Phoneme::Aa
            | Phoneme::E
            | Phoneme::Ee
            | Phoneme::I
            | Phoneme::Ii
            | Phoneme::Y
            | Phoneme::M
            | Phoneme::W
            | Phoneme::Mg => Err(Invalid { fst_nom_div_sing })?,
            _ => Ok(Self { fst_nom_div_sing }),
        }
    }

    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        component::person_case_gender_number_table(
            |person, case, gender, number| {
                let inflected = self
                    .inflect(person, case, gender, number)
                    .phonemes
                    .to_text();
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
            },
        )
    }

    pub fn affix(
        person: Person,
        case: Case,
        gender: Gender,
        number: Number,
    ) -> Affix {
        use BasicCase::*;
        use Case::*;
        use Gender::*;
        use Number::*;
        use Person::*;

        let nominative = Onset::parse(&[]).unwrap();
        let accusative = Onset::parse(&[Phoneme::T]).unwrap();
        let accusative2 = Onset::parse(&[Phoneme::B]).unwrap();
        let accusative3 = Onset::parse(&[Phoneme::Gw]).unwrap();
        let topical = Onset::parse(&[Phoneme::P]).unwrap();
        let topical2 = Onset::parse(&[Phoneme::Kw]).unwrap();
        let topical3 = Onset::parse(&[Phoneme::C]).unwrap();
        let postpositional = Onset::parse(&[Phoneme::D]).unwrap();
        let postpositional2 = Onset::parse(&[Phoneme::J]).unwrap();
        let passive = Onset::parse(&[Phoneme::K]).unwrap();

        let divine = Phoneme::Ee;
        let divine2 = Phoneme::I;
        let animate = Phoneme::Aa;
        let animate2 = Phoneme::Ee;
        let inanimate = Phoneme::I;

        let singular = Coda::parse(&[]).unwrap();
        let plural = Coda::parse(&[Phoneme::Y]).unwrap();
        let nullar = Coda::parse(&[Phoneme::M]).unwrap();
        let collective = Coda::parse(&[Phoneme::Mg]).unwrap();
        let collective2 = Coda::parse(&[Phoneme::W]).unwrap();

        let first = Phoneme::Aa;
        let first2 = Phoneme::A;
        let second = Phoneme::Ee;
        let third = Phoneme::I;
        let third2 = Phoneme::E;

        let onset = match (case, gender, number) {
            (Basic(Nominative), ..) => nominative,
            (Basic(Accusative), _, Nullar) => accusative2,
            (Basic(Accusative), _, Collective) => accusative3,
            (Basic(Accusative), ..) => accusative,
            (Basic(Topical), Animate, _) => topical2,
            (Basic(Topical), Inanimate, Nullar) => topical3,
            (Basic(Topical), ..) => topical,
            (Basic(Postpositional), Animate, _) => postpositional2,
            (Basic(Postpositional), ..) => postpositional,
            (Passive, ..) => passive,
        };

        let nucleus1 = match (case, gender, number) {
            (Basic(Nominative), Divine, Singular)
            | (Passive, Divine, Singular) => None,
            (_, Divine, Plural) | (Passive, Divine, _) => Some(divine),
            (_, Divine, _) => Some(divine2),
            (Basic(Accusative), Animate, Nullar)
            | (Basic(Accusative), Animate, Collective) => Some(animate2),
            (_, Animate, _) => Some(animate),
            (_, Inanimate, _) => Some(inanimate),
        };

        let coda = match (case, number) {
            (_, Singular) => singular,
            (_, Plural) => plural,
            (_, Nullar) => nullar,
            (Basic(Postpositional), Collective) => collective2,
            (_, Collective) => collective,
        };

        let nucleus2 = match (person, number) {
            (First, Singular) => None,
            (First, Plural) => Some(first),
            (First, Collective) | (First, Nullar) => Some(first2),
            (Second, _) => Some(second),
            (Third, Collective) | (Third, Nullar) => Some(third2),
            (Third, _) => Some(third),
        };

        let prefix = nucleus1
            .map(|nucleus| Syllable::new(onset, nucleus, coda).unwrap());

        Affix { prefix, nucleus: nucleus2 }
    }

    pub fn inflect(
        &self,
        person: Person,
        case: Case,
        gender: Gender,
        number: Number,
    ) -> pronoun::Inflected {
        let affix = Self::affix(person, case, gender, number);
        let phonemes = match (affix.prefix, affix.nucleus) {
            (Some(prefix), Some(nucleus)) => self
                .fst_nom_div_sing
                .prepend(prefix)
                .unwrap()
                .replace_final_nucleus(nucleus)
                .unwrap(),
            (Some(prefix), None) => {
                self.fst_nom_div_sing.prepend(prefix).unwrap()
            },
            (None, Some(nucleus)) => {
                self.fst_nom_div_sing.replace_final_nucleus(nucleus).unwrap()
            },
            (None, None) => self.fst_nom_div_sing.clone(),
        };
        pronoun::Inflected { phonemes, person, case, gender, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Demonstrative Class",
            entries: component::person_case_gender_number_table(
                |person, case, gender, number| {
                    let affix = Self::affix(person, case, gender, number);
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
            id: Id::new("this-near-far").unwrap(),
            word: Word::new(phonology::Word::parse_str("nyá").unwrap())
                .unwrap(),
            meanings: vec![Meaning::ThisNear, Meaning::ThisFar],
            notes: "Sense 2 is only used if there is no contrast between near \
                    and far demonstratives."
                .blocking()
                .to_dyn(),
        },
        Definition {
            id: Id::new("this-very-far").unwrap(),
            word: Word::new(phonology::Word::parse_str("xím").unwrap())
                .unwrap(),
            meanings: vec![Meaning::ThisVeryFar],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("this-far").unwrap(),
            word: Word::new(phonology::Word::parse_str("reŋ").unwrap())
                .unwrap(),
            meanings: vec![Meaning::ThisFar],
            notes: vec![
                "Note: only used when contrast is needed with ".to_dyn(),
                Link {
                    location: Location::internal(format!(
                        "{}/dictionary/nyá#this-near-far",
                        StarLang.path()
                    )),
                    text: WithStarAlphabet("nyá"),
                }
                .to_dyn(),
                ".".to_dyn(),
            ]
            .blocking()
            .to_dyn(),
        },
    ]
}
