use std::{collections::HashMap, path::PathBuf, process};
use wfts_lang::Lang;
use wfts_pedia_ssg::{
    component::{
        list::UnorderedList,
        text::{Link, Paragraph},
        Component,
    },
    location::{Fragment, Id},
    page::{Page, Section},
    site::{Directory, Generator, Node, Site},
};
use wfts_star_lang::StarLang;

fn main() {
    let mut site = Site { root: Directory { contents: HashMap::new() } };

    site.root
        .contents
        .insert(Fragment::new("index.html").unwrap(), Node::Page(index_page()));

    let generator = Generator {
        site,
        assets_dir: PathBuf::from("wfts-pedia/assets/"),
        output_dir: PathBuf::from("wfts-pedia/site/"),
    };

    if let Err(err) = generator.gen() {
        eprintln!("{}: {}", err, err.root_cause());
        process::exit(-1);
    }
}

fn index_page() -> Page {
    let body = vec![Paragraph(vec![String::from(
        "This is the front page of the encyclopedia of \"The World From The \
         Stars\"",
    )])
    .to_dyn()];

    Page {
        title: String::from("The World From The Stars Encyclopedia"),
        body: body.to_dyn(),
        sections: vec![Section {
            title: String::from("List Of Languages"),
            body: UnorderedList(vec![Link {
                location: StarLang.path().into(),
                text: "Star Language",
            }])
            .to_dyn(),
            id: Id::new("list-of-langs").unwrap(),
            children: Vec::new(),
        }],
    }
}
