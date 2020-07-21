#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartOfSpeech {
    Noun(Noun),
    Adjective(Adjective),
    Pronoun(Pronoun),
    Postposition(Postposition),
    Conjuction(Conjunction),
    Verb(Verb),
    Adverb(Adverb),
    Root(Root),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Noun {
    pub class: NounClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NounClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Adjective {
    pub class: AdjectiveClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdjectiveClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pronoun {
    pub class: PronounClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PronounClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Postposition {
    pub class: PostpositionClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PostpositionClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Conjunction {
    pub class: ConjunctionClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConjunctionClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Verb {
    pub class: VerbClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VerbClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Adverb {
    pub class: AdverbClass,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AdverbClass {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Root {}
