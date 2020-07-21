use crate::phonology;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("The word {phonemes:?} is invalid for the part of speech {part:?}")]
pub struct InvalidWord {
    pub phonemes: phonology::Word,
    pub part: PartOfSpeech,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word {
    phonemes: phonology::Word,
    part: PartOfSpeech,
}

impl Word {
    pub fn new(
        phonemes: phonology::Word,
        part: PartOfSpeech,
    ) -> anyhow::Result<Self> {
        if part.valid_word(&phonemes) {
            Ok(Self { phonemes, part })
        } else {
            Err(InvalidWord { phonemes, part })?
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartOfSpeech {
    Noun(Noun),
    Adjective(Adjective),
    Pronoun(Pronoun),
    Postposition(Postposition),
    Conjuction(Conjunction),
    Verb(Verb),
    Adverb(Adverb),
    Root(Root),
}

impl PartOfSpeech {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match self {
            PartOfSpeech::Noun(part) => part.valid_word(word),
            PartOfSpeech::Adjective(part) => part.valid_word(word),
            PartOfSpeech::Pronoun(part) => part.valid_word(word),
            PartOfSpeech::Postposition(part) => part.valid_word(word),
            PartOfSpeech::Conjuction(part) => part.valid_word(word),
            PartOfSpeech::Verb(part) => part.valid_word(word),
            PartOfSpeech::Adverb(part) => part.valid_word(word),
            PartOfSpeech::Root(part) => part.valid_word(word),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Noun {
    pub class: NounClass,
}

impl Noun {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NounClass {}

impl NounClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Adjective {
    pub class: AdjectiveClass,
}

impl Adjective {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdjectiveClass {}

impl AdjectiveClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pronoun {
    pub class: PronounClass,
}

impl Pronoun {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PronounClass {}

impl PronounClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Postposition {
    pub class: PostpositionClass,
}

impl Postposition {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostpositionClass {}

impl PostpositionClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Conjunction {
    pub class: ConjunctionClass,
}

impl Conjunction {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConjunctionClass {}

impl ConjunctionClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Verb {
    pub class: VerbClass,
}

impl Verb {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerbClass {}

impl VerbClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Adverb {
    pub class: AdverbClass,
}

impl Adverb {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        self.class.valid_word(word)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdverbClass {}

impl AdverbClass {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        match *self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Root {}

impl Root {
    pub fn valid_word(&self, word: &phonology::Word) -> bool {
        true
    }
}
