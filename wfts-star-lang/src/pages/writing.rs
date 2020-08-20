use crate::{component::WithStarAlphabet, StarLang};
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        table::{Entry, Table},
        text::{Link, Paragraph},
        Component,
    },
    location::{Id, InternalPath, Location},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("writing/index.html").unwrap(),
        Node::Page(Page {
            title: "Classical Star Language Writing System".to_owned(),
            body: "This page is about the writing system of the Classical \
                   Star Language. The writing system uses a phonemic alphabet \
                   principle. This means that each symbol (letter) gets \
                   mapped to one phonemic unit, as each phonemic unit gets \
                   mapped to one symbol."
                .blocking()
                .to_dyn(),
            sections: vec![Section {
                title: "List Of Letters In Alphabetic Order".to_dyn(),
                id: Id::new("list-of-letters").unwrap(),
                body: vec![
                    Paragraph(vec![
                        "See also ".to_dyn(),
                        Link {
                            location: Location::internal(format!(
                                "{}/phonology",
                                StarLang.path()
                            )),
                            text: "phonology of Classical Star Language",
                        }
                        .to_dyn(),
                        ".".to_dyn(),
                    ])
                    .to_dyn(),
                    Table {
                        title: "Letters And Sound Correspondences",
                        entries: vec![
                            vec![
                                Entry {
                                    data: "Letter".to_dyn(),
                                    header: true,
                                    colspan: 1,
                                    rowspan: 1,
                                },
                                Entry {
                                    data: "Romanized".to_dyn(),
                                    header: true,
                                    colspan: 1,
                                    rowspan: 1,
                                },
                                Entry {
                                    data: "Phoneme".to_dyn(),
                                    header: true,
                                    colspan: 1,
                                    rowspan: 1,
                                },
                                Entry {
                                    data: "Allophones".to_dyn(),
                                    header: true,
                                    colspan: 1,
                                    rowspan: 1,
                                },
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("b").to_dyn()),
                                Entry::new("b".to_dyn()),
                                Entry::new("/pʼ/".to_dyn()),
                                Entry::new("[pʼ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ǵ").to_dyn()),
                                Entry::new("ǵ".to_dyn()),
                                Entry::new("/kʷʼ/".to_dyn()),
                                Entry::new("[kʷʼ, qʷʼ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("d").to_dyn()),
                                Entry::new("d".to_dyn()),
                                Entry::new("/tʼ/".to_dyn()),
                                Entry::new("[tʼ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("j").to_dyn()),
                                Entry::new("j".to_dyn()),
                                Entry::new("/cʼ/".to_dyn()),
                                Entry::new("[cʼ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("g").to_dyn()),
                                Entry::new("g".to_dyn()),
                                Entry::new("/kʼ/".to_dyn()),
                                Entry::new("[kʼ, qʼ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("p").to_dyn()),
                                Entry::new("p".to_dyn()),
                                Entry::new("/pʰ/".to_dyn()),
                                Entry::new("[pʰ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ḱ").to_dyn()),
                                Entry::new("ḱ".to_dyn()),
                                Entry::new("/kʷʰ/".to_dyn()),
                                Entry::new("[kʷʰ, qʷʰ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("t").to_dyn()),
                                Entry::new("t".to_dyn()),
                                Entry::new("/tʰ/".to_dyn()),
                                Entry::new("[tʰ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("c").to_dyn()),
                                Entry::new("c".to_dyn()),
                                Entry::new("/cʰ/".to_dyn()),
                                Entry::new("[cʰ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("k").to_dyn()),
                                Entry::new("k".to_dyn()),
                                Entry::new("/kʰ/".to_dyn()),
                                Entry::new("[kʰ, qʰ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("m").to_dyn()),
                                Entry::new("m".to_dyn()),
                                Entry::new("/m/".to_dyn()),
                                Entry::new("[m]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ḿ").to_dyn()),
                                Entry::new("ḿ".to_dyn()),
                                Entry::new("/ŋʷ/".to_dyn()),
                                Entry::new("[ŋʷ ~ ŋ͡mʷ, ɴʷ ~ ɴ͡mʷ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("n").to_dyn()),
                                Entry::new("n".to_dyn()),
                                Entry::new("/n/".to_dyn()),
                                Entry::new("[n]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ń").to_dyn()),
                                Entry::new("ń".to_dyn()),
                                Entry::new("/ɲ/".to_dyn()),
                                Entry::new("[ɲ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ŋ").to_dyn()),
                                Entry::new("ŋ".to_dyn()),
                                Entry::new("/ŋ/".to_dyn()),
                                Entry::new("[ŋ, ɴ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("f").to_dyn()),
                                Entry::new("f".to_dyn()),
                                Entry::new("/f/".to_dyn()),
                                Entry::new("[f]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ẋ").to_dyn()),
                                Entry::new("ẋ".to_dyn()),
                                Entry::new("/xʷ/".to_dyn()),
                                Entry::new("[xʷ, χʷ, çʷ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("w").to_dyn()),
                                Entry::new("w".to_dyn()),
                                Entry::new("/w/".to_dyn()),
                                Entry::new("[w]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("s").to_dyn()),
                                Entry::new("s".to_dyn()),
                                Entry::new("/s/".to_dyn()),
                                Entry::new("[s, ɕ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("r").to_dyn()),
                                Entry::new("r".to_dyn()),
                                Entry::new("/ɹ/".to_dyn()),
                                Entry::new("[ɹ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("y").to_dyn()),
                                Entry::new("y".to_dyn()),
                                Entry::new("/j/".to_dyn()),
                                Entry::new("[j]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("í").to_dyn()),
                                Entry::new("í".to_dyn()),
                                Entry::new("/iː/".to_dyn()),
                                Entry::new("[iː, ɨː, ɯə̯, uː]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("x").to_dyn()),
                                Entry::new("x".to_dyn()),
                                Entry::new("/x/".to_dyn()),
                                Entry::new("[x, χ, ç]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("i").to_dyn()),
                                Entry::new("i".to_dyn()),
                                Entry::new("/i/".to_dyn()),
                                Entry::new("[i, ɨ, ɯ, u]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("é").to_dyn()),
                                Entry::new("é".to_dyn()),
                                Entry::new("/eː/".to_dyn()),
                                Entry::new("[eː, əː, ɤ̞ə̯, o̞ː]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("h").to_dyn()),
                                Entry::new("h".to_dyn()),
                                Entry::new("/ħ/".to_dyn()),
                                Entry::new("[ħ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("e").to_dyn()),
                                Entry::new("e".to_dyn()),
                                Entry::new("/e/".to_dyn()),
                                Entry::new("[e, ə, ɤ̞, o̞]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("ŕ").to_dyn()),
                                Entry::new("ŕ".to_dyn()),
                                Entry::new("/ʕ/".to_dyn()),
                                Entry::new("[ʕ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("a").to_dyn()),
                                Entry::new("a".to_dyn()),
                                Entry::new("/a/".to_dyn()),
                                Entry::new("[æ, ä, ɑ, ɒ]".to_dyn()),
                            ],
                            vec![
                                Entry::new(WithStarAlphabet("á").to_dyn()),
                                Entry::new("á".to_dyn()),
                                Entry::new("/aː/".to_dyn()),
                                Entry::new("[æː, äː, ɑː, ɒɔ̯]".to_dyn()),
                            ],
                        ],
                    }
                    .to_dyn(),
                ]
                .to_dyn(),
                children: vec![],
            }],
        }),
    )
}
