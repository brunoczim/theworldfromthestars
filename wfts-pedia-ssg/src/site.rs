use crate::{
    component::{page::Page, Context, DynComponent},
    location,
};
use anyhow::Context as _;
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct Site<'page> {
    pub page: Page<Arc<DynComponent<'page>>>,
    pub subsites: HashMap<location::Internal, Site<'page>>,
}

impl<'page> Site<'page> {
    pub fn access(
        &self,
        location: location::Internal,
    ) -> Option<Page<Arc<DynComponent<'page>>>> {
        let mut site = self;
        for piece in location.pieces() {
            site = site.subsites.get(&piece)?;
        }
        Some(site.page.clone())
    }
}

impl<'page, 'site> IntoIterator for &'site Site<'page> {
    type Item = (location::Internal, &'site Site<'page>);
    type IntoIter = Iter<'page, 'site>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { subsites: vec![(location::Internal::root(), self)] }
    }
}

#[derive(Debug, Clone)]
pub struct Iter<'page, 'site> {
    subsites: Vec<(location::Internal, &'site Site<'page>)>,
}

impl<'page, 'site> Iterator for Iter<'page, 'site> {
    type Item = (location::Internal, &'site Site<'page>);

    fn next(&mut self) -> Option<Self::Item> {
        self.subsites.pop().map(|(loc, site)| {
            for (piece, subsite) in site.subsites.iter() {
                let mut loc = loc.clone();
                loc.append(piece.clone());
                self.subsites.push((loc, subsite));
            }
            (loc, site)
        })
    }
}

#[derive(Debug, Clone)]
pub struct Generator<'pages> {
    pub site: Site<'pages>,
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl<'pages> Generator<'pages> {
    pub fn gen(&self) -> anyhow::Result<()> {
        if self.assets_dir != self.output_dir {
            self.copy_assets()?;
        }
        self.gen_pages()?;

        Ok(())
    }

    fn copy_assets(&self) -> anyhow::Result<()> {
        let mut dirs = vec![PathBuf::new()];

        while let Some(dir) = dirs.pop() {
            let src_dir = self.assets_dir.join(&dir);
            let output_dir = self.output_dir.join(&dir);
            fs::create_dir_all(&output_dir).with_context(|| {
                format!("Creating dir{}", output_dir.display().to_string())
            })?;

            let iter = fs::read_dir(&src_dir).with_context(|| {
                format!("Opening dir {}", src_dir.display())
            })?;
            for entry in iter {
                let entry = entry.with_context(|| {
                    format!("Reading dir {}", src_dir.display())
                })?;
                let name = entry.file_name();

                let typ = entry.file_type().with_context(|| {
                    format!("Reading file type of {}", entry.path().display())
                })?;
                if typ.is_dir() {
                    dirs.push(dir.join(name));
                } else {
                    let mut src_path = src_dir.clone();
                    src_path.push(&name);
                    let mut output_path = output_dir.clone();
                    output_path.push(&name);
                    fs::copy(&src_path, &output_path).with_context(|| {
                        format!(
                            "Copying {} to {}",
                            src_path.display(),
                            output_path.display()
                        )
                    })?;
                }
            }
        }

        Ok(())
    }

    fn gen_pages(&self) -> anyhow::Result<()> {
        for (mut loc, subsite) in &self.site {
            if loc.is_root() {
                loc = location::Internal::new("index.html").unwrap();
            }
            let path = self.output_dir.join(Path::new(loc.as_str()));
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Creating dir {}", parent.display())
                })?;
            }

            let mut file = fs::File::create(&path).with_context(|| {
                format!("Creating page file {}", path.display())
            })?;
            write!(
                file,
                "{}",
                Context::new(&loc, subsite, &self.site).renderer(&subsite.page)
            )
            .with_context(|| format!("Generating page {}", path.display()))?;
        }

        Ok(())
    }
}
