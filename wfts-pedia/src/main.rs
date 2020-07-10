use std::{collections::HashMap, path::PathBuf, process, sync::Arc};
use wfts_pedia_ssg::{
    component::{
        list::UnorderedList,
        page::{Page, Section},
        text::{Link, Paragraph},
        Component,
        DynComponent,
    },
    location::{InternalPath, Location},
    site::{Directory, Generator, Node, Site},
};

fn main() {
    let mut site = Site { root: Directory { contents: HashMap::new() } };

    site.root.contents.insert(
        InternalPath::parse("index.html").unwrap(),
        Node::Page(index_page()),
    );

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

fn index_page() -> Page<Arc<DynComponent<'static>>> {
    let body = vec![
        Paragraph(vec![String::from(
            "This is the front page of the encyclopedia of \"The World From \
             The Stars\"",
        )])
        .to_dyn(),
        Section {
            title: String::from("List Of Languages"),
            body: UnorderedList(vec![Link {
                location: Location::internal("langs/star"),
                text: "Star Language",
            }]),
        }
        .to_dyn(),
    ];
    Page {
        top_section: Section {
            title: String::from("The World From The Stars Encyclopedia"),
            body: body.to_dyn(),
        },
    }
}
