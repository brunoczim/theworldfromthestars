use crate::{proto_divine::ProtoDivine, Lang};
use wfts_pedia_ssg::{
    component::{
        table::{Entry, Table},
        text::{Link, Paragraph},
        Component,
    },
    location::{Id, InternalPath},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("writing/index.html").unwrap(),
        Node::Page(Page {
            title: "Proto-Divine Language Writing".to_owned(),
            body: vec![Paragraph(vec![
                "This article is about the writing of the ".to_dyn(),
                Link {
                    location: ProtoDivine.path().into(),
                    text: "Proto-Divine language",
                }
                .to_dyn(),
                ". The language was never itself written, but a notation \
                 system is used in order to transcribe words, sentences, \
                 speeches. This system, however, is arbitrary, while it will \
                 still make sense to those familiarized with latin script."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![Section {
                title: "Letter and Phoneme Correspondence".to_dyn(),
                body: vec![
                    Paragraph(
                        "The modern notation system is entirely based on the \
                         phonemic system, as with many reconstructions. And \
                         so, there is a one-to-one correspondence between \
                         letter and phoneme. An asterisk is usually placed \
                         before the letter to indicate it is a \
                         reconstruction. Below there is a table on that:",
                    )
                    .to_dyn(),
                    Table {
                        title: "Notation Letter and Phoneme Table",
                        entries: vec![
                            vec![
                                Entry {
                                    rowspan: 1,
                                    colspan: 1,
                                    header: true,
                                    data: "Letter",
                                },
                                Entry {
                                    rowspan: 1,
                                    colspan: 1,
                                    header: true,
                                    data: "Phoneme",
                                },
                            ],
                            vec![Entry::new("*a"), Entry::new("/a/")],
                            vec![Entry::new("*å"), Entry::new("/ɒ/")],
                            vec![Entry::new("*e"), Entry::new("/e/")],
                            vec![Entry::new("*f"), Entry::new("/f/")],
                            vec![Entry::new("*h"), Entry::new("/h/")],
                            vec![Entry::new("*i"), Entry::new("/i/")],
                            vec![Entry::new("*j"), Entry::new("/j/")],
                            vec![Entry::new("*k"), Entry::new("/k/")],
                            vec![Entry::new("*l"), Entry::new("/l/")],
                            vec![Entry::new("*m"), Entry::new("/m/")],
                            vec![Entry::new("*n"), Entry::new("/n/")],
                            vec![Entry::new("*ŋ"), Entry::new("/ŋ/")],
                            vec![Entry::new("*o"), Entry::new("/o/")],
                            vec![Entry::new("*p"), Entry::new("/p/")],
                            vec![Entry::new("*s"), Entry::new("/s/")],
                            vec![Entry::new("*t"), Entry::new("/t/")],
                            vec![Entry::new("*u"), Entry::new("/u/")],
                            vec![Entry::new("*w"), Entry::new("/w/")],
                        ],
                    }
                    .to_dyn(),
                ]
                .to_dyn(),
                children: vec![],
                id: Id::new("letter-phoneme-corresp").unwrap(),
            }],
        }),
    );
}
