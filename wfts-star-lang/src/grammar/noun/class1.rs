use crate::{
    component::{DefinitionHead, Pronunciation, WithStarAlphabet},
    grammar::{
        grammemes::{BasicCase, Gender, Number},
        noun,
    },
    phonology::{self, Coda, Parse, Phoneme, Syllable},
};
use std::{
    collections::HashMap,
    fmt::{self},
};
use thiserror::Error;
use wfts_lang::semantics::Meaning;
use wfts_pedia_ssg::{
    component::{
        list::OrderedList,
        table::Table,
        Component,
        DynComponent,
        InlineComponent,
    },
    location::Id,
    page::Section,
};

#[derive(Debug, Clone)]
pub struct Definition<N>
where
    N: Component + Send + Sync + 'static,
{
    pub id: Id,
    pub word: Word,
    pub meanings: Vec<Meaning>,
    pub notes: N,
}

impl<N> Definition<N>
where
    N: Component + Send + Sync + 'static + Clone,
{
    pub fn make_sections(self) -> Vec<(phonology::Word, Section)> {
        let mut map = HashMap::new();
        for &case in BasicCase::ALL {
            for &gender in Gender::ALL {
                for &number in Number::ALL {
                    let table = self.word.table(case, gender, number);
                    let inflected =
                        self.word.inflect(case, gender, number).phonemes;
                    let (vec, _) =
                        map.entry(inflected).or_insert((Vec::new(), table));
                    vec.push((case, number, gender));
                }
            }
        }

        let mut sections = Vec::new();
        let meanings = self
            .meanings
            .into_iter()
            .map(|def| def.description())
            .collect::<Vec<_>>();
        for (inflected, (inflections, table)) in map {
            let head = DefinitionHead {
                name: inflected.to_text(),
                inflected_for: inflections
                    .into_iter()
                    .map(|(case, gender, number)| {
                        format!("{} {} {}", case, gender, number)
                    })
                    .collect(),
            };

            let pronunciation = Section {
                title: "Pronunciation".to_dyn(),
                id: Id::new(format!("{}-pronunciation", self.id.as_str()))
                    .unwrap(),
                body: Pronunciation(inflected.clone().into()).to_dyn(),
                children: vec![],
            };

            let inflection = Section {
                title: "Inflection".to_dyn(),
                id: Id::new(format!("{}-inflection", self.id.as_str()))
                    .unwrap(),
                body: table.to_dyn(),
                children: vec![],
            };

            let section = Section {
                title: "Definition".to_dyn(),
                id: self.id.clone(),
                body: vec![
                    head.to_dyn(),
                    OrderedList(meanings.clone()).to_dyn(),
                    self.notes.clone().blocking().to_dyn(),
                ]
                .to_dyn(),
                children: vec![pronunciation, inflection],
            };

            sections.push((inflected, section));
        }
        sections
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid nominative divine singular {nom_div_sing:?} for noun class 1")]
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

    pub fn table(
        &self,
        title_case: BasicCase,
        title_gender: Gender,
        title_number: Number,
    ) -> Table<DynComponent<InlineComponent>, DynComponent> {
        let title_word = self
            .inflect(title_case, title_gender, title_number)
            .phonemes
            .to_text();
        let title = vec![
            "Inflection for ".to_dyn(),
            WithStarAlphabet(title_word).to_dyn(),
            ".".to_dyn(),
        ];
        noun::full_inflection_table(title.to_dyn(), |case, gender, number| {
            self.inflect(case, gender, number)
                .phonemes
                .to_text()
                .blocking()
                .to_dyn()
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
        noun::full_inflection_table(
            "Inflection For Class 1",
            |case, gender, number| {
                Self::affix(case, gender, number)
                    .to_string()
                    .blocking()
                    .to_dyn()
            },
        )
    }
}
