use crate::StarLang;
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        text::{Link, Paragraph},
        Component,
    },
    location::{Fragment, Id, InternalPath, Location},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("index.html").unwrap(),
        Node::Page(Page {
            title: "Classical Star Language".to_owned(),
            body: vec![
                Paragraph(
                    "This article is about the classical dialect of the Star \
                     Language, spoken by the Star Folk people. It is the \
                     earlier form of the Star Language, and was spoken during \
                     the first to third century (circa 0 ─ 250). Classical \
                     Star Language is the first human language ever, and the \
                     ancestor of any other human language.",
                ),
                Paragraph(
                    "As the world started with the Star Folk people, the gods \
                     gave them the insights required to communicate with each \
                     other and form a language. The classical period goes up \
                     to year 250, when the single Star Folk people began to \
                     split itself in three major groups.",
                ),
            ]
            .to_dyn(),
            sections: vec![
                Section {
                    title: "Phonology".to_dyn(),
                    body: Paragraph(Link {
                        text: "See this article.",
                        location: Location::from(
                            StarLang
                                .path()
                                .append(Fragment::new("phonology").unwrap()),
                        ),
                    })
                    .to_dyn(),
                    children: vec![],
                    id: Id::new("phonology").unwrap(),
                },
                Section {
                    title: "Grammar".to_dyn(),
                    body: Paragraph(Link {
                        text: "See this article.",
                        location: Location::from(
                            StarLang
                                .path()
                                .append(Fragment::new("grammar").unwrap()),
                        ),
                    })
                    .to_dyn(),
                    children: vec![],
                    id: Id::new("grammar").unwrap(),
                },
            ],
        }),
    );
}
