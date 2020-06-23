use std::{borrow::Cow, hash::Hash};
use wfts_pedia_ssg::site::Site;

pub trait Lang: Sized {
    type Character: Character<Lang = Self>;
    type Word: Word<Lang = Self>;
    type Syllable: Syllable<Lang = Self>;
    type Phoneme: Phoneme<Lang = Self>;

    fn subsite(&self) -> Site;
}

pub trait Character:
    Sized + Clone + PartialEq + Eq + PartialOrd + Ord + Hash
{
    type Lang: Lang<Character = Self>;

    fn to_text(&self) -> Cow<str>;
}

pub trait Phoneme:
    Sized + Clone + PartialEq + Eq + PartialOrd + Ord + Hash
{
    type Lang: Lang<Phoneme = Self>;

    fn to_broad_ipa(&self) -> Cow<str>;
}

pub trait Syllable:
    Sized + Clone + PartialEq + Eq + PartialOrd + Ord + Hash
{
    type Lang: Lang<Syllable = Self>;

    fn phonemes(&self) -> Cow<[<Self::Lang as Lang>::Phoneme]>;

    fn to_broad_ipa(&self) -> Cow<str> {
        let mut output = String::new();

        for phoneme in self.phonemes().iter() {
            output.push_str(&phoneme.to_broad_ipa());
        }

        Cow::from(output)
    }
}

pub trait Word:
    Sized + Clone + PartialEq + Eq + PartialOrd + Ord + Hash
{
    type Lang: Lang<Word = Self>;

    fn chars(&self) -> Cow<[<Self::Lang as Lang>::Character]>;

    fn to_text(&self) -> Cow<str> {
        let mut output = String::new();
        for ch in self.chars().iter() {
            output.push_str(&ch.to_text());
        }
        Cow::from(output)
    }

    fn syllables(&self) -> Cow<[<Self::Lang as Lang>::Syllable]>;

    fn to_broad_ipa(&self) -> Cow<str>;

    fn to_narrow_ipa(&self) -> Cow<str>;
}
