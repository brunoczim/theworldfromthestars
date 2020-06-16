use crate::{
    component::{page::Page, Context},
    location,
};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Site {
    pub pages: HashMap<location::Internal, Page>,
}

#[derive(Debug)]
pub struct Generation {
    pub site: Site,
    pub assets_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl Generation {
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
            fs::create_dir_all(&output_dir)?;

            for entry in fs::read_dir(&src_dir)? {
                let entry = entry?;
                let name = entry.file_name();

                if entry.file_type()?.is_dir() {
                    dirs.push(dir.join(name));
                } else {
                    let mut src_path = src_dir.clone();
                    src_path.push(&name);
                    let mut output_path = output_dir.clone();
                    output_path.push(&name);
                    fs::copy(src_path, output_path)?;
                }
            }
        }

        Ok(())
    }

    fn gen_pages(&self) -> anyhow::Result<()> {
        for (loc, page) in &self.site.pages {
            let path = self.output_dir.join(Path::new(loc.as_str()));
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut file = fs::File::create(path)?;
            write!(file, "{}", Context::new(loc).renderer(page))?;
        }

        Ok(())
    }
}
