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
                "The language's grammar is agglutinative. For instance, nouns \
                 are marked by case, but the case marker is an independent \
                 morpheme (and every morpheme is a syllable). Here, ⟨∅⟩ means \
                 \"empty set\", in this case, \"empty sequence\"."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![
                Section {
                    title: "Grammatical Cases".to_dyn(),
                    id: Id::new("cases").unwrap(),
                    body: vec![
                        Paragraph(
                            "Nouns, adjectives and pronouns are marked by a \
                             total of 14 cases:",
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
                            "There are also prepositions, but they are \
                             applied only once on a noun phrase, besides that \
                             they do not become part of any word in the noun \
                             phrase. Prepositions are also regular, while \
                             which morphemes used as case endings depend on \
                             the noun phrase's gender.",
                        )
                        .to_dyn(),
                        Paragraph(
                            "The following table shows the case endings for \
                             each case and gender, if regular noun, adjective \
                             or pronoun, as well the purpose of each case \
                             (⟨∅⟩ means empty):",
                        )
                        .to_dyn(),
                        Table {
                            title: "Case Purposes and Endings".to_owned(),
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
                                    Entry::new("∅"),
                                    Entry::new("∅"),
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
                                        "Direct object, usually the \
                                         \"direct\" patient.",
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
                                        "Indirect object, usually the \
                                         receiver with movement.",
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
                                    Entry::new("ja"),
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
                },
                Section {
                    title: "Numbers".to_dyn(),
                    id: Id::new("numbers").unwrap(),
                    body: vec![
                        Paragraph(
                            "Nouns, adjectives and pronouns are marked by 4 \
                             numbers: singular, plural, nullar and \
                             collective. Plural endings differ by gender, as \
                             with cases. The following table shows the number \
                             endings for each number and gender, if regular \
                             noun, adjective or pronoun, as well the purpose \
                             of each number (⟨∅⟩ means empty):",
                        )
                        .to_dyn(),
                        Table {
                            title: "Number Purposes and Endings".to_owned(),
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
                                        data: "Singular",
                                    },
                                    Entry::new("∅"),
                                    Entry::new(
                                        "u (for adjectives and gender \
                                         variable nouns), ∅ (for gender \
                                         non-variable nouns).",
                                    ),
                                    Entry::new("A single item."),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Plural",
                                    },
                                    Entry::new("so"),
                                    Entry::new("så"),
                                    Entry::new(
                                        "Many items, not necessarily all.",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Nullar",
                                    },
                                    Entry::new("nu"),
                                    Entry::new("no"),
                                    Entry::new("No item."),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Collective",
                                    },
                                    Entry::new("su"),
                                    Entry::new("så"),
                                    Entry::new("All items as a collective."),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Genders".to_dyn(),
                    id: Id::new("genders").unwrap(),
                    body: vec![Paragraph(
                        "Nouns and adjectives are have a gender. Possible \
                         genders are two: divine and mortal. Genders do not \
                         have a special morpheme marking them. Gender are \
                         inferred by which set of morphemes was chosen to \
                         mark case and number.",
                    )
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Tenses".to_dyn(),
                    id: Id::new("tenses").unwrap(),
                    body: vec![
                        Paragraph(
                            "Verbs are marked by tense. The five tenses are: \
                             present, past, future, remote-past, \
                             remote-future. Although the same morphemes are \
                             attached for each tense, they do become part of \
                             the word. Below there is a table of tense \
                             endings and purposes:",
                        )
                        .to_dyn(),
                        Table {
                            title: "Tense Purposes and Endings".to_owned(),
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
                                        data: "Ending ",
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
                                        data: "Present",
                                    },
                                    Entry::new("∅"),
                                    Entry::new(
                                        "Something that happens/is happening \
                                         in the present.",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Past",
                                    },
                                    Entry::new("at"),
                                    Entry::new(
                                        "Something that happened/was \
                                         happening in the past, but not so \
                                         far.",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Future",
                                    },
                                    Entry::new("le"),
                                    Entry::new(
                                        "Something that will happen in the \
                                         past, but not so far.",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Remote-Past",
                                    },
                                    Entry::new("jat"),
                                    Entry::new(
                                        "Something that happened/was \
                                         happening in the past, but far back \
                                         in time.",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Remote-Future",
                                    },
                                    Entry::new("led"),
                                    Entry::new(
                                        "Something that will happen in the \
                                         future, but far ahead in time.",
                                    ),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Moods".to_dyn(),
                    id: Id::new("moods").unwrap(),
                    body: vec![
                        Paragraph(
                            "Verbs can have moods by suffixing a morpheme. \
                             Although always the same morphemes are used to \
                             mark mood, they become part of the word. There \
                             are four moods: indicative, subjunctive, \
                             optative and imperative. Below there is a table \
                             of mood endings and purposes:",
                        )
                        .to_dyn(),
                        Table {
                            title: "Mood Purposes and Endings".to_owned(),
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
                                        data: "Ending ",
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
                                        data: "Indicative",
                                    },
                                    Entry::new("∅"),
                                    Entry::new("Certain, real, factual."),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Subjunctive",
                                    },
                                    Entry::new("jal"),
                                    Entry::new("Uncertain, hypothetical."),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Imperative",
                                    },
                                    Entry::new("ok"),
                                    Entry::new("Order, command."),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Optative",
                                    },
                                    Entry::new("maw"),
                                    Entry::new("Wish, hope, request."),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Aspects".to_dyn(),
                    id: Id::new("aspects").unwrap(),
                    body: vec![
                        Paragraph(
                            "Verbs are marked with aspects by suffixing a \
                             morpheme. Although always the same morphemes are \
                             used to mark mood, they become part of the word. \
                             There are four, aspects: perfect, continuous and \
                             habitual. Below there is a table of aspect \
                             endings and purposes:",
                        )
                        .to_dyn(),
                        Table {
                            title: "Aspect Purposes and Endings".to_owned(),
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
                                        data: "Ending ",
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
                                        data: "Perfect",
                                    },
                                    Entry::new("∅"),
                                    Entry::new(
                                        "Viewing an action outside of its \
                                         time frame.",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Continuous",
                                    },
                                    Entry::new("oŋ"),
                                    Entry::new(
                                        "Viewing an action inside of its time \
                                         frame but as a single indivisible \
                                         action (generally in a specific \
                                         point in time).",
                                    ),
                                ],
                                vec![
                                    Entry {
                                        colspan: 1,
                                        rowspan: 1,
                                        header: true,
                                        data: "Habitual",
                                    },
                                    Entry::new("eŋ"),
                                    Entry::new(
                                        "Viewing an action inside of its time \
                                         but such that the action is composed \
                                         of repeated actions (generally \
                                         habitual).",
                                    ),
                                ],
                            ],
                        }
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Nouns".to_dyn(),
                    id: Id::new("nouns").unwrap(),
                    body: vec![Paragraph(
                        "Nouns are direct names to things. They all have a \
                         gender, and vary in case and number. Some nouns have \
                         a fixed gender, some can vary in gender.",
                    )
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Adjectives".to_dyn(),
                    id: Id::new("adjectives").unwrap(),
                    body: vec![Paragraph(
                        "Adjectives gives more information and details about \
                         nouns. They all vary in case, number and gender.",
                    )
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Verbs".to_dyn(),
                    id: Id::new("verbs").unwrap(),
                    body: vec![Paragraph(
                        "Verbs are words that connect the \"arguments\" of a \
                         clause, and generally express an action or state. \
                         Verbs vary in tense, person, aspect and mood. \
                         Persons are the traditional first, second and third \
                         person. Additionally, there is a nominalized form of \
                         each verb, whose exactly meaning depends on the \
                         preposition before it.",
                    )
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Prepositions".to_dyn(),
                    id: Id::new("prepositions").unwrap(),
                    body: vec![Paragraph(
                        "Prepositions are words that goes before another \
                         words to indicate either location, time, or to \
                         modify meaning to something else.",
                    )
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Pronouns".to_dyn(),
                    id: Id::new("pronouns").unwrap(),
                    body: vec![Paragraph(
                        "Pronouns are indirect references to things in the \
                         speech.",
                    )
                    .to_dyn()]
                    .to_dyn(),
                    children: vec![],
                },
            ],
        }),
    );
}
