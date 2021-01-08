use std::hash::Hash;

pub trait Phoneme: Ord + Hash {
    fn broad_ipa(&self) -> &str;
}

pub trait Syllable: Ord + Hash {
    type Phoneme: Phoneme;

    fn phonemes(&self) -> Vec<Self::Phoneme>;
}

pub trait Word: Ord + Hash {
    type Phoneme: Phoneme;
    type Syllable: Syllable<Phoneme = Self::Phoneme>;

    fn syllables(&self) -> Vec<Self::Syllable>;

    fn phonemes(&self) -> Vec<Self::Phoneme> {
        self.syllables().iter().map(Syllable::phonemes).flatten().collect()
    }

    fn broad_ipa(&self) -> String {
        self.phonemes().iter().map(Phoneme::broad_ipa).collect()
    }
}

pub trait Accent {
    type Word: Word;

    fn narrow_ipa(&self, word: &Self::Word) -> String;
}

pub trait Phonology {
    type Phoneme: Phoneme;
    type Syllable: Syllable<Phoneme = Self::Phoneme>;
    type Word: Word<Phoneme = Self::Phoneme, Syllable = Self::Syllable>;
    type Accent: Accent<Word = Self::Word>;

    fn accents(&self) -> Vec<Self::Accent>;
}
