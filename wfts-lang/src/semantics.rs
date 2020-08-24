#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SynLevel {
    L1,
    L2,
    L3,
    L4,
    L5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Relation {
    Unrelated,
    Synonym(SynLevel),
    Antonym,
    Hiponym,
    Hyperonym,
    Coordinate,
}

impl Relation {
    pub fn flip(self) -> Self {
        match self {
            Relation::Hiponym => Relation::Hyperonym,
            Relation::Hyperonym => Relation::Hiponym,
            rel => rel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Meaning {
    Star,
    NightStar,
    Sun,
    Eye,
    Wind,
    Fire,
    Tree,
    Big,
}

impl Meaning {
    pub fn relation(self, other: Self) -> Relation {
        use Relation::*;
        use SynLevel::*;

        if self == other {
            Synonym(L5)
        } else {
            self.relation_raw(other)
                .or_else(|| other.relation_raw(self).map(Relation::flip))
                .unwrap_or(Unrelated)
        }
    }

    fn relation_raw(self, other: Self) -> Option<Relation> {
        use Meaning::*;
        use Relation::*;

        match (self, other) {
            (Star, NightStar) => Some(Hyperonym),
            (Star, Sun) => Some(Hyperonym),
            (NightStar, Sun) => Some(Coordinate),
            _ => None,
        }
    }

    pub fn identifier(self) -> &'static str {
        use Meaning::*;

        match self {
            Star => "star",
            Sun => "sun",
            NightStar => "night star",
            Eye => "eye",
            Wind => "wind",
            Fire => "fire",
            Tree => "tree",
            Big => "big",
        }
    }

    pub fn description(self) -> &'static str {
        use Meaning::*;

        match self {
            Star => "A star.",
            Sun => "One of the suns; a star seen during the day.",
            NightStar => "A star seen at night.",
            Eye => "An eye.",
            Wind => "The wind; the air that blows naturally.",
            Fire => "Fire; the light and heat emitted by burning something.",
            Tree => "A tree.",
            Big => "Big; the quality of having a big size, but not huge.",
        }
    }
}
