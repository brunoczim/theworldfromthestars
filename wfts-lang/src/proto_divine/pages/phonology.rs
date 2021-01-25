use crate::{proto_divine::ProtoDivine, Lang};
use wfts_pedia_ssg::{
    component::{
        table::{Entry, Table},
        text::{Link, Paragraph},
        Component,
    },
    location::{Fragment, Id, InternalLoc, InternalPath},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("phonology/index.html").unwrap(),
        Node::Page(Page {
            title: "Proto-Divine Phonology".to_owned(),
            body: vec![Paragraph(vec![
                "This article is about the phonology of the ".to_dyn(),
                Link {
                    location: ProtoDivine.path().into(),
                    text: "Proto-Divine language",
                }
                .to_dyn(),
                ". According to seer magic, the language seems to have been \
                 phonological stable, except for the development of \
                 palatalization before the split."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![
                Section {
                    title: "Consonants".to_dyn(),
                    id: Id::new("consonants").unwrap(),
                    body: vec![
                        Paragraph("Proto-Divine had 12 phonemic consonants.")
                            .to_dyn(),
                        Table {
                            title: "Proto-Divine Consonants".to_owned(),
                            entries: vec![
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Labial",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Coronal",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Dorsal",
                                    },
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Plosive",
                                    },
                                    Entry::new("p"),
                                    Entry::new("t"),
                                    Entry::new("k"),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Fricative",
                                    },
                                    Entry::new("f"),
                                    Entry::new("s"),
                                    Entry::new("h"),
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
                                    Entry::new("ŋ"),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Approximant",
                                    },
                                    Entry::new("w"),
                                    Entry::new("l"),
                                    Entry::new("j"),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![Section {
                        title: "Allophonic Variation".to_dyn(),
                        id: Id::new("consonant-allophony").unwrap(),
                        body: vec![
                            Paragraph(
                                "Sonorants /m, n, ŋ, w, l, j/ are always \
                                 voiced, while obstruents /p, t, k, f, s, h/ \
                                 can be either voiced or voiceless. By \
                                 default, they are voiceless, but between \
                                 voiced sounds (i.e. sonorants and vowels) \
                                 they are voiced. Plosives are not aspirated.",
                            )
                            .to_dyn(),
                            Paragraph(
                                "/p/ is always bilabial [p] when voiceless, \
                                 bilabial [b] when voiced. /f/ probably had a \
                                 variation of bilabial [ɸ] and labiodental \
                                 [f], and the voiced allophone had a \
                                 variation between [β] and [v] as well. /w/ \
                                 would vary between labiovelar [w], bilabial \
                                 [β̞], and labiodental [ʋ]. /m/ was always \
                                 bilabial [m].",
                            )
                            .to_dyn(),
                            Paragraph(
                                "/l/ could be pronounced as a lateral \
                                 approximant [l] or as a tap [ɾ], where the \
                                 tap tended to be used in unstressed, fast \
                                 pronunciation. /t, n, l/ varied as dental, \
                                 alveolar and post-alveolar (also valid for \
                                 voiced allophone [d] of /t/). However, /s/ \
                                 was always alveolar (also valid for voiced \
                                 allophone [z] of /s/).",
                            )
                            .to_dyn(),
                            Paragraph(
                                "/j/ is always palatal [j]. /k, ŋ/ are by \
                                 default velar [k, ŋ] (the same applies to \
                                 voiced allophone [g] of /k/). /h/ ranges \
                                 between debuccalized [h] and velar [x] by \
                                 default, if voiceless. When voiced, it is \
                                 always velar [ɣ] by default. Besides their \
                                 default pronunciation /k, h, ŋ/ can be \
                                 palatalized.",
                            )
                            .to_dyn(),
                            Paragraph(
                                "Palatalization happens when /k, h, ŋ/ are \
                                 followed or follows a palatal sound. Such \
                                 sound can be /j/, front vowels /a, e, i/ or \
                                 another palatalized /k, h, ŋ/. Palatalized \
                                 allophones are always true palatal. \
                                 Palatalized /k/ is voiceless [c] and voiced \
                                 [ɟ]. Palatalized /h/ is voiceless [ç] and \
                                 voiced [ʝ]. Palatalized /ŋ/ is [ɲ].",
                            )
                            .to_dyn(),
                            Paragraph(
                                "Allophone [β] of /f/ is fricated, while \
                                 allophone [β̞] of /w/ is a pure approximant, \
                                 and so, they are distinguished. The same \
                                 applies to the allophone [ʝ] of /h/ \
                                 (fricated) and to /j/ (pure approximant), \
                                 and so those are distinguished too.",
                            )
                            .to_dyn(),
                        ]
                        .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Vowels".to_dyn(),
                    id: Id::new("vowels").unwrap(),
                    body: vec![
                        Paragraph("Proto-Divine had 6 phonemic vowels.")
                            .to_dyn(),
                        Table {
                            title: "Proto-Divine Vowels".to_owned(),
                            entries: vec![
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Front/Unrounded",
                                    },
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Back/Rounded",
                                    },
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "High",
                                    },
                                    Entry::new("i"),
                                    Entry::new("u"),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Mid",
                                    },
                                    Entry::new("e"),
                                    Entry::new("o"),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Open",
                                    },
                                    Entry::new("a"),
                                    Entry::new("ɒ"),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![Section {
                        title: "Allophonic Variation".to_dyn(),
                        id: Id::new("consonant-allophony").unwrap(),
                        body: vec![
                            Paragraph(
                                "Front vowels get slightly lowered when \
                                 following or followed by /j/. Back vowels \
                                 get slightly lowered when following or \
                                 followed by /w/. For instance, /i/ is close \
                                 [i] by default, near-close [ɪ] in the \
                                 vicinity of /j/. Similarly, /u/ is close [u] \
                                 by default, but near-close [ʊ] in the \
                                 vicinity of /w/.",
                            )
                            .to_dyn(),
                            Paragraph(
                                "/e/ by default varies between close-mid [e] \
                                 and mid [e̞], but in the vicinity of /j/, it \
                                 is always mid [e̞]. In the same way, /o/ by \
                                 default varies between close mid [o] and mid \
                                 [o̞], but it is mid [o̞] in the vicinity of \
                                 /w/.",
                            )
                            .to_dyn(),
                            Paragraph(
                                "/a/ by default varies between near-open [æ] \
                                 and open [a], but in the vicinity of /j/, it \
                                 is always open [a]. In the same way, /ɒ/ by \
                                 default varies between near-open [ɒ̝] and \
                                 open [ɒ], but it is open [ɒ] in the vicinity \
                                 of /w/.",
                            )
                            .to_dyn(),
                        ]
                        .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Phonotactics".to_dyn(),
                    id: Id::new("phonotactics").unwrap(),
                    body: vec![Paragraph(vec![
                        "Syllable structure of the language is (C)(C)V(C), or \
                         more specifically, (O)(S)V(C). /O/ is any obstruent \
                         /p, t, k, f, s, h/, while /S/ is any sonorant /m, n, \
                         ŋ, w, l, j/, /V/ is any vowel /a, e, i, ɒ, o, u/, \
                         and /C/ is either /O/ or /S/. If boundary between \
                         two consecutive syllables /VCV/ is found like this, \
                         for example, there is a difference between /V.CV/ \
                         and /VC.V/ in terms of prosody. "
                            .to_dyn(),
                        Link {
                            location: InternalLoc {
                                path: ProtoDivine.path().append(
                                    Fragment::new("phonology").unwrap(),
                                ),
                                id: Some(Id::new("prosody").unwrap()),
                            }
                            .into(),
                            text: "See below",
                        }
                        .to_dyn(),
                        ".".to_dyn(),
                    ])
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Prosody".to_dyn(),
                    id: Id::new("prosody").unwrap(),
                    body: vec![
                        Paragraph(vec![
                            "Proto-Divine is a stress-accent language, but \
                             the stress is fixed. The first syllable of a \
                             word is always stressed, odd syllables other \
                             than the first carry a secondary stress. Stress \
                             might help to determine where are word \
                             boundaries. As stated in "
                                .to_dyn(),
                            Link {
                                location: InternalLoc::parse("#phonotactics")
                                    .unwrap()
                                    .into(),
                                text: "phonotactics section",
                            }
                            .to_dyn(),
                            ", it makes difference to which syllable a \
                             consonant belongs."
                                .to_dyn(),
                        ])
                        .to_dyn(),
                        Paragraph(vec![
                            "More specifically, in boundaries like /VCV/ or \
                             /VOSV/, it makes a difference whether it is \
                             /V.CV/ and /V.OSV/ or if it is /VC.V/ and \
                             /VO.SV/. This difference is manifested through \
                             placement of a secondary stress in syllable \
                             boundaries inside a word. Such difference is \
                             likely caused because every syllable in \
                             Proto-Divine has an independent form. "
                                .to_dyn(),
                            Link {
                                location: ProtoDivine
                                    .path()
                                    .append(
                                        Fragment::new("morphology").unwrap(),
                                    )
                                    .into(),
                                text: "See morphology",
                            }
                            .to_dyn(),
                            ".".to_dyn(),
                        ])
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
            ],
        }),
    );
}
