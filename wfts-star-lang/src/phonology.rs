use crate::StarLang;
use anyhow::Context;
use std::{borrow::Cow, cmp::Ordering, fmt, iter};
use thiserror::Error;
use wfts_lang::Lang;
use wfts_pedia_ssg::{component::audio::Audio, location::Location};

pub fn balance_cluster(coda: &mut Coda, onset: &mut Onset) {
    let done = matches!(
        onset.phonemes().next().map(Phoneme::classify),
        Some(PhonemeClass::Aspirated) | Some(PhonemeClass::Ejective)
    );

    if !done {
        match onset.phonemes().count().checked_sub(coda.phonemes().count()) {
            Some(diff) if diff > 1 => {
                coda.outer = onset.outer.take().or_else(|| onset.medial.take());
            },
            Some(_) => (),
            None => {
                if onset.outer.is_none() {
                    if let Some(outer) = coda.outer {
                        if outer.classify() == PhonemeClass::Fricative {
                            onset.outer = coda.outer.take();
                        } else if !onset.medial.is_some() {
                            onset.medial = coda.outer.take();
                        }
                    } else if coda.inner.is_some()
                        && onset.phonemes().next().is_none()
                    {
                        onset.inner = coda.inner.take();
                    }
                }
            },
        }
    }
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(phonemes: &[Phoneme]) -> anyhow::Result<Self>;

    fn parse_str<S>(contents: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        contents
            .as_ref()
            .chars()
            .map(Phoneme::new)
            .collect::<anyhow::Result<Vec<_>>>()
            .and_then(|phonemes| Self::parse(&phonemes[..]))
            .with_context(|| contents.as_ref().to_owned())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    syllables: Vec<Syllable>,
}

impl Word {
    pub fn new(syllables: Vec<Syllable>) -> anyhow::Result<Self> {
        use PhonemeClass::*;

        let mut prev = None;
        let mut prev_coda_len = 0;

        if syllables.len() == 0 {
            Err(InvalidWord { syllables: syllables.clone() })?;
        }

        for syllable in &syllables {
            let mut iter = syllable.phonemes();
            let first = iter.next().unwrap();
            let last = iter.last();
            let onset_len = syllable.onset().phonemes().count();
            if let Some(prev) = prev {
                let bypass_dist =
                    matches!(first.classify(), Aspirated | Ejective);

                let wrong_dist = onset_len
                    .checked_sub(prev_coda_len)
                    .map_or(true, |diff| diff > 1);

                if prev == first || !bypass_dist && wrong_dist {
                    Err(InvalidWord { syllables: syllables.clone() })?;
                }
            }
            prev = Some(last.unwrap_or(first));
            prev_coda_len = syllable.coda().phonemes().count();
        }

        Ok(Self { syllables })
    }

    pub fn phonemes<'this>(
        &'this self,
    ) -> impl DoubleEndedIterator<Item = Phoneme> + 'this {
        self.syllables.iter().flat_map(Syllable::phonemes)
    }

    pub fn syllables(&self) -> &[Syllable] {
        &self.syllables
    }

    pub fn to_broad_ipa(&self) -> String {
        let mut output = String::from("ˈ");
        let mut first = true;

        for syllable in &self.syllables {
            if first {
                first = false;
            } else {
                output.push('.');
            }

            for phoneme in syllable.phonemes() {
                output.push_str(&phoneme.to_broad_ipa())
            }
        }

        output
    }

    pub fn to_early_narrow_ipa(&self) -> String {
        let mut output = String::from("ˈ");
        let mut prev = None;
        let mut last = None;

        for syllable in &self.syllables {
            let mut iter = syllable.phonemes();
            let mut curr = last.or_else(|| iter.next()).unwrap();
            let mut first = true;
            for next in iter {
                output.push_str(curr.to_narrow_ipa(prev, Some(next), false));
                if first {
                    if last.is_some() {
                        output.push('.');
                    }
                    first = false;
                }
                prev = Some(curr);
                curr = next;
            }
            last = Some(curr);
        }
        output.push_str(last.unwrap().to_narrow_ipa(prev, None, false));

        output
    }

    pub fn to_late_narrow_ipa(&self) -> String {
        let is_palatal = self.map_patalization();
        let mut output = String::from("ˈ");

        let mut prev = None;
        let mut last = None;
        let mut i = 0;
        for syllable in &self.syllables {
            let mut iter = syllable.phonemes();
            let mut curr = last.or_else(|| iter.next()).unwrap();
            let mut first = true;
            for next in iter {
                output.push_str(curr.to_narrow_ipa(
                    prev,
                    Some(next),
                    is_palatal[i],
                ));
                if first {
                    if last.is_some() {
                        output.push('.');
                    }
                    first = false;
                }
                prev = Some(curr);
                curr = next;
                i += 1;
            }
            last = Some(curr);
        }
        output.push_str(last.unwrap().to_narrow_ipa(prev, None, is_palatal[i]));

        output
    }

    fn map_patalization(&self) -> Vec<bool> {
        let mut is_palatal = Vec::new();
        let mut prev = None;
        for phoneme in self.phonemes() {
            let curr_palatal = if let Some(prev) = prev {
                let can_be = phoneme.can_be_palatalized_progress(prev);
                let prev_palatal = is_palatal.last().cloned().unwrap_or(false);
                let curr_palatal = phoneme.is_palatal();
                can_be && prev_palatal || curr_palatal
            } else {
                false
            };
            is_palatal.push(curr_palatal);
            prev = Some(phoneme);
        }

        let mut prev = None;
        for (i, phoneme) in self.phonemes().rev().enumerate() {
            let i = is_palatal.len() - 1 - i;
            let can_be = phoneme.can_be_palatalized_regress();
            let prev_palatal = prev.map_or(false, |is| is);
            if can_be && prev_palatal {
                is_palatal[i] = true;
            }
            prev = Some(is_palatal[i]);
        }

        is_palatal
    }

    pub fn to_text(&self) -> String {
        format!("{}", self)
    }

    pub fn replace_initial_onset(&self, onset: Onset) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let first = syllables.first_mut().unwrap();
        *first = Syllable::new(onset, first.nucleus(), first.coda())?;
        Self::new(syllables)
    }

    pub fn replace_initial_coda(&self, coda: Coda) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let (first, rest) = syllables.split_first_mut().unwrap();
        *first = Syllable::new(first.onset(), first.nucleus(), coda)?;
        if let Some(second) = rest.first_mut() {
            balance_cluster(&mut first.coda, &mut second.onset);
        }
        Self::new(syllables)
    }

    pub fn replace_initial_rhyme(
        &self,
        nucleus: Phoneme,
        coda: Coda,
    ) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let (first, rest) = syllables.split_first_mut().unwrap();
        *first = Syllable::new(first.onset(), nucleus, coda)?;
        if let Some(second) = rest.first_mut() {
            balance_cluster(&mut first.coda, &mut second.onset);
        }
        Self::new(syllables)
    }

    pub fn replace_initial_nucleus(
        &self,
        nucleus: Phoneme,
    ) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let first = syllables.first_mut().unwrap();
        *first = Syllable::new(first.onset(), nucleus, first.coda())?;
        Self::new(syllables)
    }

    pub fn replace_final_onset(&self, onset: Onset) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let (last, init) = syllables.split_last_mut().unwrap();
        *last = Syllable::new(onset, last.nucleus(), last.coda())?;
        if let Some(before_last) = init.last_mut() {
            balance_cluster(&mut before_last.coda, &mut last.onset);
        }
        Self::new(syllables)
    }

    pub fn replace_final_coda(&self, coda: Coda) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let last = syllables.last_mut().unwrap();
        *last = Syllable::new(last.onset(), last.nucleus(), coda)?;
        Self::new(syllables)
    }

    pub fn replace_final_rhyme(
        &self,
        nucleus: Phoneme,
        coda: Coda,
    ) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let last = syllables.last_mut().unwrap();
        *last = Syllable::new(last.onset(), nucleus, coda)?;
        Self::new(syllables)
    }

    pub fn replace_final_nucleus(
        &self,
        nucleus: Phoneme,
    ) -> anyhow::Result<Self> {
        let mut syllables = self.syllables().to_vec();
        let last = syllables.last_mut().unwrap();
        *last = Syllable::new(last.onset(), nucleus, last.coda())?;
        Self::new(syllables)
    }

    pub fn prepend(&self, mut syllable: Syllable) -> anyhow::Result<Self> {
        let mut syllables = self.syllables.to_vec();
        if let Some(first) = syllables.first_mut() {
            balance_cluster(&mut syllable.coda, &mut first.onset);
        }
        syllables.insert(0, syllable);
        Self::new(syllables)
    }

    pub fn append(&self, mut syllable: Syllable) -> anyhow::Result<Self> {
        let mut syllables = self.syllables.to_vec();
        if let Some(last) = syllables.last_mut() {
            balance_cluster(&mut last.coda, &mut syllable.onset);
        }
        syllables.push(syllable);
        Self::new(syllables)
    }

    fn same_audio(text: &str) -> Option<bool> {
        match text {
            "saŋ" | "dse" | "gas" | "kef" | "fwi" | "mací" | "reŋ" | "nyá"
            | "xim" | "xím" | "kas" | "fá" | "jí" | "ḱen" | "pa" | "eta"
            | "r" | "etif" | "etiŕ" | "wiya" | "wiyabi" => Some(true),
            "saysen" | "says" | "saysé" | "sayiẋ" | "dseńix" | "macis"
            | "éyxám" => Some(false),
            _ => None,
        }
    }

    pub fn audio_early(&self) -> Option<Audio> {
        let text = self.to_text();
        let suffix = match Self::same_audio(&text) {
            Some(true) => Some(""),
            Some(false) => Some("-early"),
            None => None,
        };
        if let Some(suffix) = suffix {
            let location = Location::internal(format!(
                "{}/audio/{}{}.ogg",
                StarLang.path(),
                text,
                suffix,
            ));
            Some(Audio(location))
        } else {
            None
        }
    }

    pub fn audio_late(&self) -> Option<Audio> {
        let text = self.to_text();
        let suffix = match Self::same_audio(&text) {
            Some(true) => Some(""),
            Some(false) => Some("-late"),
            None => None,
        };
        if let Some(suffix) = suffix {
            let location = Location::internal(format!(
                "{}/audio/{}{}.ogg",
                StarLang.path(),
                text,
                suffix,
            ));
            Some(Audio(location))
        } else {
            None
        }
    }

    fn find_nucleus(phonemes: &[Phoneme], initial: bool) -> Option<usize> {
        let mut pos = None;

        for (i, &phoneme) in phonemes.iter().enumerate() {
            if phoneme.classify() == PhonemeClass::Vowel {
                pos = Some(i);
                break;
            }
            if pos.is_some() {
                break;
            }
            if phoneme == Phoneme::R && (initial || i > 0) {
                pos = Some(i);
            }
        }

        pos
    }

    fn find_boundary(phonemes: &[Phoneme]) -> usize {
        phonemes
            .iter()
            .map(|ph| ph.classify())
            .enumerate()
            .max_by_key(|&(_, cls)| cls)
            .map_or(0, |(i, _)| i)
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_iter = self.phonemes();
        let mut other_iter = other.phonemes();

        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_ph), Some(other_ph)) => {
                    let ordering = self_ph.cmp(&other_ph);
                    if ordering != Ordering::Equal {
                        break ordering;
                    }
                },

                (Some(_), None) => break Ordering::Greater,
                (None, Some(_)) => break Ordering::Less,
                (None, None) => break Ordering::Equal,
            }
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for ch in self.phonemes() {
            write!(fmt, "{}", ch.to_text())?;
        }

        Ok(())
    }
}

impl Parse for Word {
    fn parse(phonemes: &[Phoneme]) -> anyhow::Result<Self> {
        let mut output = Vec::new();
        let mut slice = phonemes;
        let mut nucleus = Self::find_nucleus(slice, true)
            .ok_or_else(|| WordParseError { phonemes: slice.to_vec() })?;

        loop {
            match Self::find_nucleus(&slice[nucleus + 1 ..], false) {
                Some(relative_next) => {
                    let next = nucleus + 1 + relative_next;
                    let boundary =
                        nucleus + Self::find_boundary(&slice[nucleus .. next]);

                    let syllable = Syllable::parse(&slice[.. boundary])?;
                    output.push(syllable);

                    slice = &slice[boundary ..];
                    nucleus = next - boundary;
                },

                None => {
                    output.push(Syllable::parse(slice)?);
                    break;
                },
            }
        }

        Self::new(output)
    }
}

#[derive(Debug, Clone, Error)]
#[error("Parse error on word {phonemes:?}")]
pub struct WordParseError {
    pub phonemes: Vec<Phoneme>,
}

#[derive(Debug, Clone, Error)]
#[error("Invalid word made of syllables={syllables:?}")]
pub struct InvalidWord {
    pub syllables: Vec<Syllable>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Syllable {
    onset: Onset,
    nucleus: Phoneme,
    coda: Coda,
}

impl Syllable {
    pub fn new(
        onset: Onset,
        nucleus: Phoneme,
        coda: Coda,
    ) -> anyhow::Result<Self> {
        use Phoneme::*;
        use PhonemeClass::*;

        if nucleus.classify() != Vowel {
            if nucleus != R || onset.inner == Some(R) || coda.inner == Some(R) {
                Err(InvalidSyllable {
                    onset: onset.clone(),
                    nucleus,
                    coda: coda.clone(),
                })?;
            }
        }

        Ok(Self { onset, nucleus, coda })
    }

    pub fn onset(&self) -> Onset {
        self.onset
    }

    pub fn nucleus(&self) -> Phoneme {
        self.nucleus
    }

    pub fn coda(&self) -> Coda {
        self.coda
    }

    pub fn phonemes<'this>(
        &'this self,
    ) -> impl DoubleEndedIterator<Item = Phoneme> + 'this {
        self.onset
            .phonemes()
            .chain(iter::once(self.nucleus))
            .chain(self.coda.phonemes())
    }

    pub fn to_broad_ipa(&self) -> Cow<str> {
        let mut output = String::new();

        for phoneme in self.phonemes() {
            output.push_str(&phoneme.to_broad_ipa());
        }

        Cow::from(output)
    }

    fn find_nucleus(phonemes: &[Phoneme]) -> Option<usize> {
        let mut pos = None;

        for (i, &phoneme) in phonemes.iter().enumerate() {
            if phoneme.classify() == PhonemeClass::Vowel {
                pos = Some(i);
                break;
            }
            if phoneme == Phoneme::R {
                pos = Some(i);
            }
        }

        pos
    }
}

impl PartialOrd for Syllable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Syllable {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_iter = self.phonemes();
        let mut other_iter = other.phonemes();

        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_ph), Some(other_ph)) => {
                    let ordering = self_ph.cmp(&other_ph);
                    if ordering != Ordering::Equal {
                        break ordering;
                    }
                },

                (Some(_), None) => break Ordering::Greater,
                (None, Some(_)) => break Ordering::Less,
                (None, None) => break Ordering::Equal,
            }
        }
    }
}

impl Parse for Syllable {
    fn parse(phonemes: &[Phoneme]) -> anyhow::Result<Self> {
        let nucleus = Self::find_nucleus(phonemes).ok_or_else(|| {
            SyllableParseError { phonemes: phonemes.to_vec() }
        })?;

        let onset = Onset::parse(&phonemes[.. nucleus])?;
        let coda = Coda::parse(&phonemes[nucleus + 1 ..])?;

        Self::new(onset, phonemes[nucleus], coda)
    }
}

#[derive(Debug, Clone, Error)]
#[error("Parse error on syllable {phonemes:?}")]
pub struct SyllableParseError {
    pub phonemes: Vec<Phoneme>,
}

#[derive(Debug, Clone, Error)]
#[error(
    "Invalid onset made of onset={onset:?}, nucleus={nucleus:?}, coda={coda:?}"
)]
pub struct InvalidSyllable {
    pub onset: Onset,
    pub nucleus: Phoneme,
    pub coda: Coda,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Onset {
    outer: Option<Phoneme>,
    medial: Option<Phoneme>,
    inner: Option<Phoneme>,
}

impl Onset {
    pub fn new(
        outer: Option<Phoneme>,
        medial: Option<Phoneme>,
        inner: Option<Phoneme>,
    ) -> anyhow::Result<Self> {
        let outer_cls = outer.map(|ph| ph.classify());
        let medial_cls = medial.map(|ph| ph.classify());
        let inner_cls = inner.map(|ph| ph.classify());

        if !Self::valid_outer_medial(outer_cls, medial_cls)
            || !Self::valid_inner(inner_cls)
            || outer.is_some() && outer == medial
        {
            Err(InvalidOnset { outer, medial, inner })?
        }

        Ok(Self { outer, medial, inner })
    }

    pub fn replace_keep_plosive(
        self,
        outer_medial: Option<Phoneme>,
        inner: Option<Phoneme>,
    ) -> anyhow::Result<Self> {
        use PhonemeClass::*;

        let this = self.outer().map(Phoneme::classify);
        let param = outer_medial.map(Phoneme::classify);

        let (outer, medial) = match this {
            Some(Fricative) | None => match param {
                Some(Fricative) => (outer_medial, None),
                Some(_) => (None, outer_medial),
                None => (self.outer(), self.medial()),
            },
            _ => match param {
                Some(_) => (self.outer(), outer_medial),
                None => (self.outer(), self.medial()),
            },
        };

        Self::new(outer, medial, inner.or(self.inner()))
    }

    pub fn outer(&self) -> Option<Phoneme> {
        self.outer
    }

    pub fn medial(&self) -> Option<Phoneme> {
        self.medial
    }

    pub fn inner(&self) -> Option<Phoneme> {
        self.inner
    }

    pub fn valid_outer_medial(
        outer: Option<PhonemeClass>,
        medial: Option<PhonemeClass>,
    ) -> bool {
        use PhonemeClass::*;

        match outer {
            Some(Ejective) => matches!(
                medial,
                Some(Ejective) | Some(Fricative) | Some(Nasal) | None
            ),
            Some(Aspirated) => matches!(
                medial,
                Some(Aspirated) | Some(Fricative) | Some(Nasal) | None
            ),
            Some(Fricative) | None => matches!(medial, Some(Nasal) | None),
            _ => false,
        }
    }

    pub fn valid_inner(inner: Option<PhonemeClass>) -> bool {
        use PhonemeClass::*;
        matches!(inner, Some(Approximant) | None)
    }

    pub fn phonemes<'this>(
        &'this self,
    ) -> impl DoubleEndedIterator<Item = Phoneme> + 'this {
        iter::once(self.outer)
            .chain(iter::once(self.medial))
            .chain(iter::once(self.inner))
            .filter_map(|opt| opt)
    }
}

impl PartialOrd for Onset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Onset {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_iter = self.phonemes();
        let mut other_iter = other.phonemes();

        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_ph), Some(other_ph)) => {
                    let ordering = self_ph.cmp(&other_ph);
                    if ordering != Ordering::Equal {
                        break ordering;
                    }
                },

                (Some(_), None) => break Ordering::Greater,
                (None, Some(_)) => break Ordering::Less,
                (None, None) => break Ordering::Equal,
            }
        }
    }
}

impl Parse for Onset {
    fn parse(phonemes: &[Phoneme]) -> anyhow::Result<Self> {
        match phonemes {
            &[] => Self::new(None, None, None),
            &[first] => {
                let class = first.classify();
                if Self::valid_outer_medial(Some(class), None) {
                    Self::new(Some(first), None, None)
                } else if Self::valid_outer_medial(None, Some(class)) {
                    Self::new(None, Some(first), None)
                } else {
                    Self::new(None, None, Some(first))
                }
            },
            &[first, second] => {
                let first_cls = first.classify();
                let second_cls = first.classify();
                if Self::valid_outer_medial(Some(first_cls), Some(second_cls)) {
                    Self::new(Some(first), Some(second), None)
                } else if Self::valid_outer_medial(Some(first_cls), None) {
                    Self::new(Some(first), None, Some(second))
                } else {
                    Self::new(None, Some(first), Some(second))
                }
            },
            &[first, second, third] => {
                Self::new(Some(first), Some(second), Some(third))
            },
            _ => Err(OnsetParseError { phonemes: phonemes.to_vec() })?,
        }
    }
}

#[derive(Debug, Clone, Error)]
#[error(
    "Invalid onset made of outer={outer:?}, medial={medial:?}, inner={inner:?}"
)]
pub struct InvalidOnset {
    pub outer: Option<Phoneme>,
    pub medial: Option<Phoneme>,
    pub inner: Option<Phoneme>,
}

#[derive(Debug, Clone, Error)]
#[error("Parse error on onset {phonemes:?}")]
pub struct OnsetParseError {
    pub phonemes: Vec<Phoneme>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coda {
    inner: Option<Phoneme>,
    outer: Option<Phoneme>,
}

impl Coda {
    pub fn new(
        inner: Option<Phoneme>,
        outer: Option<Phoneme>,
    ) -> anyhow::Result<Self> {
        let inner_cls = inner.map(|ph| ph.classify());
        let outer_cls = outer.map(|ph| ph.classify());

        if !Self::valid_inner(inner_cls) || !Self::valid_outer(outer_cls) {
            Err(InvalidCoda { inner, outer })?
        }
        Ok(Self { inner, outer })
    }

    pub fn inner(&self) -> Option<Phoneme> {
        self.inner
    }

    pub fn outer(&self) -> Option<Phoneme> {
        self.outer
    }

    pub fn valid_outer(outer: Option<PhonemeClass>) -> bool {
        use PhonemeClass::*;
        matches!(outer, Some(Fricative) | Some(Nasal) | None)
    }

    pub fn valid_inner(inner: Option<PhonemeClass>) -> bool {
        use PhonemeClass::*;
        matches!(inner, Some(Approximant) | None)
    }

    pub fn phonemes<'this>(
        &'this self,
    ) -> impl DoubleEndedIterator<Item = Phoneme> + 'this {
        iter::once(self.inner)
            .chain(iter::once(self.outer))
            .filter_map(|opt| opt)
    }
}

impl PartialOrd for Coda {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coda {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_iter = self.phonemes();
        let mut other_iter = other.phonemes();

        loop {
            match (self_iter.next(), other_iter.next()) {
                (Some(self_ph), Some(other_ph)) => {
                    let ordering = self_ph.cmp(&other_ph);
                    if ordering != Ordering::Equal {
                        break ordering;
                    }
                },

                (Some(_), None) => break Ordering::Greater,
                (None, Some(_)) => break Ordering::Less,
                (None, None) => break Ordering::Equal,
            }
        }
    }
}

impl Parse for Coda {
    fn parse(phonemes: &[Phoneme]) -> anyhow::Result<Self> {
        match phonemes {
            &[] => Self::new(None, None),
            &[first] => {
                if Self::valid_inner(Some(first.classify())) {
                    Self::new(Some(first), None)
                } else {
                    Self::new(None, Some(first))
                }
            },
            &[first, second] => Self::new(Some(first), Some(second)),
            _ => Err(CodaParseError { phonemes: phonemes.to_vec() })?,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("Invalid coda made of inner={inner:?}, outer={outer:?}")]
pub struct InvalidCoda {
    pub inner: Option<Phoneme>,
    pub outer: Option<Phoneme>,
}

#[derive(Debug, Clone, Error)]
#[error("Parse error on coda {phonemes:?}")]
pub struct CodaParseError {
    pub phonemes: Vec<Phoneme>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phoneme {
    B,
    Gw,
    D,
    J,
    G,
    P,
    Kw,
    T,
    C,
    K,
    M,
    Mg,
    N,
    Nj,
    Ng,
    F,
    Xw,
    W,
    S,
    R,
    Y,
    Ii,
    X,
    I,
    Ee,
    H,
    E,
    Rr,
    A,
    Aa,
}

impl Phoneme {
    pub fn new(ch: char) -> anyhow::Result<Self> {
        use Phoneme::*;

        let phoneme = match ch.to_lowercase().next() {
            Some('b') => B,
            Some('ǵ') => Gw,
            Some('d') => D,
            Some('j') => J,
            Some('g') => G,
            Some('p') => P,
            Some('ḱ') => Kw,
            Some('t') => T,
            Some('c') => C,
            Some('k') => K,
            Some('m') => M,
            Some('ḿ') => Mg,
            Some('n') => N,
            Some('ń') => Nj,
            Some('ŋ') => Ng,
            Some('f') => F,
            Some('ẋ') => Xw,
            Some('w') => W,
            Some('s') => S,
            Some('r') => R,
            Some('y') => Y,
            Some('í') => Ii,
            Some('x') => X,
            Some('i') => I,
            Some('é') => Ee,
            Some('h') => H,
            Some('e') => E,
            Some('ŕ') => Rr,
            Some('a') => A,
            Some('á') => Aa,
            _ => Err(InvalidPhonemeChar { ch })?,
        };

        Ok(phoneme)
    }

    pub fn is_palatal(self) -> bool {
        use Phoneme::*;
        matches!(self, C | J | Nj | Y)
    }

    pub fn can_be_palatalized_regress(self) -> bool {
        use Phoneme::*;
        matches!(self, S | X | Xw)
    }

    pub fn can_be_palatalized_progress(self, prev: Self) -> bool {
        use Phoneme::*;
        match self {
            S | X | Xw => true,
            A | Aa | E | Ee | I | Ii => prev.classify() != PhonemeClass::Vowel,
            _ => false,
        }
    }

    pub fn triggers_retraction(self) -> bool {
        use Phoneme::*;
        matches!(self, A | Aa)
    }

    pub fn triggers_front(self) -> bool {
        use Phoneme::*;
        matches!(self, C | J | Nj | Y | Rr | H)
    }

    pub fn triggers_back(self) -> bool {
        use Phoneme::*;
        matches!(self, K | G | Ng | X)
    }

    pub fn triggers_back_rounded(self) -> bool {
        use Phoneme::*;
        matches!(self, Kw | Gw | Mg | Xw | W)
    }

    #[allow(dead_code)]
    pub fn can_be_nucleus(self) -> bool {
        use Phoneme::*;
        self.classify() == PhonemeClass::Vowel || self == R
    }

    pub fn classify(self) -> PhonemeClass {
        use Phoneme::*;
        match self {
            A | Aa | E | Ee | I | Ii => PhonemeClass::Vowel,
            R | Y | W | Rr => PhonemeClass::Approximant,
            M | N | Nj | Ng | Mg => PhonemeClass::Nasal,
            F | S | X | Xw | H => PhonemeClass::Fricative,
            B | D | J | G | Gw => PhonemeClass::Ejective,
            P | T | C | K | Kw => PhonemeClass::Aspirated,
        }
    }

    pub fn to_narrow_ipa(
        &self,
        prev: Option<Self>,
        next: Option<Self>,
        palatalized: bool,
    ) -> &'static str {
        use Phoneme::*;

        let triggers_front =
            prev.map_or(false, Phoneme::triggers_front) || palatalized;
        let triggers_back = prev.map_or(false, Phoneme::triggers_back);
        let triggers_back_rounded =
            prev.map_or(false, Phoneme::triggers_back_rounded);
        let triggers_retraction =
            next.map_or(false, Phoneme::triggers_retraction);

        match self {
            B => "pʼ",
            Gw if triggers_retraction => "qʷʼ",
            Gw => "kʷʼ",
            D => "tʼ",
            J => "cʼ",
            G if triggers_retraction => "qʼ",
            G => "kʼ",
            P => "pʰ",
            Kw if triggers_retraction => "qʷʰ",
            Kw => "kʷʰ",
            T => "tʰ",
            C => "cʰ",
            K if triggers_retraction => "qʰ",
            K => "kʰ",
            M => "m",
            Mg if triggers_retraction => "ɴ͡mʷ",
            Mg => "ŋ͡mʷ",
            N => "n",
            Nj => "ɲ",
            Ng if triggers_retraction => "ɴ",
            Ng => "ŋ",
            F => "ɸ",
            Xw if palatalized => "çʷ",
            Xw if triggers_retraction => "χʷ",
            Xw => "xʷ",
            W if triggers_retraction => "w̠",
            W => "w",
            S if palatalized => "ɕ",
            S => "s",
            R => "ɹ",
            Y => "j",
            Ii if triggers_front => "iː",
            Ii if triggers_back => "ɯə̯",
            Ii if triggers_back_rounded => "uː",
            Ii => "ɨː",
            X if palatalized => "ç",
            X if triggers_retraction => "χ",
            X => "x",
            I if triggers_front => "i",
            I if triggers_back => "ɯ",
            I if triggers_back_rounded => "u",
            I => "ɨ",
            Ee if triggers_front => "e̞ː",
            Ee if triggers_back => "ɤ̞ə̯",
            Ee if triggers_back_rounded => "o̞ː",
            Ee => "əː",
            H => "ħ",
            E if triggers_front => "e̞",
            E if triggers_back => "ɤ̞",
            E if triggers_back_rounded => "o̞",
            E => "ə",
            Rr => "ʕ",
            A if triggers_front => "æ",
            A if triggers_back => "ɑ",
            A if triggers_back_rounded => "ɒ",
            A => "ä",
            Aa if triggers_front => "æː",
            Aa if triggers_back => "ɑː",
            Aa if triggers_back_rounded => "ɒɔ̯",
            Aa => "äː",
        }
    }

    pub fn to_text(&self) -> &'static str {
        use Phoneme::*;

        match self {
            B => "b",
            Gw => "ǵ",
            D => "d",
            J => "j",
            G => "g",
            P => "p",
            Kw => "ḱ",
            T => "t",
            C => "c",
            K => "k",
            M => "m",
            Mg => "ḿ",
            N => "n",
            Nj => "ń",
            Ng => "ŋ",
            F => "f",
            Xw => "ẋ",
            W => "w",
            S => "s",
            R => "r",
            Y => "y",
            Ii => "í",
            X => "x",
            I => "i",
            Ee => "é",
            H => "h",
            E => "e",
            Rr => "ŕ",
            A => "a",
            Aa => "á",
        }
    }

    pub fn to_broad_ipa(&self) -> &'static str {
        use Phoneme::*;

        match self {
            B => "pʼ",
            Gw => "kʷʼ",
            D => "tʼ",
            J => "cʼ",
            G => "kʼ",
            P => "pʰ",
            Kw => "kʷʰ",
            T => "tʰ",
            C => "cʰ",
            K => "kʰ",
            M => "m",
            Mg => "ŋʷ",
            N => "n",
            Nj => "ɲ",
            Ng => "ŋ",
            F => "ɸ",
            Xw => "xʷ",
            W => "w",
            S => "s",
            R => "ɹ",
            Y => "j",
            Ii => "iː",
            X => "x",
            I => "i",
            Ee => "eː",
            H => "ħ",
            E => "e",
            Rr => "ʕ",
            A => "a",
            Aa => "aː",
        }
    }
}

impl Parse for Phoneme {
    fn parse(phonemes: &[Phoneme]) -> anyhow::Result<Self> {
        match phonemes {
            &[ph] => Ok(ph),
            _ => Err(PhonemeParseError { phonemes: phonemes.to_vec() })?,
        }
    }
}

impl fmt::Display for Phoneme {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.to_text())
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid phoneme orthography \"{ch}\"")]
pub struct InvalidPhonemeChar {
    pub ch: char,
}

#[derive(Debug, Clone, Error)]
#[error("Phoneme error on parse {phonemes:?}")]
pub struct PhonemeParseError {
    pub phonemes: Vec<Phoneme>,
}

#[derive(Debug, Clone, Error)]
#[error("forbidden consonant cluster {cluster:?}")]
pub struct ForbiddenCluster {
    pub cluster: Vec<Phoneme>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhonemeClass {
    Vowel,
    Approximant,
    Nasal,
    Fricative,
    Ejective,
    Aspirated,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn onsets() {
        use Phoneme::*;
        Onset::new(Some(D), Some(J), Some(R)).unwrap();
        Onset::new(Some(C), Some(P), Some(W)).unwrap();
        Onset::new(Some(K), Some(F), Some(Y)).unwrap();
        Onset::new(Some(G), Some(N), Some(R)).unwrap();
        Onset::new(Some(T), None, Some(W)).unwrap();
        Onset::new(Some(P), Some(S), None).unwrap();
        Onset::new(Some(B), Some(M), None).unwrap();
        Onset::new(Some(Xw), Some(Mg), None).unwrap();
        Onset::new(Some(H), None, None).unwrap();

        let onset = Onset::new(None, None, Some(W)).unwrap();
        assert_eq!(vec![W], onset.phonemes().collect::<Vec<_>>());

        let onset = Onset::new(Some(X), Some(Mg), Some(R)).unwrap();
        assert_eq!(vec![X, Mg, R], onset.phonemes().collect::<Vec<_>>());

        let onset = Onset::new(None, Some(Ng), Some(Y)).unwrap();
        assert_eq!(vec![Ng, Y], onset.phonemes().collect::<Vec<_>>());

        let onset = Onset::new(None, Some(Nj), None).unwrap();
        assert_eq!(vec![Nj], onset.phonemes().collect::<Vec<_>>());

        let onset = Onset::new(None, None, None).unwrap();
        assert_eq!(onset.phonemes().collect::<Vec<_>>().len(), 0);

        Onset::new(Some(D), Some(C), Some(R)).unwrap_err();
        Onset::new(Some(P), Some(C), Some(T)).unwrap_err();
        Onset::new(Some(P), Some(C), Some(F)).unwrap_err();
        Onset::new(None, None, Some(F)).unwrap_err();
        Onset::new(Some(N), None, Some(S)).unwrap_err();
        Onset::new(Some(X), None, Some(H)).unwrap_err();
        Onset::new(Some(X), Some(F), Some(H)).unwrap_err();
        Onset::new(Some(K), Some(K), None).unwrap_err();
    }

    #[test]
    fn codas() {
        use Phoneme::*;
        Coda::new(Some(Y), Some(F)).unwrap();
        Coda::new(None, Some(M)).unwrap();

        let coda = Coda::new(Some(R), Some(N)).unwrap();
        assert_eq!(vec![R, N], coda.phonemes().collect::<Vec<_>>());

        let coda = Coda::new(Some(W), None).unwrap();
        assert_eq!(vec![W], coda.phonemes().collect::<Vec<_>>());

        let coda = Coda::new(None, Some(S)).unwrap();
        assert_eq!(vec![S], coda.phonemes().collect::<Vec<_>>());

        let coda = Coda::new(None, None).unwrap();
        assert_eq!(coda.phonemes().collect::<Vec<_>>().len(), 0);

        Coda::new(Some(F), None).unwrap_err();
        Coda::new(Some(N), None).unwrap_err();
        Coda::new(Some(Xw), Some(H)).unwrap_err();
        Coda::new(Some(M), Some(S)).unwrap_err();
        Coda::new(Some(M), Some(Y)).unwrap_err();
    }

    #[test]
    fn syllables() {
        use Phoneme::*;

        let syl = Syllable::new(
            Onset::new(Some(K), Some(P), Some(Y)).unwrap(),
            Ee,
            Coda::new(Some(W), Some(N)).unwrap(),
        )
        .unwrap();
        assert_eq!(vec![K, P, Y, Ee, W, N], syl.phonemes().collect::<Vec<_>>());

        let syl = Syllable::new(
            Onset::new(Some(S), None, Some(W)).unwrap(),
            R,
            Coda::new(None, None).unwrap(),
        )
        .unwrap();
        assert_eq!(vec![S, W, R], syl.phonemes().collect::<Vec<_>>());

        let syl = Syllable::new(
            Onset::new(Some(F), None, None).unwrap(),
            I,
            Coda::new(None, Some(Ng)).unwrap(),
        )
        .unwrap();
        assert_eq!(vec![F, I, Ng], syl.phonemes().collect::<Vec<_>>());

        Syllable::new(
            Onset::new(Some(X), None, Some(R)).unwrap(),
            R,
            Coda::new(None, None).unwrap(),
        )
        .unwrap_err();
    }

    #[test]
    fn words() {
        use Phoneme::*;

        let word = Word::new(vec![
            Syllable::new(
                Onset::new(Some(F), None, None).unwrap(),
                I,
                Coda::new(None, Some(Ng)).unwrap(),
            )
            .unwrap(),
            Syllable::new(
                Onset::new(Some(S), None, Some(W)).unwrap(),
                R,
                Coda::new(None, None).unwrap(),
            )
            .unwrap(),
            Syllable::new(
                Onset::new(Some(K), Some(P), None).unwrap(),
                Ee,
                Coda::new(Some(Y), None).unwrap(),
            )
            .unwrap(),
        ])
        .unwrap();
        assert_eq!(
            vec![F, I, Ng, S, W, R, K, P, Ee, Y],
            word.phonemes().collect::<Vec<_>>()
        );

        Word::new(vec![
            Syllable::new(
                Onset::new(Some(F), None, None).unwrap(),
                I,
                Coda::new(None, Some(Ng)).unwrap(),
            )
            .unwrap(),
            Syllable::new(
                Onset::new(None, Some(Ng), None).unwrap(),
                Ee,
                Coda::new(Some(Y), None).unwrap(),
            )
            .unwrap(),
        ])
        .unwrap_err();

        Word::new(vec![
            Syllable::new(
                Onset::new(Some(F), None, None).unwrap(),
                I,
                Coda::new(Some(Y), None).unwrap(),
            )
            .unwrap(),
            Syllable::new(
                Onset::new(Some(S), Some(N), Some(W)).unwrap(),
                Ee,
                Coda::new(Some(Y), None).unwrap(),
            )
            .unwrap(),
        ])
        .unwrap_err();
    }
}
