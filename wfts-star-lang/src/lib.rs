pub mod phonology;
mod pages;

use self::phonology::{Phoneme, Syllable, Word};
use std::collections::HashMap;
use wfts_lang::{Lang, LangCode};
use wfts_pedia_ssg::site::Directory;

#[derive(Debug, Clone)]
pub struct StarLang;

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
        self::pages::index::make(&mut dir);
        self::pages::phonology::make(&mut dir);
        dir
    }
}
