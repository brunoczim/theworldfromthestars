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
    ToDivSuffer,
    ToMortSuffer,
    To,
    ToBelow,
    From,
    Of,
    With,
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
        use SynLevel::*;

        match (self, other) {
            (ToDivSuffer, ToMortSuffer) => Some(Coordinate),
            (To, ToBelow) => Some(Hyperonym),
            (From, Of) => Some(Synonym(L3)),
            _ => None,
        }
    }

    pub fn identifier(self) -> &'static str {
        use Meaning::*;

        match self {
            ToDivSuffer => "to-div-suffer",
            ToMortSuffer => "to-mortal-suffer",
            To => "to",
            ToBelow => "to-below",
            From => "from",
            Of => "of",
            With => "with",
        }
    }

    pub fn description(self) -> &'static str {
        use Meaning::*;

        match self {
            ToDivSuffer => {
                "Shows that something suffers from an action with divine \
                 consequences."
            },
            ToMortSuffer => {
                "Shows that something suffers from an action with mortal \
                 consequences."
            },
            To => "to",
            ToBelow => "to directed downwards",
            From => "coming from something, belonging to something",
            Of => "related to something, belonging to something",
            With => "with",
        }
    }
}
