use crate::{
    component::{page::Page, Context},
    location,
};
use anyhow::Context as _;
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
        for (loc, page) in &self.site.pages {
            let path = self.output_dir.join(Path::new(loc.as_str()));
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!("Creating dir {}", parent.display())
                })?;
            }

            let mut file = fs::File::create(&path).with_context(|| {
                format!("Creating page file {}", path.display())
            })?;
            write!(file, "{}", Context::new(loc).renderer(page)).with_context(
                || format!("Generating page {}", path.display()),
            )?;
        }

        Ok(())
    }
}
