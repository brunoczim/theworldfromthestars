pub mod regular1;

use crate::{
    dictionary::Entry,
    grammar::grammemes::{Person, Tense},
    phonology,
};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in regular1::definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub person: Person,
    pub tense: Tense,
}
