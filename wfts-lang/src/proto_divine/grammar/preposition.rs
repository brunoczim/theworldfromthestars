use crate::{
    proto_divine::{
        dictionary,
        phonology::{Obstruent, Sonorant, Vowel, Word},
    },
    semantics::Meaning,
};
use indexmap::IndexMap;
use wfts_pedia_ssg::{
    component::{Component, DynComponent},
    location::Id,
};

#[derive(Debug, Clone)]
pub struct Preposition {
    phonemes: Word,
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub id: Id,
    pub word: Preposition,
    pub meanings: Vec<Meaning>,
    pub notes: DynComponent,
}

impl Definition {
    pub fn to_dict_entry(self) -> dictionary::Entry {
        dictionary::Entry::Uninflected(dictionary::UninflectedEntry {
            id: self.id,
            meanings: self.meanings,
            notes: self.notes,
            word: self.word.phonemes,
        })
    }
}

pub fn entries() -> Vec<dictionary::Entry> {
    let mut entries = Vec::new();

    for def in definitions() {
        entries.push(def.to_dict_entry())
    }

    entries
}

pub fn prep_to_div_suffer() -> Preposition {
    use Vowel::*;
    Preposition { phonemes: proto_div_word![Ae] }
}

pub fn prep_to_mort_suffer() -> Preposition {
    use Vowel::*;
    Preposition { phonemes: proto_div_word![Ao] }
}

pub fn prep_to() -> Preposition {
    use Vowel::*;
    Preposition { phonemes: proto_div_word![I] }
}

pub fn prep_to_below() -> Preposition {
    use Sonorant::*;
    use Vowel::*;
    Preposition { phonemes: proto_div_word![J, Ao] }
}

pub fn prep_from() -> Preposition {
    use Obstruent::*;
    use Vowel::*;
    Preposition { phonemes: proto_div_word![T, I, S] }
}

pub fn prep_of() -> Preposition {
    use Obstruent::*;
    use Sonorant::*;
    use Vowel::*;
    Preposition { phonemes: proto_div_word![W, O, S] }
}

pub fn prep_with() -> Preposition {
    use Sonorant::*;
    use Vowel::*;
    Preposition { phonemes: proto_div_word![W, Ae] }
}

pub fn definitions() -> Vec<Definition> {
    vec![
        Definition {
            id: Id::new("to-div-suffer").unwrap(),
            word: prep_to_div_suffer(),
            meanings: vec![Meaning::ToDivSuffer],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("to-mort-suffer").unwrap(),
            word: prep_to_mort_suffer(),
            meanings: vec![Meaning::ToMortSuffer],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("to").unwrap(),
            word: prep_to(),
            meanings: vec![Meaning::To],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("to-below").unwrap(),
            word: prep_to_below(),
            meanings: vec![Meaning::ToBelow],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("from").unwrap(),
            word: prep_from(),
            meanings: vec![Meaning::From],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("of").unwrap(),
            word: prep_of(),
            meanings: vec![Meaning::Of],
            notes: "".blocking().to_dyn(),
        },
        Definition {
            id: Id::new("with").unwrap(),
            word: prep_with(),
            meanings: vec![Meaning::With],
            notes: "".blocking().to_dyn(),
        },
    ]
}
