//! This module exports phonetics utilites related to any language.

use std::fmt;

/// Phones possibly used by any language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phone {
    /// Open front unrounded vowel.
    A,
    /// Near-open front unrounded vowel.
    Ae,
    /// Open back rounded vowel.
    Ao,
    /// Near-open back rounded vowel.
    AoRaised,
    /// Close-mid front unrounded vowel.
    E,
    /// True-mid front unrounded vowel.
    EMid,
    /// Close front unrounded vowel.
    I,
    /// Near-close near-front unrounded vowel.
    IMidCent,
    /// Close-mid back rounded vowel.
    O,
    /// True-mid back rounded vowel.
    OMid,
    /// Close back rounded vowel.
    U,
    /// Near-close near-back rounded vowel.
    UMidCent,
    /// Non-syllabic close front unrounded vowel.
    NonSylI,
    /// Voiceless tenuis (or voiceless undefined) bilabial stop.
    P,
    /// Voiced bilabial stop.
    B,
    /// Voiceless tenuis (or voiceless undefined) alveolar stop.
    T,
    /// Voiced alveolar stop.
    D,
    /// Voiceless tenuis (or voiceless undefined) palatal top.
    C,
    /// Voiced palatal stop.
    Gj,
    /// Voiceless tenuis (or voiceless undefined) velar stop.
    K,
    /// Voiced velar stop.
    G,
    /// Voiceless labiodental fricative.
    F,
    /// Voiceless bilabial fricative.
    Ph,
    /// Voiced labiodental fricative.
    V,
    /// Voiced bilabial fricative.
    Bh,
    /// Voiceless alveolar fricative.
    S,
    /// Voiced alveolar fricative.
    Z,
    /// Voiceless palatal fricative.
    Ch,
    /// Voiced palatal fricative.
    Jh,
    /// Voiceless velar fricative.
    X,
    /// Voiced velar fricative.
    Gh,
    /// Voiceless glottal transition.
    H,
    /// Voiced bilabial nasal.
    M,
    /// Voiced alveolar nasal.
    N,
    /// Voiced palatal nasal.
    Nj,
    /// Voiced velar nasal.
    Ng,
    /// Voiced labiovelar approximant.
    W,
    /// Voiced bilabial approximant.
    Bw,
    /// Voiced labiodental approximant.
    Vw,
    /// Voiced lateral alveolar approximant.
    L,
    /// Voiced alveolar trill.
    R,
    /// Voiced alveolar tap/flap.
    Rd,
    /// Voiced palatal approximant.
    J,
    /// Syllable break.
    SylBreak,
    /// Primary stress.
    Stress,
    /// Secondary stress.
    SecStress,
    /// Hyphen ('-').
    Hyphen,
}

impl fmt::Display for Phone {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            Phone::A => "a",
            Phone::Ae => "æ",
            Phone::Ao => "ɒ",
            Phone::AoRaised => "ɒ̝",
            Phone::E => "e",
            Phone::EMid => "e̞",
            Phone::I => "i",
            Phone::IMidCent => "ɪ",
            Phone::O => "o",
            Phone::OMid => "o̞",
            Phone::U => "u",
            Phone::UMidCent => "ʊ",
            Phone::NonSylI => "i̯",
            Phone::P => "p",
            Phone::B => "b",
            Phone::T => "t",
            Phone::D => "d",
            Phone::C => "c",
            Phone::Gj => "ɟ",
            Phone::K => "k",
            Phone::G => "g",
            Phone::F => "f",
            Phone::Ph => "ɸ",
            Phone::V => "v",
            Phone::Bh => "β",
            Phone::S => "s",
            Phone::Z => "z",
            Phone::Ch => "ç",
            Phone::Jh => "ʝ",
            Phone::X => "x",
            Phone::Gh => "ɣ",
            Phone::H => "h",
            Phone::M => "m",
            Phone::N => "n",
            Phone::Nj => "ɲ",
            Phone::Ng => "ŋ",
            Phone::W => "w",
            Phone::Bw => "β̞",
            Phone::Vw => "ʋ",
            Phone::L => "l",
            Phone::R => "ɹ",
            Phone::Rd => "ɾ",
            Phone::J => "j",
            Phone::SylBreak => ".",
            Phone::Stress => "ˈ",
            Phone::SecStress => "ˌ",
            Phone::Hyphen => "-",
        })
    }
}

/// A single, narrow pronunciation of a piece of speech.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Pronunc {
    root: Vec<Phone>,
}

/// A variation of narrow pronunciations.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variation {
    pronuncs: Vec<Pronunc>,
}

impl Default for Variation {
    fn default() -> Self {
        Self { pronuncs: vec![Pronunc::default()] }
    }
}

impl Variation {
    /// Adds a list of variable phones of this pronunciation variables. Each
    /// element is a different pronunciation, variating between each other. A
    /// cartesian product is performed between each existing variation and a
    /// new phone to be added.
    pub fn add_phones(&mut self, phones: &[Phone]) {
        let capacity = phones.len() * (self.pronuncs.len() + 1);
        let mut new_pronuncs = Vec::with_capacity(capacity);
        for &phone in phones {
            for pronunc in &self.pronuncs {
                let mut pronunc = pronunc.clone();
                pronunc.root.push(phone);
                new_pronuncs.push(pronunc);
            }
        }
        self.pronuncs = new_pronuncs;
    }

    /// Adds a list of variable phones sequences of this pronunciation
    /// variables. Like [`Variation::add_phones`], but instead of dealing of
    /// indiviudal [`Phone`]s, it deals with sequences of phones. So, each
    /// element is a different pronunciation, variating between each other. A
    /// cartesian product is performed between each existing variation and a
    /// new phone sequence to be added.
    pub fn add_phone_seqs(&mut self, seqs: &[&[Phone]]) {
        let capacity = seqs.len() * (self.pronuncs.len() + 1);
        let mut new_pronuncs = Vec::with_capacity(capacity);
        for &seq in seqs {
            for pronunc in &self.pronuncs {
                let mut pronunc = pronunc.clone();
                pronunc.root.extend_from_slice(seq);
                new_pronuncs.push(pronunc);
            }
        }
        self.pronuncs = new_pronuncs;
    }
}
