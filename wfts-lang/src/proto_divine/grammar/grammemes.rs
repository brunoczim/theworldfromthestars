use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Case {
    Nominative,
    Accusative,
    Dative,
    Genitive,
    Instrumental,
    Comitative,
    Ablative,
    Allative,
    Adessive,
    Inessive,
    Supersessive,
    Perlative,
    Temporal,
    Vocative,
}

impl fmt::Display for Case {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Case::*;

        fmt.pad(match self {
            Nominative => "nominative",
            Accusative => "accusative",
            Dative => "dative",
            Genitive => "genitive",
            Instrumental => "instrumental",
            Comitative => "comitative",
            Ablative => "ablative",
            Allative => "allative",
            Adessive => "adessive",
            Inessive => "inessive",
            Supersessive => "supersessive",
            Perlative => "perlative",
            Temporal => "temporal",
            Vocative => "vocative",
        })
    }
}

impl Case {
    pub const ALL: &'static [Self] = &[
        Case::Nominative,
        Case::Accusative,
        Case::Dative,
        Case::Genitive,
        Case::Instrumental,
        Case::Comitative,
        Case::Ablative,
        Case::Allative,
        Case::Adessive,
        Case::Inessive,
        Case::Supersessive,
        Case::Perlative,
        Case::Temporal,
        Case::Vocative,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gender {
    Divine,
    Mortal,
}

impl fmt::Display for Gender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Gender::*;

        fmt.pad(match self {
            Divine => "divine",
            Mortal => "mortal",
        })
    }
}

impl Gender {
    pub const ALL: &'static [Self] = &[Gender::Divine, Gender::Mortal];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Number {
    Singular,
    Plural,
    Nullar,
    Collective,
}

impl fmt::Display for Number {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Number::*;

        fmt.pad(match self {
            Singular => "singular",
            Plural => "plural",
            Nullar => "nullar",
            Collective => "collective",
        })
    }
}

impl Number {
    pub const ALL: &'static [Self] =
        &[Number::Singular, Number::Plural, Number::Nullar, Number::Collective];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tense {
    Present,
    Past,
    RemotePast,
    Future,
    RemoteFuture,
}

impl fmt::Display for Tense {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Tense::*;

        fmt.pad(match self {
            Present => "present",
            Past => "past",
            RemotePast => "remote past",
            Future => "future",
            RemoteFuture => "remote future",
        })
    }
}

impl Tense {
    pub const ALL: &'static [Self] = &[
        Tense::Present,
        Tense::Past,
        Tense::RemotePast,
        Tense::Future,
        Tense::RemoteFuture,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mood {
    Indicative,
    Subjunctive,
    Optative,
    Imperative,
}

impl fmt::Display for Mood {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Mood::*;

        fmt.pad(match self {
            Indicative => "indicative",
            Subjunctive => "subjunctive",
            Optative => "optative",
            Imperative => "imperative",
        })
    }
}

impl Mood {
    pub const ALL: &'static [Self] = &[
        Mood::Indicative,
        Mood::Subjunctive,
        Mood::Optative,
        Mood::Imperative,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Aspect {
    Perfect,
    Continuous,
    Habitual,
}

impl fmt::Display for Aspect {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Aspect::*;

        fmt.pad(match self {
            Perfect => "perfect",
            Continuous => "continuous",
            Habitual => "habitual",
        })
    }
}

impl Aspect {
    pub const ALL: &'static [Self] =
        &[Aspect::Perfect, Aspect::Continuous, Aspect::Habitual];
}
