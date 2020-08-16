use std::fmt;

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

impl BasicCase {
    pub const ALL: &'static [Self] = &[
        BasicCase::Nominative,
        BasicCase::Accusative,
        BasicCase::Topical,
        BasicCase::Postpositional,
    ];
}

impl fmt::Display for BasicCase {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            BasicCase::Nominative => "nominative",
            BasicCase::Accusative => "accusative",
            BasicCase::Topical => "topical",
            BasicCase::Postpositional => "postpositional",
        })
    }
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

impl Case {
    pub const ALL: &'static [Self] = &[
        Case::Basic(BasicCase::Nominative),
        Case::Basic(BasicCase::Accusative),
        Case::Basic(BasicCase::Topical),
        Case::Basic(BasicCase::Postpositional),
        Case::Passive,
    ];
}

impl fmt::Display for Case {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Case::Basic(basic) => fmt::Display::fmt(basic, fmt),
            Case::Passive => fmt.pad("passive"),
        }
    }
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
    Coordinative,
}

impl ClauseCase {
    pub const ALL: &'static [Self] = &[
        ClauseCase::Subordinative(Case::Basic(BasicCase::Nominative)),
        ClauseCase::Subordinative(Case::Basic(BasicCase::Accusative)),
        ClauseCase::Subordinative(Case::Basic(BasicCase::Topical)),
        ClauseCase::Subordinative(Case::Basic(BasicCase::Postpositional)),
        ClauseCase::Subordinative(Case::Passive),
        ClauseCase::Coordinative,
    ];
}

impl fmt::Display for ClauseCase {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClauseCase::Subordinative(sub) => fmt::Display::fmt(sub, fmt),
            ClauseCase::Coordinative => fmt.pad("coordinative"),
        }
    }
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
            ClauseCase::Coordinative => false,
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

impl Gender {
    pub const ALL: &'static [Self] =
        &[Gender::Divine, Gender::Animate, Gender::Inanimate];
}

impl fmt::Display for Gender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            Gender::Divine => "divine",
            Gender::Animate => "animate",
            Gender::Inanimate => "inanimate",
        })
    }
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

impl Number {
    pub const ALL: &'static [Self] =
        &[Number::Singular, Number::Plural, Number::Nullar, Number::Collective];
}

impl Agreement for Number {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            Number::Singular => "singular",
            Number::Plural => "plural",
            Number::Nullar => "nullar",
            Number::Collective => "collective",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Person {
    First,
    Second,
    Third,
}

impl Person {
    pub const ALL: &'static [Self] =
        &[Person::First, Person::Second, Person::Third];
}

impl fmt::Display for Person {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            Person::First => "1st-person",
            Person::Second => "2nd-person",
            Person::Third => "3rd-person",
        })
    }
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

impl BasicMood {
    pub const ALL: &'static [Self] =
        &[BasicMood::Indicative, BasicMood::Imperative];
}

impl fmt::Display for BasicMood {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            BasicMood::Indicative => "indicative",
            BasicMood::Imperative => "imperative",
        })
    }
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

impl Mood {
    pub const ALL: &'static [Self] = &[
        Mood::Basic(BasicMood::Indicative),
        Mood::Basic(BasicMood::Imperative),
        Mood::Subjunctive,
        Mood::Interrogative,
        Mood::Optative,
    ];
}

impl fmt::Display for Mood {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mood::Basic(basic) => fmt::Display::fmt(basic, fmt),
            Mood::Subjunctive => fmt.pad("subjunctive"),
            Mood::Interrogative => fmt.pad("interrogative"),
            Mood::Optative => fmt.pad("optative"),
        }
    }
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

impl IndicativeTense {
    pub const ALL: &'static [Self] = &[
        IndicativeTense::Present,
        IndicativeTense::Past,
        IndicativeTense::NearFuture,
        IndicativeTense::FarFuture,
    ];
}

impl fmt::Display for IndicativeTense {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            IndicativeTense::Present => "present",
            IndicativeTense::Past => "past",
            IndicativeTense::NearFuture => "near-future",
            IndicativeTense::FarFuture => "far-future",
        })
    }
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

impl ImperativeTense {
    pub const ALL: &'static [Self] =
        &[ImperativeTense::Present, ImperativeTense::Future];
}

impl fmt::Display for ImperativeTense {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(match self {
            ImperativeTense::Present => "present",
            ImperativeTense::Future => "future",
        })
    }
}

impl Agreement for ImperativeTense {
    fn agrees(&self, other: &Self) -> bool {
        self == other
    }
}
