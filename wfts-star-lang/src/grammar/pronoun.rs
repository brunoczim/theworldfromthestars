pub mod demonstrative;
pub mod personal;
pub mod relative;

use crate::{
    dictionary::Entry,
    grammar::grammemes::{Case, Gender, Number, Person},
    phonology,
};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in demonstrative::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in personal::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in relative::definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub person: Person,
    pub case: Case,
    pub gender: Gender,
    pub number: Number,
}
