mod phonology;

use self::phonology::{Phoneme, Syllable, Word};
use std::collections::HashMap;
use wfts_lang::{Lang, LangCode};
use wfts_pedia_ssg::{
    component::Component,
    location::InternalPath,
    page::Page,
    site::{Directory, Node},
};

#[derive(Debug, Clone)]
pub struct StarLang;

impl StarLang {
    fn make_index(&self, dir: &mut Directory) {
        dir.contents.insert(
            InternalPath::parse("index.html").unwrap(),
            Node::Page(Page {
                title: String::from("Star Language"),
                body: "hello".to_dyn(),
                sections: Vec::new(),
            }),
        );
    }
}

impl Lang for StarLang {
    type Character = Phoneme;
    type Phoneme = Phoneme;
    type Syllable = Syllable;
    type Word = Word;

    fn code(&self) -> LangCode {
        LangCode::parse("str-cls").unwrap()
    }

    fn subsite(&self) -> Directory {
        let mut dir = Directory { contents: HashMap::new() };
        self.make_index(&mut dir);
        dir
    }
}
