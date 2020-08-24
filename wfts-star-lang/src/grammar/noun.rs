pub mod full1;
pub mod full2;
pub mod divine1;
pub mod divine2;

use crate::{
    dictionary::Entry,
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology,
};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in full1::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in full2::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in divine1::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in divine2::definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub case: BasicCase,
    pub gender: Gender,
    pub number: Number,
}
