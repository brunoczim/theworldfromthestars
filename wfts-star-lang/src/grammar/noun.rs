pub mod class1;

use crate::{
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub case: BasicCase,
    pub gender: Gender,
    pub number: Number,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Word {
    Class1 { word: class1::Word, gender: Gender, number: Number },
}
