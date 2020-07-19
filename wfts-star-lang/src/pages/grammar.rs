use crate::StarLang;
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        img::{Figure, Image},
        list::UnorderedList,
        table::{Entry, Table},
        text::{Link, Paragraph},
        Component,
    },
    location::{Fragment, Id, InternalPath},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("grammar/index.html").unwrap(),
        Node::Page(Page {
            title: String::from("Classical Star Language Grammar"),
            body: vec![Paragraph(vec![
                "This article is about the grammar of the ".to_dyn(),
                Link {
                    location: StarLang.path().into(),
                    text: "classical dialect of the Star Language",
                }
                .to_dyn(),
                ". Besides some innovations and new phrases, the dialect's \
                 grammar was considerably stable."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![Section {
                title: "Nouns".to_owned(),
                id: Id::new("nouns").unwrap(),
                body: vec![
                    Paragraph(
                        "Nouns in Classical Star Language are similar to \
                         nouns in English. They usually are the direct \
                         representative of things in the language, rather \
                         than referring to them in an indirect way like \
                         pronouns do. Another difference between them is that \
                         nouns can take adjectives, while pronouns don't.",
                    )
                    .to_dyn(),
                    Paragraph(
                        "Nouns vary in case, gender and number. Some nouns \
                         won't have a form in all genders or in all numbers, \
                         but will always inflect for case. The cases are:",
                    )
                    .to_dyn(),
                    UnorderedList(vec![
                        "Nominative",
                        "Accusative",
                        "Topical",
                        "Postpositional",
                    ])
                    .to_dyn(),
                    Paragraph(
                        "The nominative case is used when the noun is the \
                         subject (it agrees with the verb). The accusative is \
                         used when the noun is a (direct) object but without \
                         any preposition (it is an important argument to a \
                         verb, without agreement). Topical case is used when \
                         the noun is the topic of a clause, and not the \
                         \"comment\". Postpositional is used when a \
                         postposition follows the noun.",
                    )
                    .to_dyn(),
                ]
                .to_dyn(),
                children: vec![],
            }],
        }),
    );
}
