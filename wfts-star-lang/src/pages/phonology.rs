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
            sections: vec![
                Section {
                    title: "Consonants".to_owned(),
                    id: Id::new("consonants").unwrap(),
                    body: vec![
                        Paragraph(
                            "Classical Star Language had 24 phonemic \
                             consonants.",
                        )
                        .to_dyn(),
                        Table {
                            title: "Classical Star Language Consonants"
                                .to_owned(),
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
                    children: vec![Section {
                        title: "Allophonic Variation".to_owned(),
                        id: Id::new("consonant-allophony").unwrap(),
                        body: vec![
                            Paragraph(
                                "Velar consonants might become uvular before \
                                 /a(ː)/. Labiovelar nasal /ŋʷ/ might be \
                                 pronounced as a doubly articulated nasal \
                                 [ŋ͡m].",
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
                                "At later times, when adjacent to palatal \
                                 sounds, many speakers pronounce the alveolar \
                                 fricative /s/ as an alveolo-palatal sibilant \
                                 [ɕ], the velar fricative /x/ as a palatal \
                                 non-sibilant fricative [ç], and the \
                                 labiovelar fricative /xʷ/ as a labiopalatal \
                                 non-sibilant fricative [çʷ]. Adjacent sounds \
                                 are not deleted (yet). The geographic \
                                 distribution of this change seems to be \
                                 non-uniform and quite widespread. If one \
                                 speaker in one area features this variation, \
                                 another speaker might not feature it.",
                            )
                            .to_dyn(),
                        ]
                        .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Vowels".to_owned(),
                    id: Id::new("vowels").unwrap(),
                    body: vec![
                        Paragraph(
                            "Classical Star Language had 6 phonemic vowels. \
                             The vowels do not contrast on backness or \
                             rounding, only on height and length.",
                        )
                        .to_dyn(),
                        Table {
                            title: "Classical Star Language Vowels".to_owned(),
                            entries: vec![
                                vec![
                                    Entry::default(),
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Short",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Long",
                                    },
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Closed",
                                    },
                                    Entry::new("i"),
                                    Entry::new("iː"),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Mid",
                                    },
                                    Entry::new("e"),
                                    Entry::new("eː"),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Open",
                                    },
                                    Entry::new("a"),
                                    Entry::new("aː"),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![Section {
                        title: "Allophonic Variation".to_owned(),
                        id: Id::new("vowel-allophony").unwrap(),
                        body: vec![
                            Paragraph(
                                "Vowels backness and roundness vary depending \
                                 on the consonant before them.",
                            )
                            .to_dyn(),
                            Table {
                                title: "Vowel Allophones".to_owned(),
                                entries: vec![
                                    vec![
                                        Entry::default(),
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "After /cʰ, cʼ, ɲ, j/ (and \
                                                   [ɕ, ç, çʷ])",
                                        },
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "Default Case",
                                        },
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "After /kʰ, kʼ, ŋ, x/",
                                        },
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "After /kʷʰ, kʷʼ, ŋʷ, xʷ, w/",
                                        },
                                    ],
                                    vec![
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "/i/",
                                        },
                                        Entry::new("[i ~ ɪ]"),
                                        Entry::new("[ɨ ~ ɨ̞]"),
                                        Entry::new("[ɯ ~ ɯ̽]"),
                                        Entry::new("[u ~ ʊ]"),
                                    ],
                                    vec![
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "/iː/",
                                        },
                                        Entry::new("[iː]"),
                                        Entry::new("[ɨː]"),
                                        Entry::new("[ɯː]"),
                                        Entry::new("[uː]"),
                                    ],
                                    vec![
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "/e/",
                                        },
                                        Entry::new("[e̞ ~ e̽]"),
                                        Entry::new("[ə]"),
                                        Entry::new("[ɤ̞ ~ ɤ̽]"),
                                        Entry::new("[o̞ ~ o̽]"),
                                    ],
                                    vec![
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "/eː/",
                                        },
                                        Entry::new("[e̞ː]"),
                                        Entry::new("[əː ~ ɘː]"),
                                        Entry::new("[ɤ̞ə̯]"),
                                        Entry::new("[o̞ː]"),
                                    ],
                                    vec![
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "/a/",
                                        },
                                        Entry::new("[æ ~ æ̽]"),
                                        Entry::new("[ä ~ ɐ]"),
                                        Entry::new("[ɑ ~ ɑ̽]"),
                                        Entry::new("[ɒ ~ ɒ̝]"),
                                    ],
                                    vec![
                                        Entry {
                                            rowspan: 1,
                                            colspan: 1,
                                            header: true,
                                            data: "/aː/",
                                        },
                                        Entry::new("[æː]"),
                                        Entry::new("[äː]"),
                                        Entry::new("[ɑː]"),
                                        Entry::new("[ɒɔ̯]"),
                                    ],
                                ],
                            }
                            .to_dyn(),
                            Figure {
                                img: Image {
                                    alt: "Vowel allophones".to_owned(),
                                    src: StarLang
                                        .path()
                                        .append(
                                            Fragment::new("vowels.svg")
                                                .unwrap(),
                                        )
                                        .into(),
                                },
                                legend: "Trapezoid of vowel allophones.",
                            }
                            .to_dyn(),
                        ]
                        .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Phonotactics".to_owned(),
                    id: Id::new("phonotactics").unwrap(),
                    body: vec![
                        Paragraph(
                            "In general, Classical Star Language follows this \
                             syllable structure: (C)(C)(C)V(C)(C). There are, \
                             however, a few restrictions on the combination \
                             of kinds of phonemes. For instance, geminated \
                             (long) consonants are forbidden, and so, any \
                             sequence of repeated consonant simplifies to a \
                             single simple consonant; there is no length \
                             distinction. Vowels in hiatus cannot be the same \
                             as well. The table below lists the restrictions \
                             based on the kind of the consonants",
                        )
                        .to_dyn(),
                        Table {
                            title: "Syllabic Structure".to_owned(),
                            entries: vec![
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Outer Onset".blocking().to_dyn(),
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Medial Onset"
                                            .blocking()
                                            .to_dyn(),
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Inner Onset".blocking().to_dyn(),
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Nucleus".blocking().to_dyn(),
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Inner Coda".blocking().to_dyn(),
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Outer Coda".blocking().to_dyn(),
                                    },
                                ],
                                vec![
                                    Entry::new(
                                        UnorderedList(vec!["Aspirated"])
                                            .to_dyn(),
                                    ),
                                    Entry::new(
                                        UnorderedList(vec![
                                            "Aspirated",
                                            "Fricative",
                                            "Nasal",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    ),
                                    Entry {
                                        rowspan: 3,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec![
                                            "Approximant",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    },
                                    Entry {
                                        rowspan: 3,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec!["Vowel"])
                                            .to_dyn(),
                                    },
                                    Entry {
                                        rowspan: 3,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec![
                                            "Approximant",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    },
                                    Entry {
                                        rowspan: 6,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec![
                                            "Fricative",
                                            "Nasal",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    },
                                ],
                                vec![
                                    Entry::new(
                                        UnorderedList(vec!["Ejective"])
                                            .to_dyn(),
                                    ),
                                    Entry::new(
                                        UnorderedList(vec![
                                            "Ejective",
                                            "Fricative",
                                            "Nasal",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    ),
                                ],
                                vec![
                                    Entry::new(
                                        UnorderedList(vec!["Fricative", "∅"])
                                            .to_dyn(),
                                    ),
                                    Entry::new(
                                        UnorderedList(vec!["Nasal", "∅"])
                                            .to_dyn(),
                                    ),
                                ],
                                vec![
                                    Entry::new(
                                        UnorderedList(vec!["Aspirated"])
                                            .to_dyn(),
                                    ),
                                    Entry::new(
                                        UnorderedList(vec![
                                            "Aspirated",
                                            "Fricative",
                                            "Nasal",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    ),
                                    Entry {
                                        rowspan: 3,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec!["∅"]).to_dyn(),
                                    },
                                    Entry {
                                        rowspan: 3,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec!["/ɹ/"])
                                            .to_dyn(),
                                    },
                                    Entry {
                                        rowspan: 3,
                                        colspan: 1,
                                        header: false,
                                        data: UnorderedList(vec!["∅"]).to_dyn(),
                                    },
                                ],
                                vec![
                                    Entry::new(
                                        UnorderedList(vec!["Ejective"])
                                            .to_dyn(),
                                    ),
                                    Entry::new(
                                        UnorderedList(vec![
                                            "Ejective",
                                            "Fricative",
                                            "Nasal",
                                            "∅",
                                        ])
                                        .to_dyn(),
                                    ),
                                ],
                                vec![
                                    Entry::new(
                                        UnorderedList(vec!["Fricative", "∅"])
                                            .to_dyn(),
                                    ),
                                    Entry::new(
                                        UnorderedList(vec!["Nasal", "∅"])
                                            .to_dyn(),
                                    ),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
            ],
        }),
    );
}
