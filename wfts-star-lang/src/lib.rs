pub mod component;
pub mod phonology;
pub mod grammar;
pub mod pages;

use std::collections::HashMap;
use wfts_lang::{Lang, LangCode};
use wfts_pedia_ssg::site::Directory;

#[derive(Debug, Clone)]
pub struct StarLang;

impl Lang for StarLang {
    fn code(&self) -> LangCode {
        LangCode::parse("str-cls").unwrap()
    }

    fn subsite(&self) -> Directory {
        let mut dir = Directory { contents: HashMap::new() };
        self::pages::index::make(&mut dir);
        self::pages::phonology::make(&mut dir);
        self::pages::grammar::make(&mut dir);
        self::pages::words::make(&mut dir);
        dir
    }
}
