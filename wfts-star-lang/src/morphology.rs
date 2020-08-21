use crate::phonology::{Parse, Phoneme, Word};
use std::{collections::BTreeSet, fmt, fmt::Write};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Morpheme {
    Template(Template),
    Word(Word),
}

impl From<Template> for Morpheme {
    fn from(template: Template) -> Self {
        Self::Template(template)
    }
}

impl From<Word> for Morpheme {
    fn from(word: Word) -> Self {
        Self::Word(word)
    }
}

impl Morpheme {
    pub fn to_broad_ipa(&self) -> String {
        match self {
            Morpheme::Template(temp) => temp.to_broad_ipa(),
            Morpheme::Word(word) => word.to_broad_ipa(),
        }
    }

    pub fn to_text(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for Morpheme {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Morpheme::Template(temp) => fmt::Display::fmt(temp, fmt),
            Morpheme::Word(word) => fmt::Display::fmt(word, fmt),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Template {
    phonemes: Vec<Phoneme>,
    holes: BTreeSet<usize>,
}

impl Template {
    pub fn new<I>(phonemes: Vec<Phoneme>, holes_iter: I) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = usize>,
    {
        let mut holes = BTreeSet::new();

        for hole in holes_iter {
            if hole > phonemes.len() || !holes.insert(hole) {
                Err(TemplateHoleOutOfBounds {
                    phonemes: phonemes.clone(),
                    hole,
                })?;
            }
        }

        Ok(Self { phonemes, holes })
    }

    pub fn phonemes(&self) -> &[Phoneme] {
        &self.phonemes
    }

    pub fn holes(&self) -> &BTreeSet<usize> {
        &self.holes
    }

    pub fn fill(
        &mut self,
        hole: usize,
        phonemes: &[Phoneme],
    ) -> anyhow::Result<()> {
        if !self.holes.remove(&hole) {
            Err(InvalidTemplateHole { template: self.clone(), hole })?;
        }

        let prev_len = self.phonemes.len();
        self.phonemes.extend_from_slice(phonemes);
        let (left, right) = self.phonemes.split_at_mut(prev_len);
        left[hole .. hole + phonemes.len()].swap_with_slice(right);

        Ok(())
    }

    pub fn into_word(&self) -> anyhow::Result<Word> {
        if self.holes.is_empty() {
            Word::parse(&self.phonemes)
        } else {
            Err(NonFilledTemplate { template: self.clone() })?
        }
    }

    pub fn to_text(&self) -> String {
        format!("{}", self)
    }

    pub fn to_broad_ipa(&self) -> String {
        let mut holes = self.holes.iter().peekable();
        let mut output = String::new();

        for (i, ch) in self.phonemes().iter().enumerate() {
            if holes.peek().map_or(false, |&&hole| i == hole) {
                output.push_str("-");
                holes.next();
            }
            write!(output, "{}", ch.to_broad_ipa()).unwrap();
        }
        if let Some(_) = holes.next() {
            output.push_str("-");
        }

        output
    }
}

impl fmt::Display for Template {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut holes = self.holes.iter().peekable();

        for (i, ch) in self.phonemes().iter().enumerate() {
            if holes.peek().map_or(false, |&&hole| i == hole) {
                fmt.write_str("-")?;
                holes.next();
            }
            write!(fmt, "{}", ch.to_text())?;
        }
        if let Some(_) = holes.next() {
            fmt.write_str("-")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Error)]
#[error("Hole {hole} out bounds of phonemes {phonemes:?}")]
pub struct TemplateHoleOutOfBounds {
    pub phonemes: Vec<Phoneme>,
    pub hole: usize,
}

#[derive(Debug, Clone, Error)]
#[error("Invalid hole {hole} of template {template:?}")]
pub struct InvalidTemplateHole {
    pub template: Template,
    pub hole: usize,
}

#[derive(Debug, Clone, Error)]
#[error("Template {template:?} is not fully filled")]
pub struct NonFilledTemplate {
    pub template: Template,
}
