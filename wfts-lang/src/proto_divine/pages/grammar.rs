use crate::{proto_divine::ProtoDivine, Lang};
use wfts_pedia_ssg::{
    component::{
        list::UnorderedList,
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
        InternalPath::parse("grammar/index.html").unwrap(),
        Node::Page(Page {
            title: "Proto-Divine Grammar".to_owned(),
            body: vec![Paragraph(vec![
                "This article is about the grammar of the ".to_dyn(),
                Link {
                    location: ProtoDivine.path().into(),
                    text: "Proto-Divine language",
                }
                .to_dyn(),
                "The language's grammar is polysynthetic. For instance, nouns \
                 are marked by case, but the case marker is an independent \
                 morpheme (and every morpheme is a syllable)."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![Section {
                title: "Grammatical Cases".to_dyn(),
                id: Id::new("cases").unwrap(),
                body: vec![
                    Paragraph(
                        "Nouns and adjectives are marked by a total of 14 \
                         cases:",
                    )
                    .to_dyn(),
                    UnorderedList(vec![
                        "Nominative",
                        "Accusative",
                        "Dative",
                        "Genitive",
                        "Instrumental",
                        "Comitative",
                        "Ablative",
                        "Allative",
                        "Adessive",
                        "Inessive",
                        "Supersessive",
                        "Perlative",
                        "Temporal",
                        "Vocative",
                    ])
                    .to_dyn(),
                    Paragraph(
                        "There are also prepositions, but they are applied \
                         only once on a noun phrase, besides that they do not \
                         become part of any word in the noun phrase. \
                         Prepositions are also regular, while which morphemes \
                         used as case endings depend on the noun phrase's \
                         gender.",
                    )
                    .to_dyn(),
                    Paragraph(
                        "The following table shows the case endings for each \
                         case and gender, as well the purpose of each case:",
                    )
                    .to_dyn(),
                    Table {
                        title: "Cases Purposes and Endings".to_owned(),
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
                                    data: "Ending for Divine Gender",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Ending for Mortal Gender",
                                },
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Purpose",
                                },
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Nominative",
                                },
                                Entry::new(""),
                                Entry::new(""),
                                Entry::new("Subject, usually the agent."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Accusative",
                                },
                                Entry::new("a"),
                                Entry::new("å"),
                                Entry::new(
                                    "Direct object, usually the \"direct\" \
                                     patient.",
                                ),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Dative",
                                },
                                Entry::new("i"),
                                Entry::new("jå"),
                                Entry::new(
                                    "Indirect object, usually the receiver \
                                     with movement.",
                                ),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Genitive",
                                },
                                Entry::new("tis"),
                                Entry::new("twos"),
                                Entry::new(
                                    "The main component of a generic \
                                     relation, usually the possessor.",
                                ),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Instrumental",
                                },
                                Entry::new("wa"),
                                Entry::new("wå"),
                                Entry::new(
                                    "Means used to perform the action, \
                                     usually a tool.",
                                ),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Comitative",
                                },
                                Entry::new("ja"),
                                Entry::new("jo"),
                                Entry::new("Companion on the action."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Ablative",
                                },
                                Entry::new("je"),
                                Entry::new("jo"),
                                Entry::new("Away from something."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Allative",
                                },
                                Entry::new("ŋi"),
                                Entry::new("ŋo"),
                                Entry::new("Into something."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Adessive",
                                },
                                Entry::new("ka"),
                                Entry::new("ku"),
                                Entry::new("At something."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Inessive",
                                },
                                Entry::new("kje"),
                                Entry::new("kjå"),
                                Entry::new("In something."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Supersessive",
                                },
                                Entry::new("kje"),
                                Entry::new("kja"),
                                Entry::new("On something."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Perlative",
                                },
                                Entry::new("pi"),
                                Entry::new("pe"),
                                Entry::new("Through something."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Temporal",
                                },
                                Entry::new("im"),
                                Entry::new("em"),
                                Entry::new("Time of an event."),
                            ],
                            vec![
                                Entry {
                                    colspan: 1,
                                    rowspan: 1,
                                    header: true,
                                    data: "Vocative",
                                },
                                Entry::new("o"),
                                Entry::new("on"),
                                Entry::new("Something called."),
                            ],
                        ],
                    }
                    .to_dyn(),
                ]
                .to_dyn(),
                children: vec![],
            }],
        }),
    );
}
