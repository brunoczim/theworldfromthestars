pub mod unary;
pub mod binary;

use crate::{dictionary::Entry, grammar::grammemes::Case, phonology};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in unary::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in binary::definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub case: Case,
}
