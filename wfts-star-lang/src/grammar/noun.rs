use crate::{
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology::{Coda, Parse, Phoneme, Syllable, Word},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid nominative divine singular {nom_div_sing:?} for noun class 1")]
pub struct InvalidClass1 {
    pub nom_div_sing: Word,
}

#[derive(Debug)]
pub struct Class1 {
    nom_div_sing: Word,
}

impl Class1 {
    pub fn new(nom_div_sing: Word) -> anyhow::Result<Self> {
        let last = nom_div_sing.syllables().last().unwrap();
        let no_coda = last.coda().iter().next().is_none();
        match last.nucleus() {
            Phoneme::E | Phoneme::Ee | Phoneme::I if no_coda => {
                Err(InvalidClass1 { nom_div_sing })?
            },
            _ => Ok(Self { nom_div_sing }),
        }
    }

    pub fn inflect(
        &self,
        case: BasicCase,
        gender: Gender,
        number: Number,
    ) -> Word {
        use BasicCase::*;
        use Gender::*;
        use Number::*;

        let accusative = Coda::new(None, Some(Phoneme::N)).unwrap();
        let accusative2 = Coda::new(None, Some(Phoneme::Mg)).unwrap();
        let accusative3 = Coda::new(Some(Phoneme::W), None).unwrap();
        let topical = Coda::new(None, Some(Phoneme::F)).unwrap();
        let topical2 = Coda::new(Some(Phoneme::W), None).unwrap();
        let topical3 = Coda::new(Some(Phoneme::J), None).unwrap();
        let postpositional =
            Coda::new(Some(Phoneme::J), Some(Phoneme::S)).unwrap();
        let postpositional2 = Coda::new(Some(Phoneme::J), None).unwrap();
        let animate = Phoneme::Aa;
        let animate2 = Phoneme::Ee;
        let inanimate = Phoneme::I;

        let plural = Syllable::parse(&[Phoneme::Ee]).unwrap();
        let nullar = Syllable::parse(&[Phoneme::E, Phoneme::N]).unwrap();
        let collective = Syllable::parse(&[Phoneme::I, Phoneme::Xw]).unwrap();
        let collective2 = Syllable::parse(&[Phoneme::I, Phoneme::W]).unwrap();

        let base = match (case, gender) {
            (Nominative, Divine) => self.nom_div_sing.clone(),
            (Accusative, Divine) => {
                self.nom_div_sing.replace_final_coda(accusative).unwrap()
            },
            (Topical, Divine) => {
                self.nom_div_sing.replace_final_coda(topical).unwrap()
            },
            (Postpositional, Divine) => {
                self.nom_div_sing.replace_final_coda(postpositional).unwrap()
            },
            (Nominative, Animate) => {
                self.nom_div_sing.replace_final_nucleus(animate).unwrap()
            },
            (Accusative, Animate) if number == Nullar => self
                .nom_div_sing
                .replace_final_rhyme(animate2, accusative2)
                .unwrap(),
            (Accusative, Animate) if number == Collective => self
                .nom_div_sing
                .replace_final_rhyme(animate2, accusative3)
                .unwrap(),
            (Accusative, Animate) => self
                .nom_div_sing
                .replace_final_rhyme(animate, accusative)
                .unwrap(),
            (Topical, Animate) => self
                .nom_div_sing
                .replace_final_rhyme(animate, topical2)
                .unwrap(),
            (Postpositional, Animate) => self
                .nom_div_sing
                .replace_final_rhyme(animate, postpositional2)
                .unwrap(),
            (Nominative, Inanimate) => {
                self.nom_div_sing.replace_final_nucleus(inanimate).unwrap()
            },
            (Accusative, Inanimate) => self
                .nom_div_sing
                .replace_final_rhyme(inanimate, accusative)
                .unwrap(),
            (Topical, Inanimate) if number == Nullar => self
                .nom_div_sing
                .replace_final_rhyme(inanimate, topical3)
                .unwrap(),
            (Topical, Inanimate) => self
                .nom_div_sing
                .replace_final_rhyme(inanimate, topical)
                .unwrap(),
            (Postpositional, Inanimate) => self
                .nom_div_sing
                .replace_final_rhyme(inanimate, postpositional)
                .unwrap(),
        };

        match number {
            Singular => base,
            Plural => base.append(plural).unwrap(),
            Nullar => base.append(nullar).unwrap(),
            Collective if case == Postpositional => {
                base.append(collective2).unwrap()
            },
            Collective => base.append(collective).unwrap(),
        }
    }
}
