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
            class: "Demonstrative Class".to_owned(),
            id: self.id,
            inflections: {
                let mut map = IndexMap::new();
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
    pub suffix: Vec<Syllable>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        for syllable in &self.suffix {
            for ph in syllable.phonemes() {
                write!(fmt, "{}", ph)?;
            }
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
        let last = fst_nom_div_sing.phonemes().next_back().unwrap();
        match last {
            Phoneme::R
            | Phoneme::Rr
            | Phoneme::Y
            | Phoneme::A
            | Phoneme::E
            | Phoneme::I
            | Phoneme::Ii => Err(Invalid { fst_nom_div_sing })?,
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

        let first = Coda::parse(&[]).unwrap();
        let first2 = Coda::parse(&[Phoneme::S]).unwrap();
        let second = Coda::parse(&[Phoneme::X]).unwrap();
        let second2 = Coda::parse(&[Phoneme::W]).unwrap();
        let third = Coda::parse(&[Phoneme::Y]).unwrap();
        let third2 = Coda::parse(&[Phoneme::X]).unwrap();

        let nominative = Phoneme::A;
        let nominative2 = Phoneme::E;
        let accusative = Phoneme::E;
        let accusative2 = Phoneme::I;
        let accusative3 = Phoneme::Ii;
        let topical = Phoneme::Ii;
        let postpositional = Phoneme::E;
        let passive = Phoneme::A;
        let passive2 = Phoneme::Ii;

        let divine = Onset::parse(&[]).unwrap();
        let animate = Onset::parse(&[Phoneme::Y]).unwrap();
        let animate2 = Onset::parse(&[Phoneme::Rr]).unwrap();
        let inanimate = Onset::parse(&[Phoneme::R]).unwrap();

        let plural = Syllable::parse(&[Phoneme::T, Phoneme::E]).unwrap();
        let plural2 = Syllable::parse(&[Phoneme::T, Phoneme::A]).unwrap();
        let nullar = Syllable::parse(&[Phoneme::P, Phoneme::E]).unwrap();
        let nullar2 =
            Syllable::parse(&[Phoneme::P, Phoneme::A, Phoneme::N]).unwrap();
        let nullar3 =
            Syllable::parse(&[Phoneme::T, Phoneme::A, Phoneme::N]).unwrap();
        let collective = Syllable::parse(&[Phoneme::B, Phoneme::Aa]).unwrap();

        let mut suffix = Vec::new();

        let onset = match (person, case, gender) {
            (_, _, Divine) => divine,
            (First, Basic(Accusative), Animate)
            | (Second, Basic(Accusative), Animate)
            | (Second, Passive, Animate) => animate2,
            (_, _, Animate) => animate,
            (_, _, Inanimate) => inanimate,
        };

        let nucleus = match (person, case, gender, number) {
            (First, Basic(Nominative), Divine, Singular)
            | (First, Basic(Nominative), Animate, Singular) => None,
            (_, Basic(Nominative), _, Singular)
            | (_, Basic(Nominative), _, Plural) => Some(nominative),
            (_, Basic(Nominative), ..) => Some(nominative2),
            (_, Basic(Accusative), _, Singular)
            | (_, Basic(Accusative), _, Plural) => Some(accusative),
            (_, Basic(Accusative), _, Nullar)
            | (Second, Basic(Accusative), ..) => Some(accusative2),
            (_, Basic(Accusative), ..) => Some(accusative3),
            (_, Basic(Topical), ..) => Some(topical),
            (First, Basic(Postpositional), Divine, Singular)
            | (First, Basic(Postpositional), Animate, Singular) => None,
            (_, Basic(Postpositional), ..) => Some(postpositional),
            (_, Passive, _, Singular) | (_, Passive, _, Plural) => {
                Some(passive)
            },
            (_, Passive, ..) => Some(passive2),
        };

        let coda = match (person, case, number) {
            (First, Basic(Nominative), _) => first,
            (First, ..) => first2,
            (Second, Basic(Nominative), _)
            | (Second, Passive, _)
            | (Second, _, Collective) => second2,
            (Second, ..) => second,
            (Third, _, Collective) => third2,
            (Third, ..) => third,
        };

        if let Some(nucleus) = nucleus {
            suffix.push(Syllable::new(onset, nucleus, coda).unwrap());
        }

        let extra_syllable =
            match (person, case, number) {
                (_, _, Singular)
                | (_, Basic(Nominative), Plural)
                | (_, Basic(Nominative), Collective)
                | (_, Basic(Accusative), Collective) => None,
                (Third, Basic(Accusative), Plural)
                | (Third, Passive, Plural) => Some(plural2),
                (_, _, Plural) => Some(plural),
                (Third, Basic(Topical), Nullar) | (Third, Passive, Nullar) => {
                    Some(nullar2)
                },
                (First, Passive, Nullar)
                | (Third, Basic(Nominative), Nullar) => Some(nullar3),
                (_, _, Nullar) => Some(nullar),
                (_, _, Collective) => Some(collective),
            };

        if let Some(extra) = extra_syllable {
            suffix.push(extra);
        }

        Affix { suffix }
    }

    pub fn inflect(
        &self,
        person: Person,
        case: Case,
        gender: Gender,
        number: Number,
    ) -> pronoun::Inflected {
        let affix = Self::affix(person, case, gender, number);
        let mut phonemes = self.fst_nom_div_sing.clone();
        for &syllable in &affix.suffix {
            phonemes = phonemes.append(syllable).unwrap();
        }
        pronoun::Inflected { phonemes, person, case, gender, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Relative Class",
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
    vec![Definition {
        id: Id::new("what").unwrap(),
        word: Word::new(phonology::Word::parse_str("kas").unwrap()).unwrap(),
        meanings: vec![Meaning::What, Meaning::ThatRelative],
        notes: "".blocking().to_dyn(),
    }]
}
