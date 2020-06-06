use super::StarLang;
use std::{borrow::Cow, iter};
use thiserror::Error;
use wfts_lang::{
    Character as CharTrait,
    Lang,
    Phoneme as PhonemeTrait,
    Syllable as SylTrait,
    Word as WordTrait,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            let mut iter = syllable.iter();
            let first = iter.next().unwrap();
            let last = iter.last();
            let onset_len = syllable.onset().iter().count();
            if let Some(prev) = prev {
                let bypass_dist =
                    matches!(first.classify(), Aspirated | Ejective);

                let wrong_dist = onset_len
                    .checked_sub(prev_coda_len)
                    .map_or(false, |diff| diff > 1);

                if prev == first || !bypass_dist && wrong_dist {
                    Err(InvalidWord { syllables: syllables.clone() })?;
                }
            }
            prev = Some(last.unwrap_or(first));
            prev_coda_len = syllable.coda().iter().count();
        }

        Ok(Self { syllables })
    }

    pub fn iter<'this>(&'this self) -> impl Iterator<Item = Phoneme> + 'this {
        self.syllables.iter().flat_map(Syllable::iter)
    }
}

impl WordTrait for Word {
    type Lang = StarLang;

    fn chars(&self) -> Cow<[<Self::Lang as Lang>::Character]> {
        Cow::from(self.iter().collect::<Vec<_>>())
    }

    fn syllables(&self) -> Cow<[<Self::Lang as Lang>::Syllable]> {
        Cow::from(&self.syllables)
    }

    fn to_broad_ipa(&self) -> Cow<str> {
        let mut output = String::from("ˈ");
        let mut first = false;

        for syllable in &self.syllables {
            if first {
                first = false;
            } else {
                output.push('.');
            }

            for phoneme in syllable.iter() {
                output.push_str(&phoneme.to_broad_ipa())
            }
        }

        Cow::from(output)
    }

    fn to_narrow_ipa(&self) -> Cow<str> {
        let mut output = String::from("ˈ");
        let mut prev = None;

        for syllable in &self.syllables {
            if prev.is_some() {
                output.push('.');
            }

            for phoneme in syllable.iter() {
                output.push_str(phoneme.to_narrow_ipa(prev));
                prev = Some(phoneme);
            }
        }

        Cow::from(output)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("Invalid word made of syllables={syllables:?}")]
pub struct InvalidWord {
    pub syllables: Vec<Syllable>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn onset(&self) -> &Onset {
        &self.onset
    }

    pub fn nucleus(&self) -> Phoneme {
        self.nucleus
    }

    pub fn coda(&self) -> &Coda {
        &self.coda
    }

    pub fn iter<'this>(&'this self) -> impl Iterator<Item = Phoneme> + 'this {
        self.onset
            .iter()
            .chain(iter::once(self.nucleus))
            .chain(self.coda.iter())
    }
}

impl SylTrait for Syllable {
    type Lang = StarLang;

    fn phonemes(&self) -> Cow<[<Self::Lang as Lang>::Phoneme]> {
        Cow::from(self.iter().collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error(
    "Invalid onset made of onset={onset:?}, nucleus={nucleus:?}, coda={coda:?}"
)]
pub struct InvalidSyllable {
    pub onset: Onset,
    pub nucleus: Phoneme,
    pub coda: Coda,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub fn outer(&self) -> Option<Phoneme> {
        self.outer
    }

    pub fn medial(&self) -> Option<Phoneme> {
        self.medial
    }

    pub fn inner(&self) -> Option<Phoneme> {
        self.inner
    }

    fn valid_outer_medial(
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

    fn valid_inner(inner: Option<PhonemeClass>) -> bool {
        use PhonemeClass::*;
        matches!(inner, Some(Approximant) | None)
    }

    pub fn iter<'this>(&'this self) -> impl Iterator<Item = Phoneme> + 'this {
        iter::once(self.outer)
            .chain(iter::once(self.medial))
            .chain(iter::once(self.inner))
            .filter_map(|opt| opt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error(
    "Invalid onset made of outer={outer:?}, medial={medial:?}, inner={inner:?}"
)]
pub struct InvalidOnset {
    pub outer: Option<Phoneme>,
    pub medial: Option<Phoneme>,
    pub inner: Option<Phoneme>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    fn valid_outer(outer: Option<PhonemeClass>) -> bool {
        use PhonemeClass::*;
        matches!(outer, Some(Fricative) | Some(Nasal) | None)
    }

    fn valid_inner(inner: Option<PhonemeClass>) -> bool {
        use PhonemeClass::*;
        matches!(inner, Some(Approximant) | None)
    }

    pub fn iter<'this>(&'this self) -> impl Iterator<Item = Phoneme> + 'this {
        iter::once(self.inner)
            .chain(iter::once(self.outer))
            .filter_map(|opt| opt)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("Invalid coda made of inner={inner:?}, outer={outer:?}")]
pub struct InvalidCoda {
    pub inner: Option<Phoneme>,
    pub outer: Option<Phoneme>,
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
    fn triggers_retraction(self) -> bool {
        use Phoneme::*;
        matches!(self, A | Aa)
    }

    fn triggers_front(self) -> bool {
        use Phoneme::*;
        matches!(self, C | J | Nj | Y | Rr | H)
    }

    fn triggers_back(self) -> bool {
        use Phoneme::*;
        matches!(self, K | G | Ng | X)
    }

    fn triggers_back_rounded(self) -> bool {
        use Phoneme::*;
        matches!(self, Kw | Gw | Mg | Xw | W)
    }

    fn can_be_nucleus(self) -> bool {
        use Phoneme::*;
        self.classify() == PhonemeClass::Vowel || self == R
    }

    fn classify(self) -> PhonemeClass {
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

    fn to_narrow_ipa(&self, prev: Option<Self>) -> &'static str {
        use Phoneme::*;

        let triggers_front = prev.map_or(false, Phoneme::triggers_front);
        let triggers_back = prev.map_or(false, Phoneme::triggers_back);
        let triggers_back_rounded =
            prev.map_or(false, Phoneme::triggers_back_rounded);
        let triggers_retraction =
            prev.map_or(false, Phoneme::triggers_retraction);

        match self {
            B => "pʼ",
            Gw if triggers_retraction => "kʷʼ",
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
            K => "kʰ",
            M => "m",
            Mg if triggers_retraction => "ɴ͡mʷ",
            Mg => "ŋ͡mʷ",
            N => "n",
            Nj => "ɲ",
            Ng if triggers_retraction => "ɴ",
            Ng => "ŋ",
            F => "f",
            Xw if triggers_retraction => "χʷ",
            Xw => "xʷ",
            W if triggers_retraction => "w̠",
            W => "w",
            S => "s",
            R => "ɹ",
            Y => "j",
            Ii if triggers_front => "iː",
            Ii if triggers_back => "ɯə̯",
            Ii if triggers_back_rounded => "uː",
            Ii => "ɨː",
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
}

impl CharTrait for Phoneme {
    type Lang = StarLang;

    fn to_text(&self) -> Cow<str> {
        use Phoneme::*;

        Cow::from(match self {
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
        })
    }
}

impl PhonemeTrait for Phoneme {
    type Lang = StarLang;

    fn to_broad_ipa(&self) -> Cow<str> {
        use Phoneme::*;

        Cow::from(match self {
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
            F => "f",
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
        })
    }
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
        assert_eq!(vec![W], onset.iter().collect::<Vec<_>>());

        let onset = Onset::new(Some(X), Some(Mg), Some(R)).unwrap();
        assert_eq!(vec![X, Mg, R], onset.iter().collect::<Vec<_>>());

        let onset = Onset::new(None, Some(Ng), Some(Y)).unwrap();
        assert_eq!(vec![Ng, Y], onset.iter().collect::<Vec<_>>());

        let onset = Onset::new(None, Some(Nj), None).unwrap();
        assert_eq!(vec![Nj], onset.iter().collect::<Vec<_>>());

        let onset = Onset::new(None, None, None).unwrap();
        assert_eq!(onset.iter().collect::<Vec<_>>().len(), 0);

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
        assert_eq!(vec![R, N], coda.iter().collect::<Vec<_>>());

        let coda = Coda::new(Some(W), None).unwrap();
        assert_eq!(vec![W], coda.iter().collect::<Vec<_>>());

        let coda = Coda::new(None, Some(S)).unwrap();
        assert_eq!(vec![S], coda.iter().collect::<Vec<_>>());

        let coda = Coda::new(None, None).unwrap();
        assert_eq!(coda.iter().collect::<Vec<_>>().len(), 0);

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
        assert_eq!(vec![K, P, Y, Ee, W, N], syl.iter().collect::<Vec<_>>());

        let syl = Syllable::new(
            Onset::new(Some(S), None, Some(W)).unwrap(),
            R,
            Coda::new(None, None).unwrap(),
        )
        .unwrap();
        assert_eq!(vec![S, W, R], syl.iter().collect::<Vec<_>>());

        let syl = Syllable::new(
            Onset::new(Some(F), None, None).unwrap(),
            I,
            Coda::new(None, Some(Ng)).unwrap(),
        )
        .unwrap();
        assert_eq!(vec![F, I, Ng], syl.iter().collect::<Vec<_>>());

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
            word.iter().collect::<Vec<_>>()
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
