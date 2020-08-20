use crate::{
    grammar::noun::class1,
    phonology::{Parse, Word},
};
use wfts_lang::semantics::Meaning;
use wfts_pedia_ssg::{
    component::Component,
    location::{Id, InternalPath},
    page::Page,
    site::{Directory, Node},
};

pub fn make(dir: &mut Directory) {
    let word = class1::Definition {
        id: Id::new("eye").unwrap(),
        meanings: vec![Meaning::Eye],
        notes: "".blocking().to_dyn(),
        word: class1::Word::new(Word::parse_str("gas").unwrap()).unwrap(),
    };

    for (word, section) in word.to_dict_entry().sections() {
        let path = InternalPath::parse(format!(
            "dictionary/{}/index.html",
            word.to_text()
        ))
        .unwrap();
        dir.insert(
            path,
            Node::Page({
                Page {
                    title: word.to_text(),
                    body: "".blocking().to_dyn(),
                    sections: vec![section],
                }
            }),
        );
    }
}
