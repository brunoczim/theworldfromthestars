use crate::{proto_divine::ProtoDivine, Lang};
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
            title: "Proto-Divine Language".to_owned(),
            body: vec![
                Paragraph(
                    "This article is about the ancestor of all languages: \
                     Proto-Divine. This language was spoken by the younger \
                     gods, and split into several languages as the group of \
                     gods was split by itself. It was never written, and so, \
                     it is a reconstructed language.",
                ),
                Paragraph(
                    "The reconstruction is mainly based on the languages of \
                     the later gods. However, some divine seer magic is also \
                     used, in order to gather visions from the youger gods \
                     speaking. Although the younger gods died to raise the \
                     next generation of gods, some memories were carried on \
                     through the generations, and those are also important.",
                ),
            ]
            .to_dyn(),
            sections: vec![
                Section {
                    title: "Phonology".to_dyn(),
                    body: Paragraph(Link {
                        text: "See this article.",
                        location: Location::from(
                            ProtoDivine
                                .path()
                                .append(Fragment::new("phonology").unwrap()),
                        ),
                    })
                    .to_dyn(),
                    children: vec![],
                    id: Id::new("phonology").unwrap(),
                },
                Section {
                    title: "Writing System".to_dyn(),
                    body: Paragraph(Link {
                        text: "See this article.",
                        location: Location::from(
                            ProtoDivine
                                .path()
                                .append(Fragment::new("writing").unwrap()),
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
                            ProtoDivine
                                .path()
                                .append(Fragment::new("grammar").unwrap()),
                        ),
                    })
                    .to_dyn(),
                    children: vec![],
                    id: Id::new("grammar").unwrap(),
                },
                Section {
                    title: "Dictionary".to_dyn(),
                    body: Paragraph(Link {
                        text: "See this article.",
                        location: Location::from(
                            ProtoDivine
                                .path()
                                .append(Fragment::new("dictionary").unwrap()),
                        ),
                    })
                    .to_dyn(),
                    children: vec![],
                    id: Id::new("dictionary").unwrap(),
                },
            ],
        }),
    );
}
