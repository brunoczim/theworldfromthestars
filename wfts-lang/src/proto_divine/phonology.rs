//! This module defines phonological items related to Proto-Divine.

use crate::{
    fmt::{NarrowPronunc, WriteBroadPronunc, WriteOrthography},
    phonetics::{Phone, Variation},
    proto_divine::{
        phonetics::{Context, Triggers},
        ProtoDivine,
    },
    Lang,
};
use std::{fmt, slice};
use wfts_pedia_ssg::{component::audio::Audio, location::Location};

/// Obstruents of Proto-Divine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Obstruent {
    /// *p
    P,
    /// *t
    T,
    /// *k
    K,
    /// *f
    F,
    /// *s
    S,
    /// *h
    H,
}

impl Obstruent {
    /// Returns the reconstructed orthographic representation of this obstruent.
    pub fn orthography(self) -> &'static str {
        use Obstruent::*;

        match self {
            P => "p",
            T => "t",
            K => "k",
            F => "f",
            S => "s",
            H => "h",
        }
    }

    /// Returns the reconstructed broad phonemic pronunciation of this
    /// obstruent.
    pub fn broad_pronunc(self) -> &'static str {
        use Obstruent::*;

        match self {
            P => "p",
            T => "t",
            K => "k",
            F => "f",
            S => "s",
            H => "x",
        }
    }

    /// Returns the isolated phonetic triggers of this obstruent.
    pub fn phonetic_triggers(self) -> Triggers {
        use Obstruent::*;

        Triggers {
            voices: false,
            palatalizable: matches!(self, K | H),
            palatalizes: false,
            dissocs_palatal: false,
            dissocs_labial: false,
        }
    }

    /// Adds the phonetic variations of this obstruent to the variation object,
    /// given the phonetic context. This is the "narrow pronunciation".
    pub fn narrow_pronunc(self, variations: &mut Variation, ctx: Context) {
        use Obstruent::*;
        let phones: &[_] = match self {
            P if ctx.voiced => &[Phone::B],
            P => &[Phone::P],
            T if ctx.voiced => &[Phone::D],
            T => &[Phone::T],
            K if ctx.voiced && ctx.palatalized => &[Phone::J],
            K if ctx.voiced => &[Phone::G],
            K if ctx.palatalized => &[Phone::C],
            K => &[Phone::K],
            F if ctx.voiced => &[Phone::V, Phone::Bh],
            F => &[Phone::Ph, Phone::F],
            S if ctx.voiced => &[Phone::Z],
            S => &[Phone::S],
            H if ctx.voiced && ctx.palatalized => &[Phone::Jh],
            H if ctx.voiced => &[Phone::Gh],
            H if ctx.palatalized => &[Phone::Ch],
            H => &[Phone::X, Phone::H],
        };
        variations.add_phones(phones);
    }
}

impl WriteOrthography for Obstruent {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).orthography())
    }
}

impl WriteBroadPronunc for Obstruent {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).broad_pronunc())
    }
}

impl NarrowPronunc for Obstruent {
    fn narrow_pronunc(&self) -> Variation {
        let ctx = Context::default();
        let mut variations = Variation::default();
        (*self).narrow_pronunc(&mut variations, ctx);
        variations
    }
}

/// Sonorant consonants of Proto-Divine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sonorant {
    /// *m
    M,
    /// *n
    N,
    /// *ŋ
    Ng,
    /// *w
    W,
    /// *l
    L,
    /// *j
    J,
}

impl Sonorant {
    /// Returns the reconstructed orthographic representation of this sonorant.
    pub fn orthography(self) -> &'static str {
        use Sonorant::*;

        match self {
            M => "m",
            N => "n",
            Ng => "ŋ",
            W => "w",
            L => "l",
            J => "j",
        }
    }

    /// Returns the reconstructed broad phonemic pronunciation of this
    /// sonorant.
    pub fn broad_pronunc(self) -> &'static str {
        use Sonorant::*;

        match self {
            M => "m",
            N => "n",
            Ng => "ŋ",
            W => "w",
            L => "l",
            J => "j",
        }
    }

    /// Returns the isolated phonetic triggers of this sonorant.
    pub fn phonetic_triggers(self) -> Triggers {
        use Sonorant::*;

        Triggers {
            voices: true,
            palatalizable: matches!(self, Ng),
            palatalizes: matches!(self, J),
            dissocs_palatal: matches!(self, J),
            dissocs_labial: matches!(self, W),
        }
    }

    /// Adds the phonetic variations of this sonorant to the variation object,
    /// given the phonetic context. This is the "narrow pronunciation".
    pub fn narrow_pronunc(self, variations: &mut Variation, ctx: Context) {
        use Sonorant::*;

        let phones: &[_] = match self {
            M => &[Phone::M],
            N => &[Phone::N],
            Ng if ctx.palatalized => &[Phone::Nj],
            Ng => &[Phone::Ng],
            W => &[Phone::W, Phone::Bw, Phone::Vw],
            L => &[Phone::L, Phone::Rd],
            J => &[Phone::J],
        };
        variations.add_phones(phones)
    }
}

impl WriteOrthography for Sonorant {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).orthography())
    }
}

impl WriteBroadPronunc for Sonorant {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).broad_pronunc())
    }
}

impl NarrowPronunc for Sonorant {
    fn narrow_pronunc(&self) -> Variation {
        let ctx = Context::default();
        let mut variations = Variation::default();
        (*self).narrow_pronunc(&mut variations, ctx);
        variations
    }
}

/// A generic consonant of the Proto-Divine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Consonant {
    /// A sonorant.
    Son(Sonorant),
    /// An obstruent.
    Obs(Obstruent),
}

impl Consonant {
    /// Returns the reconstructed orthographic representation of this consonant.
    pub fn orthography(self) -> &'static str {
        use Consonant::*;

        match self {
            Son(cons) => cons.orthography(),
            Obs(cons) => cons.orthography(),
        }
    }

    /// Returns the reconstructed broad phonemic pronunciation of this
    /// consonant.
    pub fn broad_pronunc(self) -> &'static str {
        use Consonant::*;

        match self {
            Son(cons) => cons.broad_pronunc(),
            Obs(cons) => cons.broad_pronunc(),
        }
    }

    /// Returns the isolated phonetic triggers of this consonant.
    pub fn phonetic_triggers(self) -> Triggers {
        use Consonant::*;

        match self {
            Son(son) => son.phonetic_triggers(),
            Obs(obs) => obs.phonetic_triggers(),
        }
    }

    /// Adds the phonetic variations of this consonant to the variation object,
    /// given the phonetic context. This is the "narrow pronunciation".
    pub fn narrow_pronunc(self, variations: &mut Variation, ctx: Context) {
        use Consonant::*;

        match self {
            Son(son) => son.narrow_pronunc(variations, ctx),
            Obs(obs) => obs.narrow_pronunc(variations, ctx),
        }
    }
}

impl From<Sonorant> for Consonant {
    fn from(phoneme: Sonorant) -> Self {
        Consonant::Son(phoneme)
    }
}

impl From<Obstruent> for Consonant {
    fn from(phoneme: Obstruent) -> Self {
        Consonant::Obs(phoneme)
    }
}

impl WriteOrthography for Consonant {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).orthography())
    }
}

impl WriteBroadPronunc for Consonant {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).broad_pronunc())
    }
}

impl NarrowPronunc for Consonant {
    fn narrow_pronunc(&self) -> Variation {
        let ctx = Context::default();
        let mut variations = Variation::default();
        (*self).narrow_pronunc(&mut variations, ctx);
        variations
    }
}

/// The vowels of Proto-Divine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Vowel {
    /// *a
    Ae,
    /// *e
    E,
    /// *i
    I,
    /// *å
    Ao,
    /// *o
    O,
    /// *u
    U,
}

impl Vowel {
    /// Returns the reconstructed orthographic representation of this vowel.
    pub fn orthography(self) -> &'static str {
        use Vowel::*;

        match self {
            Ae => "a",
            E => "e",
            I => "i",
            Ao => "å",
            O => "o",
            U => "u",
        }
    }

    /// Returns the reconstructed broad phonemic pronunciation of this
    /// vowel.
    pub fn broad_pronunc(self) -> &'static str {
        use Vowel::*;

        match self {
            Ae => "a",
            E => "e",
            I => "i",
            Ao => "ɒ",
            O => "o",
            U => "u",
        }
    }

    /// Returns the isolated phonetic triggers of this vowel.
    pub fn phonetic_triggers(self) -> Triggers {
        use Vowel::*;

        Triggers {
            voices: true,
            palatalizable: false,
            palatalizes: matches!(self, Ae | E | I),
            dissocs_palatal: false,
            dissocs_labial: false,
        }
    }

    /// Adds the phonetic variations of this vowel to the variation object,
    /// given the phonetic context. This is the "narrow pronunciation".
    pub fn narrow_pronunc(self, variations: &mut Variation, ctx: Context) {
        use self::Vowel::*;

        let phones: &[_] = match self {
            Ae if ctx.palatal_dissoc => &[Phone::A],
            Ae => &[Phone::Ae, Phone::A],
            E if ctx.palatal_dissoc => &[Phone::EMid],
            E => &[Phone::E, Phone::EMid],
            I if ctx.palatal_dissoc => &[Phone::IMidCent],
            I => &[Phone::I],
            Ao if ctx.labial_dissoc => &[Phone::Ao],
            Ao => &[Phone::Ao, Phone::AoRaised],
            O if ctx.labial_dissoc => &[Phone::OMid],
            O => &[Phone::O, Phone::OMid],
            U if ctx.labial_dissoc => &[Phone::UMidCent],
            U => &[Phone::U],
        };
        variations.add_phones(phones);
    }
}

impl WriteOrthography for Vowel {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).orthography())
    }
}

impl WriteBroadPronunc for Vowel {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).broad_pronunc())
    }
}

impl NarrowPronunc for Vowel {
    fn narrow_pronunc(&self) -> Variation {
        let ctx = Context::default();
        let mut variations = Variation::default();
        (*self).narrow_pronunc(&mut variations, ctx);
        variations
    }
}

/// A generic phoneme of the Proto-Divine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phoneme {
    /// A vowel phoneme.
    Vowel(Vowel),
    /// A consonant phoneme.
    Conson(Consonant),
}

impl Phoneme {
    /// Returns the reconstructed orthographic representation of this phoneme.
    pub fn orthography(self) -> &'static str {
        use Phoneme::*;

        match self {
            Conson(phoneme) => phoneme.orthography(),
            Vowel(phoneme) => phoneme.orthography(),
        }
    }

    /// Returns the reconstructed broad phonemic pronunciation of this
    /// phoneme.
    pub fn broad_pronunc(self) -> &'static str {
        use Phoneme::*;

        match self {
            Conson(phoneme) => phoneme.broad_pronunc(),
            Vowel(phoneme) => phoneme.broad_pronunc(),
        }
    }

    /// Returns the isolated phonetic triggers of this phoneme.
    pub fn phonetic_triggers(self) -> Triggers {
        use Phoneme::*;

        match self {
            Vowel(vowel) => vowel.phonetic_triggers(),
            Conson(conson) => conson.phonetic_triggers(),
        }
    }

    /// Adds the phonetic variations of this phoneme to the variation object,
    /// given the phonetic context. This is the "narrow pronunciation".
    pub fn narrow_pronunc(self, variations: &mut Variation, ctx: Context) {
        use Phoneme::*;

        match self {
            Vowel(vowel) => vowel.narrow_pronunc(variations, ctx),
            Conson(conson) => conson.narrow_pronunc(variations, ctx),
        }
    }
}

impl From<Consonant> for Phoneme {
    fn from(phoneme: Consonant) -> Self {
        Phoneme::Conson(phoneme)
    }
}

impl From<Vowel> for Phoneme {
    fn from(phoneme: Vowel) -> Self {
        Phoneme::Vowel(phoneme)
    }
}

impl From<Sonorant> for Phoneme {
    fn from(phoneme: Sonorant) -> Self {
        Phoneme::from(Consonant::from(phoneme))
    }
}

impl From<Obstruent> for Phoneme {
    fn from(phoneme: Obstruent) -> Self {
        Phoneme::from(Consonant::from(phoneme))
    }
}

impl WriteOrthography for Phoneme {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).orthography())
    }
}

impl WriteBroadPronunc for Phoneme {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad((*self).broad_pronunc())
    }
}

impl NarrowPronunc for Phoneme {
    fn narrow_pronunc(&self) -> Variation {
        let ctx = Context::default();
        let mut variations = Variation::default();
        (*self).narrow_pronunc(&mut variations, ctx);
        variations
    }
}

/// The onset of a syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Onset {
    /// Outer part of the onset (the obstruent).
    pub outer: Option<Obstruent>,
    /// Inner part of the onset (the sonorant).
    pub inner: Option<Sonorant>,
}

impl IntoIterator for Onset {
    type Item = Consonant;
    type IntoIter = OnsetIter;

    fn into_iter(self) -> Self::IntoIter {
        OnsetIter { onset: self, state: OnsetIterState::Outer }
    }
}

impl WriteOrthography for Onset {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(outer) = self.outer {
            (&outer).orthography_fmt(fmt)?;
        }
        if let Some(inner) = self.inner {
            (&inner).orthography_fmt(fmt)?;
        }
        Ok(())
    }
}

impl WriteBroadPronunc for Onset {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if let Some(outer) = self.outer {
            (&outer).broad_pronunc_fmt(fmt)?;
        }
        if let Some(inner) = self.inner {
            (&inner).broad_pronunc_fmt(fmt)?;
        }
        Ok(())
    }
}

impl NarrowPronunc for Onset {
    fn narrow_pronunc(&self) -> Variation {
        let mut transcription = Transcription::default();
        transcription.make_unstressed();

        if let Some(outer) = self.outer {
            transcription.add_phoneme(outer.into());
        }
        if let Some(inner) = self.inner {
            transcription.add_phoneme(inner.into());
        }
        transcription.narrow_pronunc()
    }
}

/// Iterator over the onset phonemes (0 to 2 phonemes).
#[derive(Debug, Clone)]
pub struct OnsetIter {
    onset: Onset,
    state: OnsetIterState,
}

/// Internal state of the onset iterator.
#[derive(Debug, Clone)]
enum OnsetIterState {
    Outer,
    Inner,
    Done,
}

impl Iterator for OnsetIter {
    type Item = Consonant;

    fn next(&mut self) -> Option<Self::Item> {
        use Consonant::*;

        loop {
            match self.state {
                OnsetIterState::Outer => {
                    self.state = OnsetIterState::Inner;
                    if let Some(obs) = self.onset.outer {
                        break Some(Obs(obs));
                    }
                },
                OnsetIterState::Inner => {
                    self.state = OnsetIterState::Done;
                    if let Some(son) = self.onset.inner {
                        break Some(Son(son));
                    }
                },
                OnsetIterState::Done => break None,
            }
        }
    }
}

/// A morpheme of the Proto-Divine language; a single syllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Syllable {
    /// Onset of the morpheme/syllable.
    pub onset: Onset,
    /// Nucleus (vowel) of the morpheme/syllable.
    pub nucleus: Vowel,
    /// Optiona coda of the morpheme/syllable.
    pub coda: Option<Consonant>,
}

impl Syllable {
    pub fn parse(phonemes: &[Phoneme]) -> Result<Self, &[Phoneme]> {
        use Consonant::*;
        use Phoneme::*;

        let mut iter = phonemes.iter();

        let mut phoneme = iter.next();
        let mut onset = Onset { outer: None, inner: None };

        if let Some(Conson(Obs(obstruent))) = phoneme {
            onset.outer = Some(*obstruent);
            phoneme = iter.next();
        }

        if let Some(Conson(Son(sonorant))) = phoneme {
            onset.inner = Some(*sonorant);
            phoneme = iter.next();
        }

        let nucleus = match phoneme {
            Some(Vowel(vowel)) => {
                phoneme = iter.next();
                *vowel
            },
            _ => return Err(iter.as_slice()),
        };

        let mut coda = None;

        if let Some(Conson(consonant)) = phoneme {
            coda = Some(*consonant);
        }

        if iter.as_slice().len() == 0 {
            Ok(Self { onset, nucleus, coda })
        } else {
            Err(iter.as_slice())
        }
    }
}

impl IntoIterator for Syllable {
    type Item = Phoneme;
    type IntoIter = SyllableIter;

    fn into_iter(self) -> Self::IntoIter {
        SyllableIter {
            syllable: self,
            front: SyllableIterState::OnsetOuter,
            back: SyllableIterState::Done,
        }
    }
}

impl WriteOrthography for Syllable {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for phoneme in *self {
            (&phoneme).orthography_fmt(fmt)?;
        }
        Ok(())
    }
}

impl WriteBroadPronunc for Syllable {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for phoneme in *self {
            (&phoneme).broad_pronunc_fmt(fmt)?;
        }
        Ok(())
    }
}

impl NarrowPronunc for Syllable {
    fn narrow_pronunc(&self) -> Variation {
        let mut trans = Transcription::default();
        trans.add_syllable(*self);
        trans.narrow_pronunc()
    }
}

/// Internal state of the syllable iterator.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SyllableIterState {
    OnsetOuter,
    OnsetInner,
    Nucleus,
    Coda,
    Done,
}

/// Iterator over the syllable phonemes (1 to 4 phonemes).
#[derive(Debug, Clone)]
pub struct SyllableIter {
    syllable: Syllable,
    front: SyllableIterState,
    back: SyllableIterState,
}

impl Iterator for SyllableIter {
    type Item = Phoneme;

    fn next(&mut self) -> Option<Self::Item> {
        use Consonant::*;
        use Phoneme::*;

        loop {
            match self.front {
                _ if self.front == self.back => break None,
                SyllableIterState::Done => break None,
                SyllableIterState::OnsetOuter => {
                    self.front = SyllableIterState::OnsetInner;
                    if let Some(obs) = self.syllable.onset.outer {
                        break Some(Conson(Obs(obs)));
                    }
                },
                SyllableIterState::OnsetInner => {
                    self.front = SyllableIterState::Nucleus;
                    if let Some(son) = self.syllable.onset.inner {
                        break Some(Conson(Son(son)));
                    }
                },
                SyllableIterState::Nucleus => {
                    self.front = SyllableIterState::Coda;
                    break Some(Vowel(self.syllable.nucleus));
                },
                SyllableIterState::Coda => {
                    self.front = SyllableIterState::Done;
                    if let Some(cons) = self.syllable.coda {
                        break Some(Conson(cons));
                    }
                },
            }
        }
    }
}

impl DoubleEndedIterator for SyllableIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        use Consonant::*;
        use Phoneme::*;

        loop {
            match self.back {
                _ if self.front == self.back => break None,
                SyllableIterState::OnsetOuter => break None,
                SyllableIterState::OnsetInner => {
                    self.back = SyllableIterState::OnsetOuter;
                    if let Some(obs) = self.syllable.onset.outer {
                        break Some(Conson(Obs(obs)));
                    }
                },
                SyllableIterState::Nucleus => {
                    self.back = SyllableIterState::OnsetInner;
                    if let Some(son) = self.syllable.onset.inner {
                        break Some(Conson(Son(son)));
                    }
                },
                SyllableIterState::Coda => {
                    self.back = SyllableIterState::Nucleus;
                    break Some(Vowel(self.syllable.nucleus));
                },
                SyllableIterState::Done => {
                    self.back = SyllableIterState::Coda;
                    if let Some(cons) = self.syllable.coda {
                        break Some(Conson(cons));
                    }
                },
            }
        }
    }
}

/// A word, either a single morpheme/syllable, or a compound of
/// morphemes/syllables.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word {
    /// Head/first syllable/morpheme of this composite word.
    pub head: Syllable,
    /// The rest of the syllables/morphemes of this composite word.
    pub tail: Vec<Syllable>,
}

impl Word {
    pub fn parse<'phonemes>(
        phonemes: &[&'phonemes [Phoneme]],
    ) -> Result<Self, &'phonemes [Phoneme]> {
        match phonemes.split_first() {
            Some((head, tail)) => Ok(Self {
                head: Syllable::parse(head)?,
                tail: tail
                    .iter()
                    .copied()
                    .map(Syllable::parse)
                    .collect::<Result<Vec<_>, _>>()?,
            }),

            None => Err(&[]),
        }
    }

    /// Iterator over the syllables of this word.
    pub fn syllables(&self) -> WordSyllables {
        WordSyllables { curr: Some(self.head), others: self.tail.iter() }
    }

    /// Returns the last syllable of this word.
    pub fn last(&self) -> Syllable {
        self.tail.last().copied().unwrap_or(self.head)
    }

    pub fn audio(&self) -> Option<Audio> {
        let orthography = self.orthography().to_string();

        let has_audio = matches!(&*orthography, _);

        if has_audio {
            let location = Location::internal(format!(
                "{}/audio/{}.ogg",
                ProtoDivine.path(),
                orthography
            ));
            Some(Audio(location))
        } else {
            None
        }
    }
}

impl<'comp> IntoIterator for &'comp Word {
    type Item = Phoneme;
    type IntoIter = WordIter<'comp>;

    fn into_iter(self) -> Self::IntoIter {
        let mut others = self.tail.iter();
        WordIter {
            front: self.head.into_iter(),
            back: others.next_back().copied().map(Syllable::into_iter),
            others,
        }
    }
}

impl WriteOrthography for Word {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (i, syllable) in self.syllables().enumerate() {
            if i > 0 {
                fmt.write_str("-")?;
            }
            (&syllable).orthography_fmt(fmt)?;
        }
        Ok(())
    }
}

impl WriteBroadPronunc for Word {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for (i, syllable) in self.syllables().enumerate() {
            fmt.write_str(match i {
                0 => "ˈ",
                _ if i % 2 == 0 => "ˌ",
                _ => ".",
            })?;
            (&syllable).broad_pronunc_fmt(fmt)?;
        }
        Ok(())
    }
}

impl NarrowPronunc for Word {
    fn narrow_pronunc(&self) -> Variation {
        let mut trans = Transcription::default();
        trans.add_word(self);
        trans.narrow_pronunc()
    }
}

/// Iterator over the phonemes of a composite word.
#[derive(Debug, Clone)]
pub struct WordIter<'comp> {
    front: SyllableIter,
    back: Option<SyllableIter>,
    others: slice::Iter<'comp, Syllable>,
}

impl<'comp> Iterator for WordIter<'comp> {
    type Item = Phoneme;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(phoneme) = self.front.next() {
                break Some(phoneme);
            }

            match self.others.next() {
                Some(syllable) => self.front = syllable.into_iter(),
                None => break self.back.as_mut()?.next(),
            }
        }
    }
}

impl<'comp> DoubleEndedIterator for WordIter<'comp> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(phoneme) = self.back.as_mut()?.next_back() {
                break Some(phoneme);
            }

            match self.others.next_back() {
                Some(syllable) => self.back = Some(syllable.into_iter()),
                None => break self.front.next_back(),
            }
        }
    }
}

/// Iterator over the syllables of a (possibly composite) word.
#[derive(Debug, Clone)]
pub struct WordSyllables<'comp> {
    curr: Option<Syllable>,
    others: slice::Iter<'comp, Syllable>,
}

impl<'comp> Iterator for WordSyllables<'comp> {
    type Item = Syllable;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr?;
        self.curr = self.others.next().copied();
        Some(curr)
    }
}

impl<'comp> DoubleEndedIterator for WordSyllables<'comp> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let back = self.others.next_back().copied();
        if back.is_some() {
            back
        } else {
            self.curr.take()
        }
    }
}

/// Phonemic transcriber of a speech, can be used to produce a phonetic
/// transcription.
#[derive(Debug, Clone)]
pub struct Transcription {
    phonemes: Vec<Phoneme>,
    syl_breaks: Vec<usize>,
    word_breaks: Vec<usize>,
    stressed: bool,
}

impl Default for Transcription {
    fn default() -> Self {
        Transcription {
            phonemes: Vec::new(),
            syl_breaks: Vec::new(),
            word_breaks: Vec::new(),
            stressed: true,
        }
    }
}

impl Transcription {
    /// Adds a single phoneme to the transcription.
    pub fn add_phoneme(&mut self, phoneme: Phoneme) {
        self.phonemes.push(phoneme);
    }

    pub fn make_unstressed(&mut self) {
        self.stressed = false;
    }

    /// Marks a syllable break at the current position. The first syllable
    /// implicitly starts at the beginning of the word.
    pub fn mark_syl_break(&mut self) {
        let index = self.phonemes.len();
        self.syl_breaks.push(index);
    }

    /// Marks a word break at the current position. The first word
    /// implicitly starts at the beginning of the transcription.
    pub fn mark_word_break(&mut self) {
        let index = self.syl_breaks.len();
        self.word_breaks.push(index);
    }

    /// Adds a whole syllable (a morpheme), automatically marking syllable
    /// break. Word break not marked.
    pub fn add_syllable(&mut self, syllable: Syllable) {
        let needs_break = self.phonemes.len() > 0;

        if needs_break {
            self.mark_syl_break();
        }

        for phoneme in syllable {
            self.add_phoneme(phoneme);
        }
    }

    /// Adds a whole word (a composite, perhaps a monosyllabic word),
    /// automatically marking word break and necessary syllable breaks.
    pub fn add_word(&mut self, word: &Word) {
        let needs_break = self.phonemes.len() > 0;

        if needs_break {
            self.mark_word_break();
        }

        for syllable in word.syllables() {
            self.add_syllable(syllable);
        }
    }

    /// Performs a narrow phonetic transcription in terms of pronunciation
    /// variation, i.e. performs narrow pronunciation.
    pub fn narrow_pronunc(&self) -> Variation {
        let mut ctxs = self.build_progressive();
        self.regress(&mut ctxs);
        self.make_variation(&ctxs)
    }

    /// Internal use: builds the first attempt of phonetic contexts to
    /// transcribe phonetically, and progressively, i.e. from the first to the
    /// last.
    fn build_progressive(&self) -> Vec<Context> {
        let mut ctxs = Vec::new();
        let mut prev_trigger = Triggers::default();
        let mut phonemes_iter = self.phonemes.iter();

        let mut curr_phoneme = match phonemes_iter.next() {
            Some(curr) => curr,
            None => return ctxs,
        };

        for next_phoneme in phonemes_iter {
            let curr_ctx = Context::from_triggers(
                prev_trigger,
                next_phoneme.phonetic_triggers(),
            );
            ctxs.push(curr_ctx);
            prev_trigger = curr_phoneme.phonetic_triggers().with_ctx(curr_ctx);
            curr_phoneme = next_phoneme;
        }

        let next_trigger = Triggers::default();
        let curr_ctx = Context::from_triggers(prev_trigger, next_trigger);
        ctxs.push(curr_ctx);

        ctxs
    }

    /// Internal use: corrects the first attempt of phonetic contexts in order
    /// to take into account previously uncomputed neighbour contexts, and
    /// regressively, i.e. from the last to the first.
    fn regress(&self, ctxs: &mut [Context]) {
        let phonemes_iter = self.phonemes.iter().rev();
        let ctxs_iter = ctxs.iter_mut().rev();

        let mut zipped_iter = phonemes_iter.zip(ctxs_iter);

        let mut next_trigger = Triggers::default();
        let (mut curr_phoneme, mut curr_ctx) = match zipped_iter.next() {
            Some(pair) => pair,
            None => return,
        };

        for (prev_phoneme, prev_ctx) in zipped_iter {
            let prev_trigger =
                prev_phoneme.phonetic_triggers().with_ctx(*prev_ctx);
            *curr_ctx = Context::from_triggers(prev_trigger, next_trigger);
            next_trigger = curr_phoneme.phonetic_triggers().with_ctx(*curr_ctx);
            curr_ctx = prev_ctx;
            curr_phoneme = prev_phoneme;
        }

        let prev_trigger = Triggers::default();
        *curr_ctx = Context::from_triggers(prev_trigger, next_trigger);
    }

    /// Internal use: after building and correcting contexts, finally builds a
    /// phonetic variation of pronunciation.
    fn make_variation(&self, ctxs: &[Context]) -> Variation {
        let mut variation = Variation::default();

        let mut syl_breaks = self.syl_breaks.iter().copied();
        let mut curr_syl_start = 0;
        let mut curr_syl_i = 0;
        let mut curr_syl_end = syl_breaks.next().unwrap_or(self.phonemes.len());

        let mut word_breaks = self.word_breaks.iter().copied();
        let mut curr_word_start = 0;
        let mut curr_word_end =
            word_breaks.next().unwrap_or(self.syl_breaks.len()) + 1;

        let phonemes = self.phonemes.iter().copied();
        let ctxs_iter = ctxs.iter().copied();

        for (phoneme_i, (phoneme, ctx)) in phonemes.zip(ctxs_iter).enumerate() {
            if phoneme_i == curr_syl_end {
                curr_syl_start = curr_syl_end;
                curr_syl_end = syl_breaks.next().unwrap_or(self.phonemes.len());

                curr_syl_i += 1;

                if curr_syl_i == curr_word_end {
                    curr_word_start = curr_word_end;
                    curr_word_end =
                        word_breaks.next().unwrap_or(self.syl_breaks.len()) + 1;
                }
            }

            if phoneme_i == curr_syl_start {
                let curr_syl_in_word = curr_syl_i - curr_word_start;
                if curr_syl_in_word == 0 && self.stressed {
                    variation.add_phones(&[Phone::Stress]);
                } else if curr_syl_in_word % 2 == 0 {
                    variation.add_phones(&[Phone::SecStress]);
                } else {
                    variation.add_phones(&[Phone::SylBreak]);
                }
            }

            phoneme.narrow_pronunc(&mut variation, ctx);
        }

        variation
    }
}

/// Pronounce phonetically an iterator of composite words as a sequence of
/// words, using phonetic variation of pronunciation.
pub fn pronounce_words<'word, I>(words: I) -> Variation
where
    I: IntoIterator<Item = &'word Word>,
{
    let mut transcription = Transcription::default();
    for word in words {
        transcription.add_word(word);
    }
    transcription.narrow_pronunc()
}
