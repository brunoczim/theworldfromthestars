use crate::StarLang;
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        img::{Figure, Image},
        list::{OrderedList, UnorderedList},
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
        InternalPath::parse("grammar/index.html").unwrap(),
        Node::Page(Page {
            title: String::from("Classical Star Language Grammar"),
            body: vec![Paragraph(vec![
                "This article is about the grammar of the ".to_dyn(),
                Link {
                    location: StarLang.path().into(),
                    text: "classical dialect of the Star Language",
                }
                .to_dyn(),
                ". Besides some innovations and new phrases, the dialect's \
                 grammar was considerably stable."
                    .to_dyn(),
            ])]
            .to_dyn(),
            sections: vec![
                Section {
                    title: "Nouns".to_owned(),
                    id: Id::new("nouns").unwrap(),
                    body: vec![
                        Paragraph(
                            "Nouns in Classical Star Language are similar to \
                             nouns in English. They usually are the direct \
                             representative of things in the language, rather \
                             than referring to them in an indirect way like \
                             pronouns do. Another difference between them is \
                             that nouns can take adjectives, while pronouns \
                             don't. Nouns vary in case, gender and number.",
                        )
                        .to_dyn(),
                        Paragraph("The cases are:").to_dyn(),
                        UnorderedList(vec![
                            "Nominative",
                            "Accusative",
                            "Topical",
                            "Postpositional",
                        ])
                        .to_dyn(),
                        Paragraph(
                            "The nominative case is used when the noun is the \
                             subject (it agrees with the verb). The \
                             accusative is used when the noun is a (direct) \
                             object but without any preposition (it is an \
                             important argument to a verb, without \
                             agreement). Topical case is used when the noun \
                             is the topic of a clause, and not the \
                             \"comment\". Postpositional is used when a \
                             postposition follows the noun.",
                        )
                        .to_dyn(),
                        Paragraph("The genders are:").to_dyn(),
                        UnorderedList(vec!["Divine", "Animate", "Inanimate"])
                            .to_dyn(),
                        Paragraph(
                            "The divine gender usually refers to the gods or \
                             a manifestation of gods, such as elements from \
                             the nature. The animate gender is used normally \
                             referring to things having a non-divine spirit; \
                             something that is alive and mortal. Inanimate \
                             gender refers to things that are lifeless. It is \
                             worth noting the words don't always match their \
                             gender's description.",
                        )
                        .to_dyn(),
                        Paragraph("The numbers are:").to_dyn(),
                        UnorderedList(vec![
                            "Singular",
                            "Plural",
                            "Nullar",
                            "Collective",
                        ])
                        .to_dyn(),
                        Paragraph(
                            "The singular number is used when the noun refers \
                             to exactly one single thing. The plural number \
                             is used when the noun refers to an indefinite \
                             amount, but bigger than one. Nullar is used to \
                             refer to an amount of zero. Collective is used \
                             to refer to every possible entity of the noun; \
                             it is used when it refers to all of them.",
                        )
                        .to_dyn(),
                        Paragraph(
                            "Nouns are divided in inflection classes. Some \
                             inflection classes vary in all case, gender and \
                             number. Some varies only in case and gender, \
                             having a fixed number. Other varies only in case \
                             and number, having a fixed gender. There are \
                             also the ones that vary only in case, having \
                             fixed gender and number.",
                        )
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Adjectives".to_owned(),
                    id: Id::new("adjectives").unwrap(),
                    body: Paragraph(
                        "Adjectives in Classical Star Language are similar to \
                         adjectives in English. They modify nouns by giving \
                         more information about a noun's characteristics. \
                         They vary in case, gender and number, with the same \
                         types of case, gender and number as nouns. \
                         Adjectives must agree in case, gender and number \
                         with their nouns. Adjectives also have inflection \
                         classes, but they always vary in case, number and \
                         gender.",
                    )
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Pronouns".to_owned(),
                    id: Id::new("pronouns").unwrap(),
                    body: vec![
                        Paragraph(
                            "Pronouns refer to things indirectly. They \
                             usually carry a reference to something, but they \
                             might be expletive (carries no actual meaning). \
                             Pronouns vary in case, gender, number and \
                             person. Gender and number are the same as with \
                             nouns. Cases, besides the ones used with nouns, \
                             there is also the intransitive case. The \
                             intransitive case distinguishes itself from the \
                             nominative case such that:",
                        )
                        .to_dyn(),
                        OrderedList(vec![
                            "The verb takes no arguments except the subject;",
                            "The subject is the patient of the action, that \
                             is, it involuntarily suffers the action.",
                        ])
                        .to_dyn(),
                        Paragraph(
                            "There are three persons. Not only personal \
                             pronouns vary in person. They are:",
                        )
                        .to_dyn(),
                        UnorderedList(vec![
                            "First (1st)",
                            "Second (2nd)",
                            "Third (3rd)",
                        ])
                        .to_dyn(),
                        Paragraph(
                            "The first person refer to who is speaking. The \
                             second person refers to who is being spoken to. \
                             The third person refers to something that is \
                             neither of these.",
                        )
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
            ],
        }),
    );
}
