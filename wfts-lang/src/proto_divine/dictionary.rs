use crate::{
    fmt::WriteOrthography,
    proto_divine::{
        components::{DefinitionHead, PronuncSection},
        grammar::{noun, preposition},
        phonology::Word,
    },
    semantics::Meaning,
};
use indexmap::IndexMap;
use std::collections::HashMap;
use wfts_pedia_ssg::{
    component::{
        list::OrderedList,
        table,
        table::Table,
        Component,
        DynComponent,
    },
    location::Id,
    page::Section,
};

#[derive(Debug, Clone)]
pub struct InflectedEntry {
    pub id: Id,
    pub meanings: Vec<Meaning>,
    pub notes: DynComponent,
    pub class: Option<String>,
    pub inflections: IndexMap<String, Word>,
    pub inflection_table: table::Entries<DynComponent>,
}

impl InflectedEntry {
    pub fn sections(self) -> Vec<(Word, Section)> {
        let mut map = HashMap::new();
        for (key, word) in self.inflections {
            let vec = map.entry(word.clone()).or_insert(Vec::new());
            vec.push(key);
        }

        let meanings = self
            .meanings
            .into_iter()
            .map(|def| def.description())
            .collect::<Vec<_>>();
        let mut sections = Vec::new();

        for (word, inflected_for) in map {
            let head = DefinitionHead { word: word.clone(), inflected_for };

            let pronunciation = Section {
                title: "Pronunciation".to_dyn(),
                id: Id::new(format!("{}-pronunciation", self.id.as_str()))
                    .unwrap(),
                body: PronuncSection::from_word(&word).to_dyn(),
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
                            "Inflection for *".to_dyn(),
                            word.orthography_ref().to_string().to_dyn(),
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
                children: vec![pronunciation, inflection],
            };

            sections.push((word, section));
        }

        sections
    }
}

#[derive(Debug, Clone)]
pub struct UninflectedEntry {
    pub id: Id,
    pub meanings: Vec<Meaning>,
    pub notes: DynComponent,
    pub word: Word,
}

impl UninflectedEntry {
    pub fn section(self) -> (Word, Section) {
        let head = DefinitionHead {
            word: self.word.clone(),
            inflected_for: Vec::new(),
        };

        let meanings = self
            .meanings
            .iter()
            .copied()
            .map(Meaning::description)
            .collect::<Vec<_>>();

        let pronunciation = Section {
            title: "Pronunciation".to_dyn(),
            id: Id::new(format!("{}-pronunciation", self.id.as_str())).unwrap(),
            body: PronuncSection::from_word(&self.word).to_dyn(),
            children: Vec::new(),
        };

        let section = Section {
            title: "Definition".to_dyn(),
            id: self.id,
            body: vec![
                head.to_dyn(),
                OrderedList(meanings).to_dyn(),
                self.notes,
            ]
            .to_dyn(),
            children: vec![pronunciation],
        };

        (self.word, section)
    }
}

#[derive(Debug, Clone)]
pub enum Entry {
    Uninflected(UninflectedEntry),
    Inflected(InflectedEntry),
}

impl Entry {
    pub fn sections(self) -> Vec<(Word, Section)> {
        match self {
            Entry::Inflected(entry) => entry.sections(),
            Entry::Uninflected(entry) => vec![entry.section()],
        }
    }

    pub fn all() -> Vec<Self> {
        let mut entries = noun::entries();
        entries.append(&mut preposition::entries());
        /*
        entries.append(&mut adjective::entries());
        entries.append(&mut pronoun::entries());
        entries.append(&mut conjunction::entries());
        entries.append(&mut verb::entries());
        */
        entries
    }
}

#[derive(Debug, Clone)]
pub struct Dictionary {
    pub sections: HashMap<Word, Vec<Section>>,
}

impl Dictionary {
    pub fn with_all_entries() -> Self {
        Self::from_entries(Entry::all())
    }

    pub fn from_entries(entries: Vec<Entry>) -> Self {
        let mut this = Self { sections: HashMap::new() };
        for entry in entries {
            for (word, section) in entry.sections() {
                let vec = this.sections.entry(word).or_insert(Vec::new());
                vec.push(section);
            }
        }
        this
    }
}
