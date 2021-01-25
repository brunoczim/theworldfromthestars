pub mod phonetics;
pub mod phonology;
pub mod pages;

#[cfg(test)]
mod test;

use crate::{Lang, LangCode};
use std::collections::HashMap;
use wfts_pedia_ssg::site::Directory;

#[derive(Debug, Clone)]
pub struct ProtoDivine;

impl Lang for ProtoDivine {
    fn code(&self) -> LangCode {
        LangCode::parse("div-prt").unwrap()
    }

    fn subsite(&self) -> Directory {
        let mut dir = Directory { contents: HashMap::new() };
        pages::index::make(&mut dir);
        pages::phonology::make(&mut dir);
        pages::grammar::make(&mut dir);
        /*
        pages::writing::make(&mut dir);
        pages::dictionary::make(&mut dir);
        */
        dir
    }
}
