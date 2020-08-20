use crate::component::{Component, Context, InlineComponent};
use percent_encoding::{percent_encode, CONTROLS};
use std::{fmt, path::PathBuf, str};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    URL(Url),
    Internal(InternalLoc),
}

impl From<InternalPath> for Location {
    fn from(path: InternalPath) -> Self {
        Location::Internal(InternalLoc::from(path))
    }
}

impl From<InternalLoc> for Location {
    fn from(loc: InternalLoc) -> Self {
        Location::Internal(loc)
    }
}

impl From<Url> for Location {
    fn from(url: Url) -> Self {
        Location::URL(url)
    }
}

impl Location {
    pub fn internal<S>(contents: S) -> Self
    where
        S: AsRef<str>,
    {
        Location::Internal(InternalLoc::parse(contents).unwrap())
    }
}

impl Component for Location {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        match self {
            Location::URL(url) => write!(fmt, "{}", url),
            Location::Internal(int) => int.to_html(fmt, ctx),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalPath {
    pub fragments: Vec<Fragment>,
}

impl InternalPath {
    pub fn parse<S>(string: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref();
        let mut this = Self { fragments: Vec::new() };

        if string.len() > 0 {
            for fragment in string.split('/') {
                this.fragments.push(Fragment::new(fragment)?);
            }
        }

        Ok(this)
    }

    pub fn root() -> Self {
        Self { fragments: Vec::new() }
    }

    pub fn is_root(&self) -> bool {
        self.fragments.len() == 0
    }

    pub fn dir_depth(&self) -> usize {
        self.fragments.len().saturating_sub(1)
    }

    pub fn to_fs_path(&self) -> PathBuf {
        PathBuf::from(format!("{}", self))
    }

    pub fn append(mut self, fragment: Fragment) -> Self {
        self.fragments.push(fragment);
        self
    }
}

impl Default for InternalPath {
    fn default() -> Self {
        Self::root()
    }
}

impl fmt::Display for InternalPath {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;

        for fragment in &self.fragments {
            if first {
                first = false;
            } else {
                fmt.write_str("/")?;
            }
            write!(fmt, "{}", fragment)?;
        }

        Ok(())
    }
}

impl Component for InternalPath {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        for _ in 0 .. ctx.location().dir_depth() {
            fmt.write_str("../")?;
        }
        let encoded = percent_encode(self.to_string().as_bytes(), CONTROLS)
            .collect::<String>();
        write!(fmt, "{}", ctx.renderer(&encoded))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct InternalLoc {
    pub path: InternalPath,
    pub id: Option<Id>,
}

impl From<InternalPath> for InternalLoc {
    fn from(path: InternalPath) -> Self {
        Self { path, id: None }
    }
}

impl InternalLoc {
    pub fn parse<S>(string: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref();
        let hash = string
            .as_bytes()
            .iter()
            .rposition(|&ch| ch == b'#')
            .unwrap_or(string.len());

        Ok(Self {
            path: InternalPath::parse(&string[.. hash])?,
            id: if hash == string.len() {
                None
            } else {
                Some(Id::new(&string[hash + 1 ..])?)
            },
        })
    }
}

impl fmt::Display for InternalLoc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", &self.path)?;

        if let Some(id) = &self.id {
            write!(fmt, "#{}", id)?;
        }

        Ok(())
    }
}

impl Component for InternalLoc {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        for _ in 0 .. ctx.location().dir_depth() {
            fmt.write_str("../")?;
        }
        let encoded = percent_encode(self.to_string().as_bytes(), CONTROLS)
            .collect::<String>();
        write!(fmt, "{}", ctx.renderer(&encoded))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid ID string")]
pub struct InvalidId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id {
    contents: Box<str>,
}

impl Id {
    pub fn new<S>(contents: S) -> anyhow::Result<Self>
    where
        S: AsRef<str> + Into<Box<str>>,
    {
        let mut iter = contents.as_ref().as_bytes().iter();

        iter.next().filter(|ch| ch.is_ascii_alphabetic()).ok_or(InvalidId)?;

        for &ch in iter {
            if !ch.is_ascii_alphanumeric() && ch != b'_' && ch != b'-' {
                Err(InvalidId)?;
            }
        }

        Ok(Self { contents: contents.into() })
    }

    pub fn as_str(&self) -> &str {
        &self.contents
    }
}

impl fmt::Display for Id {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_str())
    }
}

impl Component for Id {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, _ctx: Context) -> fmt::Result {
        write!(fmt, "{}", self)
    }
}

#[derive(Debug, Clone, Error)]
#[error("Invalid location fragment string")]
pub struct InvalidFragment;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fragment {
    contents: Box<str>,
}

impl Fragment {
    pub fn new<S>(contents: S) -> anyhow::Result<Self>
    where
        S: AsRef<str> + Into<Box<str>>,
    {
        if let "" | "." | ".." = contents.as_ref() {
            Err(InvalidFragment)?;
        }

        for ch in contents.as_ref().bytes() {
            if let b'/' | b'#' = ch {
                Err(InvalidFragment)?;
            }
        }

        Ok(Self { contents: contents.into() })
    }

    pub fn as_str(&self) -> &str {
        &self.contents
    }
}

impl fmt::Display for Fragment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.as_str())
    }
}
