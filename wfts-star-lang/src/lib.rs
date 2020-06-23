mod phonology;

use self::phonology::{Phoneme, Syllable, Word};
use std::collections::HashMap;
use wfts_lang::Lang;
use wfts_pedia_ssg::{location, location::Location, site::Site};

#[derive(Debug, Clone)]
pub struct StarLang;

impl Lang for StarLang {
    type Character = Phoneme;
    type Phoneme = Phoneme;
    type Syllable = Syllable;
    type Word = Word;

    fn subsite(&self) -> Site {
        unimplemented!()
    }
}
