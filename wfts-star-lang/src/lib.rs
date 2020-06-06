mod phonology;

use self::phonology::{Phoneme, Syllable, Word};
use wfts_lang::Lang;

#[derive(Debug, Clone)]
pub struct StarLang;

impl Lang for StarLang {
    type Character = Phoneme;
    type Phoneme = Phoneme;
    type Syllable = Syllable;
    type Word = Word;
}
