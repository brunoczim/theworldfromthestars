use crate::{
    component::{page::Page, Context, DynComponent},
    location::InternalPath,
};
use anyhow::Context as _;
use std::{
    collections::{hash_map, HashMap},
    fs,
    io::Write,
    path::PathBuf,
    sync::Arc,
};

#[derive(Debug, Clone)]
pub enum Node<'page> {
    Page(Page<Arc<DynComponent<'page>>>),
    Directory(Directory<'page>),
}

impl<'page> Node<'page> {
    pub fn page(self) -> Option<Page<Arc<DynComponent<'page>>>> {
        match self {
            Node::Page(file) => Some(file),
            Node::Directory(_) => None,
        }
    }

    pub fn page_ref(&self) -> Option<&Page<Arc<DynComponent<'page>>>> {
        match self {
            Node::Page(file) => Some(file),
            Node::Directory(_) => None,
        }
    }

    pub fn page_mut(&mut self) -> Option<&mut Page<Arc<DynComponent<'page>>>> {
        match self {
            Node::Page(file) => Some(file),
            Node::Directory(_) => None,
        }
    }

    pub fn dir(self) -> Option<Directory<'page>> {
        match self {
            Node::Page(_) => None,
            Node::Directory(dir) => Some(dir),
        }
    }

    pub fn dir_ref(&self) -> Option<&Directory<'page>> {
        match self {
            Node::Page(_) => None,
            Node::Directory(dir) => Some(dir),
        }
    }

    pub fn dir_mut(&mut self) -> Option<&mut Directory<'page>> {
        match self {
            Node::Page(_) => None,
            Node::Directory(dir) => Some(dir),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Directory<'page> {
    pub contents: HashMap<InternalPath, Node<'page>>,
}

#[derive(Debug, Clone)]
pub struct Site<'page> {
    pub root: Directory<'page>,
}

impl<'page, 'site> IntoIterator for &'site Directory<'page> {
    type Item = (InternalPath, &'site Page<Arc<DynComponent<'page>>>);
    type IntoIter = Pages<'page, 'site>;

    fn into_iter(self) -> Self::IntoIter {
        Pages {
            curr_loc: InternalPath::root(),
            curr_iter: self.contents.iter(),
            directories: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pages<'page, 'site> {
    curr_loc: InternalPath,
    curr_iter: hash_map::Iter<'site, InternalPath, Node<'page>>,
    directories: Vec<(InternalPath, &'site Directory<'page>)>,
}

impl<'page, 'site> Iterator for Pages<'page, 'site> {
    type Item = (InternalPath, &'site Page<Arc<DynComponent<'page>>>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((suffix, node)) = self.curr_iter.next() {
                let mut loc = self.curr_loc.clone();
                loc.fragments.extend(suffix.fragments.iter().cloned());
                match node {
                    Node::Page(page) => break Some((loc, page)),
                    Node::Directory(dir) => self.directories.push((loc, dir)),
                }
            } else {
                let (loc, dir) = self.directories.pop()?;
                self.curr_iter = dir.contents.iter();
                self.curr_loc = loc;
            }
        }
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
        for (loc, page) in &self.site.root {
            let path = self.output_dir.join(loc.to_fs_path());
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Creating dir {}", parent.display())
                })?;
            }

            let mut file = fs::File::create(&path).with_context(|| {
                format!("Creating page file {}", path.display())
            })?;
            write!(file, "{}", Context::new(&loc, &self.site).renderer(&page))
                .with_context(|| {
                    format!("Generating page {}", path.display())
                })?;
        }

        Ok(())
    }
}
