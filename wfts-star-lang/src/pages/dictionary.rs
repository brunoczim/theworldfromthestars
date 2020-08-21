use crate::{dictionary::Dictionary, morphology::Morpheme, StarLang};
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{list::UnorderedList, text::Link, Component},
    location::{Id, InternalPath, Location},
    page::{Page, Section},
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    let dict = Dictionary::with_all_entries();
    let mut morphemes = dict.sections.keys().cloned().collect::<Vec<_>>();
    morphemes.sort();
    make_index(dir, morphemes);
    make_words(dir, dict);
}

fn make_index(dir: &mut Directory, morphemes: Vec<Morpheme>) {
    let list = morphemes
        .into_iter()
        .map(|morpheme| Link {
            location: Location::internal(format!(
                "{}/dictionary/{}",
                StarLang.path(),
                morpheme.to_text()
            )),
            text: morpheme.to_text(),
        })
        .collect();
    dir.insert(
        InternalPath::parse("dictionary/index.html").unwrap(),
        Node::Page(Page {
            title: "Classical Star Language Dictionary".to_owned(),
            body: "This page is a list of Classical Star Language words."
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
    for (morpheme, sections) in dict.sections {
        let path = InternalPath::parse(format!(
            "dictionary/{}/index.html",
            morpheme.to_text(),
        ))
        .unwrap();
        dir.insert(
            path,
            Node::Page({
                Page {
                    title: format!(
                        "{} — Classical Star Language Dictionary",
                        morpheme
                    ),
                    body: "".blocking().to_dyn(),
                    sections,
                }
            }),
        );
    }
}
