use crate::{proto_divine::ProtoDivine, Lang};
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
                                "/j/ is always palatal. /k, ŋ/ are by default \
                                 velar [k, ŋ] (the same applies to voiced \
                                 allophone [g] of /k/). /h/ ranges between \
                                 debuccalized [h] and velar [x] by default, \
                                 if voiceless. When voiced, it is always \
                                 velar [ɣ] by default. Besides their default \
                                 pronunciation /k, h, ŋ/ can be palatalized.",
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
                    body: vec![].to_dyn(),
                    children: vec![].to_dyn(),
                },
                Section {
                    title: "Phonotactics".to_dyn(),
                    id: Id::new("phonotactics").unwrap(),
                    body: vec![].to_dyn(),
                    children: vec![],
                },
            ],
        }),
    );
}
