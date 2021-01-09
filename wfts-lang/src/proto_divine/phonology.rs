use super::phonetics::{Context, Triggers};
use crate::phonetics::{Phone, Variation};
use std::slice;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Obstruent {
    P,
    T,
    K,
    F,
    S,
    H,
}

impl Obstruent {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sonorant {
    M,
    N,
    Ng,
    W,
    L,
    J,
}

impl Sonorant {
    pub fn orthography(&self) -> &'static str {
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

    pub fn broad_pronunc(&self) -> &'static str {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Consonant {
    Son(Sonorant),
    Obs(Obstruent),
}

impl Consonant {
    pub fn orthography(self) -> &'static str {
        use Consonant::*;

        match self {
            Son(cons) => cons.orthography(),
            Obs(cons) => cons.orthography(),
        }
    }

    pub fn broad_pronunc(self) -> &'static str {
        use Consonant::*;

        match self {
            Son(cons) => cons.broad_pronunc(),
            Obs(cons) => cons.broad_pronunc(),
        }
    }

    pub fn phonetic_triggers(self) -> Triggers {
        use Consonant::*;

        match self {
            Son(son) => son.phonetic_triggers(),
            Obs(obs) => obs.phonetic_triggers(),
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Vowel {
    Ae,
    E,
    I,
    Ao,
    O,
    U,
}

impl Vowel {
    pub fn orthography(&self) -> &'static str {
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

    pub fn broad_pronunc(&self) -> &'static str {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phoneme {
    Vowel(Vowel),
    Conson(Consonant),
}

impl Phoneme {
    pub fn orthography(self) -> &'static str {
        use Phoneme::*;

        match self {
            Conson(phoneme) => phoneme.orthography(),
            Vowel(phoneme) => phoneme.orthography(),
        }
    }

    pub fn broad_pronunc(self) -> &'static str {
        use Phoneme::*;

        match self {
            Conson(phoneme) => phoneme.broad_pronunc(),
            Vowel(phoneme) => phoneme.broad_pronunc(),
        }
    }

    pub fn phonetic_triggers(self) -> Triggers {
        use Phoneme::*;

        match self {
            Vowel(vowel) => vowel.phonetic_triggers(),
            Conson(conson) => conson.phonetic_triggers(),
        }
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Onset {
    pub outer: Option<Obstruent>,
    pub inner: Option<Sonorant>,
}

impl Onset {
    pub fn first(self) -> Option<Consonant> {
        self.outer.map(Into::into).or(self.inner.map(Into::into))
    }

    pub fn last(self) -> Option<Consonant> {
        self.inner.map(Into::into).or(self.outer.map(Into::into))
    }
}

impl IntoIterator for Onset {
    type Item = Consonant;
    type IntoIter = OnsetIter;

    fn into_iter(self) -> Self::IntoIter {
        OnsetIter { onset: self, state: OnsetIterState::Outer }
    }
}

#[derive(Debug, Clone)]
pub struct OnsetIter {
    onset: Onset,
    state: OnsetIterState,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Root {
    pub onset: Onset,
    pub nucleus: Vowel,
    pub coda: Option<Consonant>,
}

impl Root {
    pub fn first(self) -> Phoneme {
        self.onset.first().map_or(self.nucleus.into(), Into::into)
    }

    pub fn last(self) -> Phoneme {
        self.coda.map_or(self.nucleus.into(), Into::into)
    }
}

impl IntoIterator for Root {
    type Item = Phoneme;
    type IntoIter = RootIter;

    fn into_iter(self) -> Self::IntoIter {
        RootIter {
            root: self,
            front: RootIterState::OnsetOuter,
            back: RootIterState::Done,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum RootIterState {
    OnsetOuter,
    OnsetInner,
    Nucleus,
    Coda,
    Done,
}

#[derive(Debug, Clone)]
pub struct RootIter {
    root: Root,
    front: RootIterState,
    back: RootIterState,
}

impl Iterator for RootIter {
    type Item = Phoneme;

    fn next(&mut self) -> Option<Self::Item> {
        use Consonant::*;
        use Phoneme::*;

        loop {
            match self.front {
                _ if self.front == self.back => break None,
                RootIterState::Done => break None,
                RootIterState::OnsetOuter => {
                    self.front = RootIterState::OnsetInner;
                    if let Some(obs) = self.root.onset.outer {
                        break Some(Conson(Obs(obs)));
                    }
                },
                RootIterState::OnsetInner => {
                    self.front = RootIterState::Nucleus;
                    if let Some(son) = self.root.onset.inner {
                        break Some(Conson(Son(son)));
                    }
                },
                RootIterState::Nucleus => {
                    self.front = RootIterState::Coda;
                    break Some(Vowel(self.root.nucleus));
                },
                RootIterState::Coda => {
                    self.front = RootIterState::Done;
                    if let Some(cons) = self.root.coda {
                        break Some(Conson(cons));
                    }
                },
            }
        }
    }
}

impl DoubleEndedIterator for RootIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        use Consonant::*;
        use Phoneme::*;

        loop {
            match self.back {
                _ if self.front == self.back => break None,
                RootIterState::OnsetOuter => break None,
                RootIterState::OnsetInner => {
                    self.back = RootIterState::OnsetOuter;
                    if let Some(obs) = self.root.onset.outer {
                        break Some(Conson(Obs(obs)));
                    }
                },
                RootIterState::Nucleus => {
                    self.back = RootIterState::OnsetInner;
                    if let Some(son) = self.root.onset.inner {
                        break Some(Conson(Son(son)));
                    }
                },
                RootIterState::Coda => {
                    self.back = RootIterState::Nucleus;
                    break Some(Vowel(self.root.nucleus));
                },
                RootIterState::Done => {
                    self.back = RootIterState::Coda;
                    if let Some(cons) = self.root.coda {
                        break Some(Conson(cons));
                    }
                },
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Composite {
    pub head: Root,
    pub tail: Vec<Root>,
}

impl Composite {
    pub fn roots(&self) -> CompositeRoots {
        CompositeRoots { curr: Some(self.head), others: self.tail.iter() }
    }

    pub fn last(&self) -> Root {
        self.tail.last().copied().unwrap_or(self.head)
    }
}

impl<'comp> IntoIterator for &'comp Composite {
    type Item = Phoneme;
    type IntoIter = CompositeIter<'comp>;

    fn into_iter(self) -> Self::IntoIter {
        let mut others = self.tail.iter();
        CompositeIter {
            front: self.head.into_iter(),
            back: others.next_back().copied().map(Root::into_iter),
            others,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeIter<'comp> {
    front: RootIter,
    back: Option<RootIter>,
    others: slice::Iter<'comp, Root>,
}

impl<'comp> Iterator for CompositeIter<'comp> {
    type Item = Phoneme;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(phoneme) = self.front.next() {
                break Some(phoneme);
            }

            match self.others.next() {
                Some(root) => self.front = root.into_iter(),
                None => break self.back.as_mut()?.next(),
            }
        }
    }
}

impl<'comp> DoubleEndedIterator for CompositeIter<'comp> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(phoneme) = self.back.as_mut()?.next_back() {
                break Some(phoneme);
            }

            match self.others.next_back() {
                Some(root) => self.back = Some(root.into_iter()),
                None => break self.front.next_back(),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompositeRoots<'comp> {
    curr: Option<Root>,
    others: slice::Iter<'comp, Root>,
}

impl<'comp> Iterator for CompositeRoots<'comp> {
    type Item = Root;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.curr?;
        self.curr = self.others.next().copied();
        Some(curr)
    }
}

impl<'comp> DoubleEndedIterator for CompositeRoots<'comp> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let back = self.others.next_back().copied();
        if back.is_some() {
            back
        } else {
            self.curr.take()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Transcription {
    phonemes: Vec<Phoneme>,
    syl_breaks: Vec<usize>,
    word_breaks: Vec<usize>,
}

impl Transcription {
    pub fn add_phoneme(&mut self, phoneme: Phoneme) {
        self.phonemes.push(phoneme);
    }

    pub fn mark_syl_break(&mut self) {
        let index = self.phonemes.len();
        self.syl_breaks.push(index);
    }

    pub fn mark_word_break(&mut self) {
        let index = self.syl_breaks.len();
        self.word_breaks.push(index);
    }

    pub fn add_syllable(&mut self, root: Root) {
        for phoneme in root {
            self.add_phoneme(phoneme);
        }

        if self.syl_breaks.len() > 0 {
            self.mark_syl_break();
        }
    }

    pub fn add_word(&mut self, composite: Composite) {
        for root in composite.roots() {
            self.add_syllable(root);
        }

        if self.word_breaks.len() > 0 {
            self.mark_word_break();
        }
    }

    pub fn narrow_pronunc(&self) -> Variation {
        let mut ctxs = self.build_progressive();
        self.regress(&mut ctxs);
        self.make_variation(&ctxs)
    }

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

    fn make_variation(&self, ctxs: &[Context]) -> Variation {
        let mut variation = Variation::default();

        let mut syl_breaks = self.syl_breaks.iter().copied();
        let mut curr_syl_i = 0;
        let mut curr_syl_end = syl_breaks.next().unwrap_or(self.phonemes.len());

        let mut word_breaks = self.word_breaks.iter().copied();
        let mut curr_word_start = 0;
        let mut curr_word_end =
            word_breaks.next().unwrap_or(self.syl_breaks.len());

        let phonemes = self.phonemes.iter().copied();
        let ctxs_iter = ctxs.iter().copied();

        for (phoneme_i, (phoneme, ctx)) in phonemes.zip(ctxs_iter).enumerate() {
            if phoneme_i == curr_syl_end {
                curr_syl_end = syl_breaks.next().unwrap_or(self.phonemes.len());

                if curr_syl_i == curr_word_end {
                    curr_word_start = curr_word_end;
                    curr_word_end =
                        word_breaks.next().unwrap_or(self.syl_breaks.len());
                }

                curr_syl_i += 1;

                let curr_syl_in_word = curr_syl_i - curr_word_start;
                if curr_syl_in_word == 0 {
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
