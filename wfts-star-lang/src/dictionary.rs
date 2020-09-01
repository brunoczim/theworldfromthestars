use crate::{
    component::{DefinitionHead, Pronunciation, WithStarAlphabet},
    grammar::{adjective, conjunction, noun, postposition, pronoun},
    morphology::Morpheme,
};
use std::collections::HashMap;
use wfts_lang::semantics::Meaning;
use wfts_pedia_ssg::{
    component::{
        list::OrderedList,
        table::{self, Table},
        Component,
        DynComponent,
    },
    location::Id,
    page::Section,
};

#[derive(Debug, Clone)]
pub struct Entry {
    pub id: Id,
    pub class: String,
    pub inflections: HashMap<String, Morpheme>,
    pub meanings: Vec<Meaning>,
    pub notes: DynComponent,
    pub inflection_table: table::Entries<DynComponent>,
}

impl Entry {
    pub fn sections(self) -> Vec<(Morpheme, Section)> {
        let mut map = HashMap::new();
        for (key, morpheme) in self.inflections {
            let vec = map.entry(morpheme.clone()).or_insert(Vec::new());
            vec.push(key);
        }

        let meanings = self
            .meanings
            .into_iter()
            .map(|def| def.description())
            .collect::<Vec<_>>();
        let mut sections = Vec::new();

        for (morpheme, inflected_for) in map {
            let head =
                DefinitionHead { name: morpheme.to_text(), inflected_for };

            let romanization = Section {
                title: "Romanization".to_dyn(),
                id: Id::new(format!("{}-roman", self.id.as_str())).unwrap(),
                body: morpheme.to_text().blocking().to_dyn(),
                children: vec![],
            };

            let pronunciation = Section {
                title: "Pronunciation".to_dyn(),
                id: Id::new(format!("{}-pronunciation", self.id.as_str()))
                    .unwrap(),
                body: match &morpheme {
                    Morpheme::Template(_) => Pronunciation {
                        morpheme: morpheme.clone(),
                        audio_early: None,
                        audio_late: None,
                    },
                    Morpheme::Word(word) => Pronunciation {
                        morpheme: morpheme.clone(),
                        audio_early: word.audio_early(),
                        audio_late: word.audio_late(),
                    },
                }
                .to_dyn(),
                children: vec![],
            };

            let inflection = Section {
                title: "Inflection".to_dyn(),
                id: Id::new(format!("{}-inflection", self.id.as_str()))
                    .unwrap(),
                body: vec![
                    self.class.clone().blocking().to_dyn(),
                    Table {
                        title: vec![
                            "Inflection for ".to_dyn(),
                            WithStarAlphabet(morpheme.to_text()).to_dyn(),
                            ".".to_dyn(),
                        ]
                        .to_dyn(),
                        entries: self.inflection_table.clone(),
                    }
                    .to_dyn(),
                ]
                .to_dyn(),
                children: vec![],
            };

            let section = Section {
                title: "Definition".to_dyn(),
                id: self.id.clone(),
                body: vec![
                    head.to_dyn(),
                    OrderedList(meanings.clone()).to_dyn(),
                    self.notes.clone(),
                ]
                .to_dyn(),
                children: vec![romanization, pronunciation, inflection],
            };

            sections.push((morpheme, section));
        }

        sections
    }

    pub fn all() -> Vec<Self> {
        let mut entries = noun::entries();
        entries.append(&mut adjective::entries());
        entries.append(&mut pronoun::entries());
        entries.append(&mut postposition::entries());
        entries.append(&mut conjunction::entries());
        entries
    }
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    pub sections: HashMap<Morpheme, Vec<Section>>,
}

impl Dictionary {
    pub fn with_all_entries() -> Self {
        Self::from_entries(Entry::all())
    }

    pub fn from_entries(entries: Vec<Entry>) -> Self {
        let mut this = Self { sections: HashMap::new() };
        for entry in entries {
            for (morpheme, section) in entry.sections() {
                let vec = this.sections.entry(morpheme).or_insert(Vec::new());
                vec.push(section);
            }
        }
        this
    }
}
