mod phonology;

use self::phonology::{Phoneme, Syllable, Word};
use std::collections::HashMap;
use wfts_lang::{Lang, LangCode};
use wfts_pedia_ssg::{
    component::{text::Paragraph, Component},
    location::Fragment,
    page::Page,
    site::{Directory, Node},
};

#[derive(Debug, Clone)]
pub struct StarLang;

impl StarLang {
    fn make_index(&self, dir: &mut Directory) {
        dir.contents.insert(
            Fragment::new("index.html").unwrap(),
            Node::Page(Page {
                title: String::from("Classical Star Language"),
                body: vec![
                    Paragraph(
                        "This article is about the classical dialect of the \
                         Star Language, spoken by the Star Folk people. It is \
                         the earlier form of the Star Language, and was \
                         spoken during the first to third century (circa 0 ─ \
                         250). Classical Star Language is the first human \
                         language ever, and the ancestor of any other human \
                         language.",
                    ),
                    Paragraph(
                        "As the world started with the Star Folk people, the \
                         gods gave them the insights required to communicate \
                         with each other and form a language. The classical \
                         period goes up to year 250, when the single Star \
                         Folk people began to split itself in three major \
                         groups.",
                    ),
                ]
                .to_dyn(),
                sections: Vec::new(),
            }),
        );
    }
}

impl Lang for StarLang {
    type Character = Phoneme;
    type Phoneme = Phoneme;
    type Syllable = Syllable;
    type Word = Word;

    fn code(&self) -> LangCode {
        LangCode::parse("str-cls").unwrap()
    }

    fn subsite(&self) -> Directory {
        let mut dir = Directory { contents: HashMap::new() };
        self.make_index(&mut dir);
        dir
    }
}
