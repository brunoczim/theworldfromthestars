use crate::{
    fmt::WriteOrthography,
    proto_divine::{dictionary::Dictionary, phonology::Word, ProtoDivine},
    Lang,
};
use wfts_pedia_ssg::{
    component::{list::UnorderedList, text::Link, Component},
    location::{Id, InternalPath, Location},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    let dict = Dictionary::with_all_entries();
    let mut words = dict.sections.keys().cloned().collect::<Vec<_>>();
    words.sort();
    make_index(dir, words);
    make_words(dir, dict);
}

fn make_index(dir: &mut Directory, words: Vec<Word>) {
    let list = words
        .into_iter()
        .map(|word| Link {
            location: Location::internal(format!(
                "{}/dictionary/{}",
                ProtoDivine.path(),
                word.orthography_ref()
            )),
            text: word.clone().orthography(),
        })
        .collect();
    dir.insert(
        InternalPath::parse("dictionary/index.html").unwrap(),
        Node::Page(Page {
            title: "Proto-Divine Language Dictionary".to_owned(),
            body: "This page is a list of reconstructed Proto-Divine words."
                .blocking()
                .to_dyn(),
            sections: vec![Section {
                title: "List of Words".to_dyn(),
                id: Id::new("list-of-words").unwrap(),
                body: UnorderedList(list).to_dyn(),
                children: vec![],
            }],
        }),
    );
}

fn make_words(dir: &mut Directory, dict: Dictionary) {
    for (word, sections) in dict.sections {
        let path = InternalPath::parse(format!(
            "dictionary/{}/index.html",
            word.orthography_ref(),
        ))
        .unwrap();
        dir.insert(
            path,
            Node::Page({
                Page {
                    title: format!(
                        "{} — Proto Divine Language Dictionary",
                        word.orthography_ref()
                    ),
                    body: "".blocking().to_dyn(),
                    sections,
                }
            }),
        );
    }
}
