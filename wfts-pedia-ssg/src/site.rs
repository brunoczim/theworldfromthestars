use crate::{
    location::{Fragment, InternalPath},
    page::{Page, RenderPage},
};
use anyhow::Context as _;
use std::{
    collections::{hash_map, HashMap},
    fs,
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum Node<P = Page, D = Directory> {
    Page(P),
    Directory(D),
}

impl From<Page> for Node {
    fn from(page: Page) -> Self {
        Node::Page(page)
    }
}

impl From<Directory> for Node {
    fn from(dir: Directory) -> Self {
        Node::Directory(dir)
    }
}

impl<P, D> Node<P, D> {
    pub fn page(self) -> Option<P> {
        match self {
            Node::Page(file) => Some(file),
            Node::Directory(_) => None,
        }
    }

    pub fn dir(self) -> Option<D> {
        match self {
            Node::Page(_) => None,
            Node::Directory(dir) => Some(dir),
        }
    }

    pub fn as_ref(&self) -> Node<&P, &D> {
        match self {
            Node::Page(file) => Node::Page(file),
            Node::Directory(dir) => Node::Directory(dir),
        }
    }

    pub fn as_mut(&mut self) -> Node<&mut P, &mut D> {
        match self {
            Node::Page(file) => Node::Page(file),
            Node::Directory(dir) => Node::Directory(dir),
        }
    }
}

impl<P, D> Node<P, D>
where
    P: Into<Page>,
    D: Into<Directory>,
{
    pub fn normalize(self) -> Node {
        match self {
            Node::Page(page) => Node::Page(page.into()),
            Node::Directory(dir) => Node::Directory(dir.into()),
        }
    }
}

impl<P, D> Node<P, D>
where
    P: AsRef<Page>,
    D: AsRef<Directory>,
{
    pub fn normalize_cloned(&self) -> Node<Page, Directory> {
        match self {
            Node::Page(page) => Node::Page(page.as_ref().clone()),
            Node::Directory(dir) => Node::Directory(dir.as_ref().clone()),
        }
    }
}

impl AsRef<Directory> for Directory {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Directory> for Directory {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[derive(Debug, Clone, Default)]
pub struct Directory {
    pub contents: HashMap<Fragment, Node>,
}

impl Directory {
    pub fn get(&self, path: InternalPath) -> Option<Node<&Page, &Directory>> {
        let mut dir = self;

        let mut last = None;

        for piece in &path.fragments {
            if let Some(last) = last {
                dir = dir.contents.get(last)?.as_ref().dir()?;
            }
            last = Some(piece);
        }

        let node = dir.contents.get(last?)?;
        Some(node.as_ref())
    }

    pub fn get_mut(
        &mut self,
        path: InternalPath,
    ) -> Option<Node<&mut Page, &mut Directory>> {
        let mut dir = self;

        let mut last = None;

        for piece in &path.fragments {
            if let Some(last) = last {
                dir = dir.contents.get_mut(last)?.as_mut().dir()?;
            }
            last = Some(piece);
        }

        let node = dir.contents.get_mut(last?)?;
        Some(node.as_mut())
    }

    pub fn insert(&mut self, path: InternalPath, node: Node) {
        let mut dir = self;

        let mut last = None::<&Fragment>;

        for piece in &path.fragments {
            if let Some(last) = last {
                dir = dir
                    .contents
                    .entry(last.clone())
                    .or_insert_with(|| Node::Directory(Directory::default()))
                    .as_mut()
                    .dir()
                    .expect("Must be a directory");
            }
            last = Some(piece);
        }

        let stem = last.expect("Cannot insert at root");
        match dir.contents.entry(stem.clone()) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(node);
            },
            _ => panic!("Cannot insert if already occupied"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Site {
    pub root: Directory,
}

impl<'dir> IntoIterator for &'dir Directory {
    type Item = (InternalPath, &'dir Page);
    type IntoIter = Pages<'dir>;

    fn into_iter(self) -> Self::IntoIter {
        Pages {
            curr_loc: InternalPath::root(),
            curr_iter: self.contents.iter(),
            directories: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pages<'dir> {
    curr_loc: InternalPath,
    curr_iter: hash_map::Iter<'dir, Fragment, Node>,
    directories: Vec<(InternalPath, &'dir Directory)>,
}

impl<'site> Iterator for Pages<'site> {
    type Item = (InternalPath, &'site Page);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some((suffix, node)) = self.curr_iter.next() {
                let mut loc = self.curr_loc.clone();
                loc.fragments.push(suffix.clone());
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
pub struct Generator {
    pub site: Site,
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl Generator {
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
            let res = write!(
                file,
                "{}",
                RenderPage { page, location: &loc, site: &self.site }
            );
            res.with_context(|| format!("Generating page {}", path.display()))?;
        }

        Ok(())
    }
}
