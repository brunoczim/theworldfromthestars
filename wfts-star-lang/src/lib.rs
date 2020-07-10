mod phonology;

use self::phonology::{Phoneme, Syllable, Word};
use wfts_lang::{Lang, LangCode};
use wfts_pedia_ssg::site::Site;

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

    fn subsite(&self) -> Site {
        unimplemented!()
    }
}
