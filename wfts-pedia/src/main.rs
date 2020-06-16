use std::{collections::HashMap, path::PathBuf, process};
use wfts_pedia_ssg::{
    component::{
        page::{Page, Section},
        text::Paragraph,
    },
    location,
    site::{Generator, Site},
};

fn main() {
    let mut site = Site { pages: HashMap::new() };

    site.pages.insert(
        location::Internal::new("index.html").unwrap(),
        Page {
            top_section: Section {
                title: String::from("The World From The Stars Encylopedia"),
                body: Box::new(Vec::<Paragraph<String>>::new()),
            },
        },
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
