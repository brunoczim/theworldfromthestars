pub mod regular;

use crate::{
    dictionary::Entry,
    grammar::grammemes::{BasicCase, Gender, Number},
    phonology,
};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in regular::definitions() {
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
