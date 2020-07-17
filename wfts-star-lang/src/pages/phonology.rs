use crate::StarLang;
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        text::{Link, Paragraph},
        Component,
    },
    location::InternalPath,
    page::Page,
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("phonology/index.html").unwrap(),
        Node::Page(Page {
            title: String::from("Classical Star Language Phonology"),
            body: vec![Paragraph(vec![
                "This article is about the phonology of the ".to_dyn(),
                Link {
                    location: StarLang.path().into(),
                    text: "classical dialect of the Star Language",
                }
                .to_dyn(),
                ". The dialect had a very stable phonology, whose only big \
                 change was the allophonic palatalization of fricatives in \
                 the final years."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![],
        }),
    );
}
