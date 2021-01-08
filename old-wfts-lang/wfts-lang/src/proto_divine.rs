/*
use std::{collections::BTreeSet, slice};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NarrowProps {
    pub phoneme: Phoneme,
    pub palatalizes: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NarrowLimits {
    pub first: NarrowProps,
    pub last: NarrowProps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NarrowAdjacency {
    pub prev: Option<NarrowProps>,
    pub next: Option<NarrowProps>,
}

impl NarrowAdjacency {
    pub fn isolated() -> Self {
        Self { prev: None, next: None }
    }
}

pub trait NarrowPronunc {
    fn narrow_limits(&self, adjacency: NarrowAdjacency)
        -> Option<NarrowLimits>;

    fn narrow_pronunc(
        &self,
        buf: &mut String,
        alts: &mut BTreeSet<String>,
        adjacency: NarrowAdjacency,
    );
}

impl<'this, T> NarrowPronunc for &'this T
where
    T: NarrowPronunc,
{
    fn narrow_limits(
        &self,
        adjacency: NarrowAdjacency,
    ) -> Option<NarrowLimits> {
        (**self).narrow_limits(adjacency)
    }

    fn narrow_pronunc(
        &self,
        buf: &mut String,
        alts: BTreeSet<String>,
        adjacency: NarrowAdjacency,
    ) {
        (**self).narrow_pronunc(buf, alts, adjacency)
    }
}

pub fn narrow_boundary<T, U>(
    adjacency: NarrowAdjacency,
    prev: T,
    next: U,
) -> (Option<NarrowProps>, Option<NarrowProps>)
where
    T: NarrowPronunc,
    U: NarrowPronunc,
{
    let (prev_lim, next_lim) = narrow_seq_pair_lims(adjacency, prev, next);

    (prev_lim.map(|lim| lim.last), next_lim.map(|lim| lim.first))
}

pub fn narrow_seq_lims<T, U>(
    adjacency: NarrowAdjacency,
    prev: T,
    next: U,
) -> Option<NarrowLimits>
where
    T: NarrowPronunc,
    U: NarrowPronunc,
{
    let (prev_lim, next_lim) = narrow_seq_pair_lims(adjacency, prev, next);

    let prev = prev_lim.map(|lim| lim.first);
    let next = next_lim.map(|lim| lim.last);

    if let (Some(first), Some(last)) = (prev, next) {
        Some(NarrowLimits { first, last })
    } else {
        None
    }
}

pub fn narrow_seq_pair_lims<T, U>(
    adjacency: NarrowAdjacency,
    prev: T,
    next: U,
) -> (Option<NarrowLimits>, Option<NarrowLimits>)
where
    T: NarrowPronunc,
    U: NarrowPronunc,
{
    let prev_isolated = prev.narrow_limits(NarrowAdjacency::isolated());
    let next_adj = NarrowAdjacency {
        prev: prev_isolated.map(|lim| lim.last).or(adjacency.prev),
        next: adjacency.next,
    };
    let next_lim = next.narrow_limits(next_adj);
    let prev_adj = NarrowAdjacency {
        prev: adjacency.prev,
        next: next_lim.map(|lim| lim.first).or(adjacency.prev),
    };
    let prev_lim = prev.narrow_limits(prev_adj);

    (prev_lim, next_lim)
}

impl NarrowAdjacency {
    pub fn test_prev<F>(self, test: F) -> bool
    where
        F: FnOnce(NarrowProps) -> bool,
    {
        self.prev.map_or(false, test)
    }

    pub fn test_next<F>(self, test: F) -> bool
    where
        F: FnOnce(NarrowProps) -> bool,
    {
        self.next.map_or(false, test)
    }

    pub fn either<F>(self, mut test: F) -> bool
    where
        F: FnMut(NarrowProps) -> bool,
    {
        self.test_prev(&mut test) || self.test_next(test)
    }

    pub fn both<F>(self, mut test: F) -> bool
    where
        F: FnMut(NarrowProps) -> bool,
    {
        self.test_prev(&mut test) && self.test_next(test)
    }
}

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

    /*
    pub fn narrow_pronunc(
        self,
        adjacency: NarrowNeighbours,
    ) -> (&'static str, &'static [&'static str]) {
        use Obstruent::*;

        let voiced = adjacency.both(|props| props.phoneme.voices());
        let palatalized = adjacency.both(|props| props.palatalizes);

        match self {
            P if voiced => ("b", &[]),
            P => ("p", &[]),
            T if voiced => ("d", &[]),
            T => ("t", &[]),
            K if voiced && palatalized => ("ɟ", &[]),
            K if palatalized => ("c", &[]),
            K if voiced => ("g", &[]),
            K => ("k", &[]),
            F if voiced => ("v", &["β"]),
            F => ("f", &["ɸ"]),
            S if voiced => ("z", &[]),
            S => ("s", &[]),
            H if voiced && palatalized => ("ʝ", &[]),
            H if palatalized => ("ç", &[]),
            H if voiced => ("ɣ", &[]),
            H => ("x", &["h"]),
        }
    }
    */
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

    pub fn narrow_pronunc(
        &self,
        palatalizes: bool,
    ) -> (&'static str, &'static [&'static str]) {
        use Sonorant::*;

        match self {
            M => ("m", &[]),
            N => ("n", &[]),
            Ng if palatalizes => ("ɲ", &[]),
            Ng => ("ŋ", &[]),
            W => ("w", &["β̞", "ʋ"]),
            L => ("l", &["ɹ", "ɾ"]),
            J => ("j", &["i̯"]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Consonant {
    Son(Sonorant),
    Obs(Obstruent),
}

impl Consonant {
    pub fn orthography(&self) -> &'static str {
        use Consonant::*;

        match self {
            Son(cons) => cons.orthography(),
            Obs(cons) => cons.orthography(),
        }
    }

    pub fn broad_pronunc(&self) -> &'static str {
        use Consonant::*;

        match self {
            Son(cons) => cons.broad_pronunc(),
            Obs(cons) => cons.broad_pronunc(),
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

    pub fn narrow_pronunc(
        &self,
        palatalizes: bool,
        labializes: bool,
    ) -> (&'static str, &'static [&'static str]) {
        use self::Vowel::*;

        match self {
            Ae if palatalizes => ("a", &[]),
            Ae => ("a", &["æ"]),
            E if palatalizes => ("e̞", &[]),
            E => ("e", &["e̞"]),
            I if palatalizes => ("ɪ", &[]),
            I => ("i", &[]),
            Ao => ("ɒ", &["ɒ̜"]),
            O if labializes => ("o̞", &[]),
            O => ("o", &["o̞"]),
            U if labializes => ("ʊ", &[]),
            U => ("u", &[]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Phoneme {
    Vowel(Vowel),
    Conson(Consonant),
}

impl Phoneme {
    pub fn orthography(&self) -> &'static str {
        use Phoneme::*;

        match self {
            Conson(phoneme) => phoneme.orthography(),
            Vowel(phoneme) => phoneme.orthography(),
        }
    }

    pub fn broad_pronunc(&self) -> &'static str {
        use Phoneme::*;

        match self {
            Conson(phoneme) => phoneme.broad_pronunc(),
            Vowel(phoneme) => phoneme.broad_pronunc(),
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
        self.outer.map(Consonant::from).or(self.inner.map(Consonant::from))
    }

    pub fn last(self) -> Option<Consonant> {
        self.inner.map(Consonant::from).or(self.outer.map(Consonant::from))
    }
}

impl NarrowPronunc for Onset {
    fn narrow_limits(
        &self,
        adjacency: NarrowAdjacency,
    ) -> Option<NarrowLimits> {
        narrow_seq_lims(self.outer, self.inner, adjacency)
    }

    fn narrow_pronunc(&self, buf: &mut String, alts: &mut BTreeSet<String>, adjacency: NarrowAdjacency) {
        let (outer_lim, inner_lim) = narrow_boundary(self.outer, self.inner, adjacency);
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
    Inner,
    Outer,
    Done,
}

impl Iterator for OnsetIter {
    type Item = Consonant;

    fn next(&mut self) -> Option<Self::Item> {
        use Consonant::*;

        loop {
            match self.state {
                OnsetIterState::Outer => {
                    self.state = OnsetIterState::Outer;
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
        self.onset.first().map(Phoneme::from).unwrap_or(self.nucleus.into())
    }

    pub fn last(self) -> Phoneme {
        self.coda.map(Phoneme::from).unwrap_or(self.nucleus.into())
    }
}

impl NarrowPronunc for Root {
    fn narrow_limits(
        &self,
        adjacency: NarrowAdjacency,
    ) -> Option<NarrowLimits> {
        let vowel_lims =
            self.nucleus.narrow_limits(NarrowAdjacency::isolated());
        let vowel_first = vowel_lims.map(|lims| lims.first);
        let vowel_last = vowel_lims.map(|lims| lims.last);
        let onset_adj =
            NarrowAdjacency { prev: adjacency.prev, next: vowel_first };
        let onset_lims = self.onset.narrow_limits(onset_adj);
        let coda_adj =
            NarrowAdjacency { prev: vowel_last, next: adjacency.next };
        let coda_lims = self.coda.narrow_limits(coda_adj);

        Some(NarrowLimits {
            first: onset_adj.map(|lims| lims.first).or(vowel_last),
            last: coda_adj.map(|lims| lims.last).or(vowel_first),
        })
    }

    fn narrow_pronunc(
        &self,
        buf: &mut String,
        alts: &mut BTreeSet<String>,
        adjacency: NarrowAdjacency,
    ) {
        let mut onset_alts = BTreeSet::new();
        let mut vowel_alts = BTreeSet::new();
        let mut coda_alts = BTreeSet::new();

        let vowel_lims =
            self.nucleus.narrow_limits(NarrowAdjacency::isolated());
        let vowel_first = vowel_lims.map(|lims| lims.first);
        let vowel_last = vowel_lims.map(|lims| lims.last);
        let onset_adj =
            NarrowAdjacency { prev: adjacency.prev, next: vowel_first };
        let onset_lims = self.onset.narrow_limits(onset_adj);
        let coda_adj =
            NarrowAdjacency { prev: vowel_last, next: adjacency.next };
        let coda_lims = self.coda.narrow_limits(coda_adj);
        let vowel_adj = NarrowAdjacency {
            prev: onset_lims.map(|lims| lims.last).or(adjacency.prev),
            next: coda_lims.map(|lims| lims.first).or(adjacency.next),
        };

        self.onset.narrow_pronunc(buf, &mut onset_alts, onset_adj);
        self.vowel.narrow_pronunc(buf, &mut vowel_alts, vowel_adj);
        self.coda.narrow_pronunc(buf, &mut coda_alts, coda_adj);
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

impl NarrowPronunc for Composite {
    fn narrow_limits(
        &self,
        adjacency: NarrowAdjacency,
    ) -> Option<NarrowLimits> {
        let limits = |root: Root| root.narrow_limits(adjacency);
        let front = self.roots().find_map(limits);
        let back = self.roots().rev().find_map(limits);

        if let (Some(front), Some(back)) = (front, back) {
            Some(NarrowLimits { first: front.first, last: back.last })
        } else {
            front.or(back)
        }
    }

    fn narrow_pronunc(
        &self,
        buf: &mut String,
        alts: &mut BTreeSet<String>,
        adjacency: NarrowAdjacency,
    ) {
        let mut curr = self.head;
        for &next in &self.tail {
            let (curr_lims, next_lims) = narrow_boundary(adjacency, curr, next);
            let curr_adjacency = NarrowAdjacency {
                prev: adjacency.prev,
                next: next_lims.or(adjacency.next),
            };
            curr.narrow_pronunc(buf, alts, adjacency);
            adjacency.prev = curr_lims.or(adjacency.prev);
            curr = next;
        }

        curr.narrow_pronunc(buf, alts, adjacency);
    }
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
        let others = self.tail.iter();
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
                None => break self.back?.next(),
            }
        }
    }
}

impl<'comp> DoubleEndedIterator for CompositeIter<'comp> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(phoneme) = self.back?.next_back() {
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
*/
