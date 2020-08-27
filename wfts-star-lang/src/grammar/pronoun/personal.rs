use crate::{
    component,
    component::WithStarAlphabet,
    dictionary,
    grammar::{
        grammemes::{BasicCase, Case, Gender, Number, Person},
        pronoun,
    },
    phonology::{self, Coda, Parse, Phoneme},
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
            class: "Personal Class".to_owned(),
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
     personal class"
)]
pub struct Invalid {
    pub fst_nom_div_sing: phonology::Word,
}

#[derive(Debug, Clone)]
pub struct Affix {
    pub onset_outer_medial: Option<Phoneme>,
    pub onset_inner: Option<Phoneme>,
    pub nucleus: Option<Phoneme>,
    pub coda: Option<Coda>,
}

impl fmt::Display for Affix {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "-")?;
        let mut needs_dash = false;
        if let Some(ph) = self.onset_outer_medial {
            write!(fmt, "{}", ph)?;
            needs_dash = true;
        }
        if let Some(ph) = self.onset_inner {
            write!(fmt, "{}", ph)?;
            needs_dash = true;
        } else if needs_dash {
            write!(fmt, "-")?;
            needs_dash = false;
        }
        if let Some(ph) = self.nucleus {
            write!(fmt, "{}", ph)?;
            needs_dash = true;
        } else if needs_dash {
            write!(fmt, "-")?;
            needs_dash = false;
        }
        if let Some(coda) = self.coda {
            for ph in coda.phonemes() {
                write!(fmt, "{}", ph)?;
                needs_dash = true;
            }
        }
        if needs_dash {
            write!(fmt, "-")?;
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
        let sec_syl_first = fst_nom_div_sing
            .syllables()
            .get(1)
            .and_then(|syl| syl.phonemes().next());
        match (first, sec_syl_first) {
            (Phoneme::H, _)
            | (Phoneme::S, _)
            | (_, Some(Phoneme::H))
            | (_, Some(Phoneme::S))
            | (_, Some(Phoneme::W)) => Err(Invalid { fst_nom_div_sing })?,
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

        let accusative = Phoneme::R;
        let accusative2 = Phoneme::W;
        let topical = Phoneme::W;
        let topical2 = Phoneme::Y;
        let postpositional = Phoneme::R;
        let postpositional2 = Phoneme::Y;
        let passive = Phoneme::Rr;

        let divine = Coda::parse(&[]).unwrap();
        let divine2 = Coda::parse(&[Phoneme::W]).unwrap();
        let animate = Coda::parse(&[Phoneme::H]).unwrap();
        let animate2 = Coda::parse(&[Phoneme::Y, Phoneme::H]).unwrap();
        let inanimate = Coda::parse(&[Phoneme::Y, Phoneme::S]).unwrap();

        let singular = Phoneme::M;
        let singular2 = Phoneme::Nj;
        let plural = Phoneme::H;
        let plural2 = Phoneme::S;
        let nullar = Phoneme::N;
        let collective = Phoneme::Mg;
        let collective2 = Phoneme::Ng;

        let first = Phoneme::Aa;
        let first2 = Phoneme::A;
        let first3 = Phoneme::I;
        let second = Phoneme::Ee;
        let second2 = Phoneme::Aa;
        let third = Phoneme::I;
        let third2 = Phoneme::Ee;
        let third3 = Phoneme::Ii;

        let onset_inner = match (case, gender, number) {
            (Basic(Nominative), ..) | (Passive, Inanimate, _) => None,
            (Basic(Accusative), _, Nullar)
            | (Basic(Accusative), _, Collective) => Some(accusative2),
            (Basic(Accusative), ..) => Some(accusative),
            (Basic(Topical), Inanimate, Nullar) => Some(topical2),
            (Basic(Topical), ..) => Some(topical),
            (Basic(Postpositional), Animate, _) => Some(postpositional2),
            (Basic(Postpositional), ..) => Some(postpositional),
            (Passive, ..) => Some(passive),
        };

        let coda = match (case, gender, number) {
            (Basic(Accusative), Divine, _) => Some(divine2),
            (Basic(Nominative), Divine, Singular)
            | (_, Divine, Plural)
            | (Passive, Divine, _) => Some(divine),
            (_, Divine, _)
            | (Basic(Postpositional), _, Singular)
            | (Basic(Postpositional), _, Plural)
            | (_, Inanimate, Collective)
            | (Basic(Nominative), Animate, Plural)
            | (Basic(Accusative), Animate, Plural)
            | (Basic(Topical), Animate, Plural)
            | (Basic(Nominative), Animate, Nullar)
            | (Basic(Accusative), Animate, Nullar)
            | (Basic(Topical), Animate, Nullar) => None,
            (Basic(Accusative), Animate, Singular)
            | (Basic(Accusative), Animate, Collective) => Some(animate2),
            (_, Animate, _) => Some(animate),
            (_, Inanimate, _) => Some(inanimate),
        };

        let onset_outer_medial = match (case, number) {
            (Basic(Nominative), Singular) => None,
            (Passive, Singular) => Some(singular2),
            (_, Singular) => Some(singular),
            (Passive, Plural) => Some(plural2),
            (_, Plural) => Some(plural),
            (_, Nullar) => Some(nullar),
            (Basic(Postpositional), Collective) => Some(collective2),
            (_, Collective) => Some(collective),
        };

        let nucleus = match (person, case, gender, number) {
            (First, _, _, Collective)
            | (First, _, _, Nullar)
            | (First, Passive, ..)
            | (First, Basic(Postpositional), ..) => Some(first2),
            (First, _, Animate, _) => Some(first3),
            (First, _, _, Plural) => Some(first),
            (First, _, _, Singular) => None,
            (Second, Basic(Topical), Animate, _)
            | (Second, Basic(Postpositional), Animate, _) => Some(second2),
            (Second, _, ..) => Some(second),
            (Third, _, _, Collective)
            | (Third, _, _, Nullar)
            | (Third, Passive, ..) => Some(third2),
            (Third, _, Animate, _) => Some(third3),
            (Third, _, ..) => Some(third),
        };

        Affix { onset_outer_medial, onset_inner, nucleus, coda }
    }

    pub fn inflect(
        &self,
        person: Person,
        case: Case,
        gender: Gender,
        number: Number,
    ) -> pronoun::Inflected {
        let affix = Self::affix(person, case, gender, number);
        let onset = self.fst_nom_div_sing.syllables().first().unwrap().onset();
        let new_onset = onset
            .replace_keep_plosive(affix.onset_outer_medial, affix.onset_inner)
            .unwrap();
        let replaced =
            self.fst_nom_div_sing.replace_initial_onset(new_onset).unwrap();
        let phonemes = match (affix.nucleus, affix.coda) {
            (Some(nucleus), Some(coda)) => {
                replaced.replace_initial_rhyme(nucleus, coda).unwrap()
            },
            (Some(nucleus), None) => {
                replaced.replace_initial_nucleus(nucleus).unwrap()
            },
            (None, Some(coda)) => replaced.replace_initial_coda(coda).unwrap(),
            (None, None) => replaced,
        };
        pronoun::Inflected { phonemes, person, case, gender, number }
    }

    pub fn affix_table() -> Table<&'static str, DynComponent> {
        Table {
            title: "Inflection For Personal Class",
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
        id: Id::new("informal-personal").unwrap(),
        word: Word::new(phonology::Word::parse_str("fá").unwrap()).unwrap(),
        meanings: vec![Meaning::InformalPersonal],
        notes: "".blocking().to_dyn(),
    }]
}
