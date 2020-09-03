pub mod additive;
pub mod isomorphic;

use crate::{dictionary::Entry, grammar::grammemes::ClauseCase, phonology};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in additive::definitions() {
        entries.push(def.to_dict_entry())
    }
    for def in isomorphic::definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Inflected {
    pub phonemes: phonology::Word,
    pub case: ClauseCase,
}
