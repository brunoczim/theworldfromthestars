use crate::StarLang;
use wfts_lang::Lang;
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
            sections: vec![Section {
                title: "Consonants".to_owned(),
                body: vec![
                    Paragraph(
                        "Classical Star Language had 24 phonemic consonants.",
                    )
                    .to_dyn(),
                    Table {
                        title: "Classical Star Language Consonants".to_owned(),
                        entries: vec![
                            vec![
                                Entry {
                                    colspan: 2,
                                    rowspan: 2,
                                    header: true,
                                    data: "",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 2,
                                    header: true,
                                    data: "Labial",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 2,
                                    header: true,
                                    data: "Alveolar",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 2,
                                    header: true,
                                    data: "Palatal",
                                },
                                Entry {
                                    colspan: 2,
                                    rowspan: 1,
                                    header: true,
                                    data: "Velar/Uvular",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 2,
                                    header: true,
                                    data: "Pharyngeal",
                                },
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Plain",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Labial",
                                },
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 3,
                                    header: true,
                                    data: "Occlusive",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Aspirated",
                                },
                                Entry::new("pʰ"),
                                Entry::new("tʰ"),
                                Entry::new("cʰ"),
                                Entry::new("kʰ"),
                                Entry::new("kʷʰ"),
                                Entry::default(),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Ejective",
                                },
                                Entry::new("pʼ"),
                                Entry::new("tʼ"),
                                Entry::new("cʼ"),
                                Entry::new("kʼ"),
                                Entry::new("kʷʼ"),
                                Entry::default(),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Nasal",
                                },
                                Entry::new("m"),
                                Entry::new("n"),
                                Entry::new("ɲ"),
                                Entry::new("ŋ"),
                                Entry::new("ŋʷ"),
                                Entry::default(),
                            ],
                            vec![
                                Entry {
                                    colspan: 2,
                                    rowspan: 1,
                                    header: true,
                                    data: "Fricative",
                                },
                                Entry::new("ɸ"),
                                Entry::new("s"),
                                Entry::new(""),
                                Entry::new("x"),
                                Entry::new("xʷ"),
                                Entry::new("ħ"),
                            ],
                            vec![
                                Entry {
                                    colspan: 2,
                                    rowspan: 1,
                                    header: true,
                                    data: "Approximant",
                                },
                                Entry::new(""),
                                Entry::new("ɹ"),
                                Entry::new("j"),
                                Entry::new(""),
                                Entry::new("w"),
                                Entry::new("ʕ"),
                            ],
                        ],
                    }
                    .to_dyn(),
                ]
                .to_dyn(),
                id: Id::new("consonants").unwrap(),
                children: vec![Section {
                    title: "Allophonic Variation".to_owned(),
                    body: vec![
                        Paragraph(
                            "Velar consonants might become uvular before \
                             /a(ː)/. Labiovelar nasal /ŋʷ/ might be \
                             pronounced as a doubly articulated nasal [ŋ͡m].",
                        )
                        .to_dyn(),
                        Table {
                            title: "Velar/Uvular Allophones".to_owned(),
                            entries: vec![
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Phoneme",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Before /a(ː)/",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Elsewhere",
                                    },
                                ],
                                vec![
                                    Entry::new("/kʰ/"),
                                    Entry::new("[qʰ]"),
                                    Entry::new("[kʰ]"),
                                ],
                                vec![
                                    Entry::new("/kʷʰ/"),
                                    Entry::new("[qʷʰ]"),
                                    Entry::new("[kʷʰ]"),
                                ],
                                vec![
                                    Entry::new("/kʼ/"),
                                    Entry::new("[qʼ]"),
                                    Entry::new("[kʼ]"),
                                ],
                                vec![
                                    Entry::new("/kʷʼ/"),
                                    Entry::new("[qʷʼ]"),
                                    Entry::new("[kʷʼ]"),
                                ],
                                vec![
                                    Entry::new("/ŋ/"),
                                    Entry::new("[ɴ]"),
                                    Entry::new("[ŋ]"),
                                ],
                                vec![
                                    Entry::new("/ŋʷ/"),
                                    Entry::new("[ɴʷ ~ ɴ͡m]"),
                                    Entry::new("[ŋʷ ~ ŋ͡m]"),
                                ],
                                vec![
                                    Entry::new("/x/"),
                                    Entry::new("[χ]"),
                                    Entry::new("[x]"),
                                ],
                                vec![
                                    Entry::new("/xʷ/"),
                                    Entry::new("[χʷ]"),
                                    Entry::new("[xʷ]"),
                                ],
                                vec![
                                    Entry::new("/w/"),
                                    Entry::new("[w̠]"),
                                    Entry::new("[w]"),
                                ],
                            ],
                        }
                        .to_dyn(),
                        Paragraph(
                            "At later times, many speakers pronounce, when \
                             adjacent to palatal sounds, the alveolar \
                             fricative /s/ as an alveolo-palatal sibilant [ɕ] \
                             and the velar fricative /x/ as a palatal \
                             non-sibilant fricative [ç]. Adjacent sounds are \
                             not deleted.",
                        )
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    id: Id::new("consonant-allophony").unwrap(),
                    children: vec![],
                }],
            }],
        }),
    );
}
