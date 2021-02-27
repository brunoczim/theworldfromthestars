use crate::{
    fmt::{Reconstructed, WriteOrthography},
    proto_divine::{
        components::case_fgender_number_table,
        dictionary,
        grammar::{
            grammemes::{Case, Gender, Number},
            noun::Inflected,
        },
        phonology::{Sonorant, Vowel, Word},
        ProtoDivine,
    },
    semantics::Meaning,
    Lang,
};
use indexmap::IndexMap;
use wfts_pedia_ssg::{
    component::{
        list::UnmarkedList,
        table,
        text::Link,
        Component,
        DynComponent,
    },
    location::{Id, Location},
};

#[derive(Debug, Clone)]
pub struct Definition {
    pub id: Id,
    pub word: Noun,
    pub meanings: Vec<Meaning>,
    pub notes: DynComponent,
}

impl Definition {
    pub fn to_dict_entry(self) -> dictionary::Entry {
        let gender = Gender::Mortal;

        dictionary::Entry::Inflected(dictionary::InflectedEntry {
            inflection_table: self.word.table(&self.id),
            class: Some("Mortal Class".to_owned()),
            id: self.id,
            inflections: {
                let mut map = IndexMap::new();
                for &case in Case::ALL {
                    for &number in Number::ALL {
                        map.insert(
                            format!("{} {} {}", case, gender, number),
                            self.word
                                .clone()
                                .inflect(case, number)
                                .phonemes
                                .into(),
                        );
                    }
                }
                map
            },
            meanings: self.meanings,
            notes: self.notes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Noun {
    nom_sing: Word,
}

impl Noun {
    pub fn table(&self, entry_id: &Id) -> table::Entries<DynComponent> {
        case_fgender_number_table(Gender::Mortal, |case, number| {
            let inflected = self.clone().inflect(case, number).phonemes;
            let link = Link {
                location: Location::internal(format!(
                    "{}/dictionary/{}#{}",
                    ProtoDivine.path(),
                    inflected.orthography_ref(),
                    entry_id,
                )),
                text: Reconstructed(inflected.clone().orthography()),
            };
            let component = UnmarkedList(vec![
                link.to_dyn(),
                inflected.orthography().to_dyn(),
            ]);
            component.blocking().to_dyn()
        })
    }

    pub fn inflect(self, case: Case, number: Number) -> Inflected {
        unimplemented!()
    }
}

pub fn noun_tool() -> Noun {
    use Sonorant::*;
    use Vowel::*;
    Noun { nom_sing: proto_div_word![W, Ao] }
}

pub fn noun_companion() -> Noun {
    use Sonorant::*;
    use Vowel::*;
    Noun { nom_sing: proto_div_word![J, O] }
}

pub fn definitions() -> Vec<Definition> {
    vec![]
}
