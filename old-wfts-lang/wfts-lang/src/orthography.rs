use std::hash::Hash;

pub trait Grapheme: Ord + Hash {
    fn to_unicode(&self) -> &str;
}

pub trait Word: Ord + Hash {
    type Grapheme: Grapheme;

    fn graphemes(&self) -> Vec<Self::Grapheme>;

    fn to_unicode(&self) -> String {
        self.graphemes().iter().map(Grapheme::to_unicode).collect()
    }
}

pub trait Orthography {
    type Grapheme: Grapheme;
    type Word: Word;

    fn make_word(
        &self,
        graphemes: &[Self::Grapheme],
    ) -> anyhow::Result<Self::Word>;

    fn parse_grapheme(&self, string: &str) -> anyhow::Result<Self::Grapheme>;

    fn parse_word(&self, graphemes: &[&str]) -> anyhow::Result<Self::Word> {
        let graphemes: anyhow::Result<Vec<_>> = graphemes
            .iter()
            .map(|string| self.parse_grapheme(string))
            .collect();
        self.make_word(&graphemes?)
    }
}
