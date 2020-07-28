pub trait Agreement<T = Self> {
    fn agrees(&self, other: &T) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BasicCase {
    Nominative,
    Accusative,
    Topical,
    Postpositional,
}

impl Agreement for BasicCase {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

impl Agreement<Case> for BasicCase {
    fn agrees(&self, other: &Case) -> bool {
        other.agrees(self)
    }
}

impl Agreement<ClauseCase> for BasicCase {
    fn agrees(&self, other: &ClauseCase) -> bool {
        other.agrees(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Case {
    Basic(BasicCase),
    Passive,
}

impl Agreement for Case {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

impl Agreement<BasicCase> for Case {
    fn agrees(&self, other: &BasicCase) -> bool {
        match self {
            Case::Basic(basic) => basic == other,
            Case::Passive => *other == BasicCase::Nominative,
        }
    }
}

impl Agreement<ClauseCase> for Case {
    fn agrees(&self, other: &ClauseCase) -> bool {
        other.agrees(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClauseCase {
    Subordinative(Case),
    Coordenative,
}

impl Agreement for ClauseCase {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

impl Agreement<Case> for ClauseCase {
    fn agrees(&self, other: &Case) -> bool {
        match self {
            ClauseCase::Subordinative(case) => case == other,
            ClauseCase::Coordenative => false,
        }
    }
}

impl Agreement<BasicCase> for ClauseCase {
    fn agrees(&self, other: &BasicCase) -> bool {
        self.agrees(&Case::Basic(*other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gender {
    Divine,
    Animate,
    Inanimate,
}

impl Agreement for Gender {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    Singular,
    Plural,
    Nullar,
    Collective,
}

impl Agreement for Number {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Person {
    First,
    Second,
    Third,
}

impl Agreement for Person {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BasicMood {
    Indicative,
    Imperative,
}

impl Agreement for BasicMood {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

impl Agreement<Mood> for BasicMood {
    fn agrees(&self, other: &Mood) -> bool {
        other.agrees(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mood {
    Basic(BasicMood),
    Subjunctive,
    Interrogative,
    Optative,
}

impl Agreement for Mood {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

impl Agreement<BasicMood> for Mood {
    fn agrees(&self, other: &BasicMood) -> bool {
        match self {
            Mood::Basic(basic) => basic == other,
            Mood::Subjunctive | Mood::Interrogative => {
                *other == BasicMood::Indicative
            },
            Mood::Optative => *other == BasicMood::Imperative,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IndicativeTense {
    Present,
    Past,
    NearFuture,
    FarFuture,
}

impl Agreement for IndicativeTense {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImperativeTense {
    Present,
    Future,
}

impl Agreement for ImperativeTense {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}
