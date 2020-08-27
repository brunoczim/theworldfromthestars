use crate::{
    grammar::{adjective, noun, pronoun},
    StarLang,
};
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        list::{OrderedList, UnorderedList},
        text::{Link, Paragraph},
        Component,
    },
    location::{Id, InternalPath},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    dir.insert(
        InternalPath::parse("grammar/index.html").unwrap(),
        Node::Page(Page {
            title: "Classical Star Language Grammar".to_owned(),
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
                    title: "Nouns".to_dyn(),
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
                    children: vec![Section {
                        title: "Inflection Classes".to_dyn(),
                        id: Id::new("noun-classes").unwrap(),
                        body: vec![
                            noun::full1::Word::affix_table(),
                            noun::full2::Word::affix_table(),
                            noun::divine1::Word::affix_table(),
                            noun::divine2::Word::affix_table(),
                        ]
                        .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Adjectives".to_dyn(),
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
                    children: vec![Section {
                        title: "Inflection Classes".to_dyn(),
                        id: Id::new("adjective-classes").unwrap(),
                        body: vec![adjective::regular::Word::affix_table()]
                            .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Pronouns".to_dyn(),
                    id: Id::new("pronouns").unwrap(),
                    body: vec![
                        Paragraph(
                            "Pronouns refer to things indirectly. They \
                             usually carry a reference to something, but they \
                             might be expletive (carries no actual meaning). \
                             Pronouns vary in case, gender, number and \
                             person. Gender and number are the same as with \
                             nouns. Cases, besides the ones used with nouns, \
                             there is also the passive case. The passive case \
                             distinguishes itself from the nominative case \
                             such that:",
                        )
                        .to_dyn(),
                        OrderedList(vec![
                            "The verb takes no arguments except the subject;",
                            "The subject is the passive of the action, that \
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
                    children: vec![Section {
                        title: "Inflection Classes".to_dyn(),
                        id: Id::new("pronoun-classes").unwrap(),
                        body: vec![
                            pronoun::demonstrative::Word::affix_table(),
                            pronoun::personal::Word::affix_table(),
                        ]
                        .to_dyn(),
                        children: vec![],
                    }],
                },
                Section {
                    title: "Postpositions".to_dyn(),
                    id: Id::new("postpositions").unwrap(),
                    body: Paragraph(
                        "Postpositions usually give information about a kind \
                         of motion, a manner of location or a manner of time. \
                         Postpositions are similar to English's prepositions, \
                         except they are words written after the noun phrases \
                         (hence post); the noun phrases are inflected in the \
                         postpositional cases then. Postpositions inflect for \
                         the case which the noun would take if it weren't \
                         followed by the postposition. Besides the same cases \
                         as nouns, postpositions also inflect for the passive \
                         case, like pronouns.",
                    )
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Conjunctions".to_dyn(),
                    id: Id::new("conjunctions").unwrap(),
                    body: Paragraph(
                        "Conjunctions are used to connect phrases or even \
                         whole clauses, like in English. In Star Language, \
                         however, they may also be used to nominalize verbs. \
                         The requirement for conjunctions is also more \
                         extreme in Star Language: they are required in order \
                         to create a noun compound. In the case of connecting \
                         noun phrases, the noun phrases are inflected with \
                         the noun phrase's case normally. Conjunctions are \
                         inflected for case. Besides the same cases as nouns, \
                         and also the passive case, they have a default case \
                         called coordenative, used when connecting two \
                         clauses.",
                    )
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Verbs".to_dyn(),
                    id: Id::new("verbs").unwrap(),
                    body: vec![
                        Paragraph(
                            "Verbs in Star Language are similar to verbs in \
                             English. They play a central role in a clause, \
                             and usually specifies the main action of the \
                             clause, but not necessarily. Verbs inflect for \
                             person, mood and tense. The persons are the same \
                             as with the pronouns.",
                        )
                        .to_dyn(),
                        Paragraph("Moods are:").to_dyn(),
                        UnorderedList(vec!["Indicative", "Imperative"])
                            .to_dyn(),
                        Paragraph(
                            "Imperative mood gives orders. Indicative is used \
                             for anything else.",
                        )
                        .to_dyn(),
                        Paragraph("Indicative tenses are:").to_dyn(),
                        UnorderedList(vec![
                            "Present",
                            "Past",
                            "Near-Future",
                            "Far-Future",
                        ])
                        .to_dyn(),
                        Paragraph(
                            "The name of the tenses pretty much explain \
                             themselves. Past is something that has already \
                             occured. Present is occuring right now. \
                             Near-Future will occur soon. Far-Future will \
                             occur in a long time. Difference betweeen the \
                             two futures are relative.",
                        )
                        .to_dyn(),
                        Paragraph("Imperative tenses are:").to_dyn(),
                        UnorderedList(vec!["Present", "Future"]).to_dyn(),
                        Paragraph(
                            "Present means an order to be executed now. \
                             Future is an order meant to be executed in a \
                             somewhat far future.",
                        )
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Adverb".to_dyn(),
                    id: Id::new("adverbs").unwrap(),
                    body: vec![
                        Paragraph(
                            "Adverbs are words that modify verbs. They work \
                             like adverbs in English. Adverbs inflect for \
                             mood.",
                        )
                        .to_dyn(),
                        Paragraph("Moods are:").to_dyn(),
                        UnorderedList(vec![
                            "Indicative",
                            "Subjunctive",
                            "Interrogative",
                            "Optative",
                            "Imperative",
                        ])
                        .to_dyn(),
                        Paragraph(
                            "Every adverb mood relates to a verb mood, and \
                             they must agree. Indicative agrees to the verb's \
                             indicative and express certainty. Subjunctive \
                             agrees to the verb's indicative, and express \
                             doubt or hypothesis. Interrogative also agrees \
                             to the verb's indicativee, and is used to ask a \
                             question. Optative agrees to the verb's \
                             imperative, and is used to express a wish or a \
                             hope. Imperative agrees to the verb's imperative \
                             and express orders.",
                        )
                        .to_dyn(),
                    ]
                    .to_dyn(),
                    children: vec![],
                },
                Section {
                    title: "Roots".to_dyn(),
                    id: Id::new("roots").unwrap(),
                    body: Paragraph(
                        "Roots are templates used to create words. They might \
                         combine with another root or create words directly. \
                         Roots may yield words of any part of speech.",
                    )
                    .to_dyn(),
                    children: vec![],
                },
            ],
        }),
    );
}
