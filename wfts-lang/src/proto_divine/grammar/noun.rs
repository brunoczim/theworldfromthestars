mod divine;
mod mortal;
mod variable;

use crate::proto_divine::{
    dictionary::Entry,
    grammar::grammemes::{Case, Gender, Number},
    phonology::Word,
};

pub fn entries() -> Vec<Entry> {
    let mut entries = Vec::new();

    for def in variable::definitions() {
        entries.push(def.to_dict_entry());
    }

    entries
}

#[derive(Debug, Clone)]
pub struct Inflected {
    pub phonemes: Word,
    pub case: Case,
    pub gender: Gender,
    pub number: Number,
}
