use crate::component::{Component, Context, InlineComponent};
use percent_encoding::{percent_encode, CONTROLS};
use std::{fmt, str};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("Invalid internal location piece {piece:?}")]
pub struct InvalidPiece {
    pub piece: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    URL(Url),
    Internal(Internal),
}

impl Location {
    pub fn internal<S>(raw: S) -> Self
    where
        S: AsRef<str>,
    {
        Location::Internal(Internal::new(raw).unwrap())
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
pub struct Internal {
    contents: String,
}

impl Default for Internal {
    fn default() -> Self {
        Self::root()
    }
}

impl Internal {
    pub fn root() -> Self {
        Self { contents: String::new() }
    }

    pub fn new<S>(raw: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let mut this = Self::root();
        for piece in raw.as_ref().split('/') {
            if piece.len() != 0 {
                this.push(piece)?;
            }
        }
        Ok(this)
    }

    pub fn is_root(&self) -> bool {
        self.contents.len() == 0
    }

    pub fn as_str(&self) -> &str {
        &self.contents
    }

    pub fn dir_depth(&self) -> usize {
        self.contents.as_bytes().iter().filter(|&&ch| ch == b'/').count()
    }

    pub fn push<S>(&mut self, piece: S) -> anyhow::Result<()>
    where
        S: AsRef<str>,
    {
        let piece = piece.as_ref();

        if piece == "" || piece == "." || piece == ".." {
            Err(InvalidPiece { piece: piece.to_owned() })?;
        }
        if self.contents.len() != 0 {
            self.contents.push('/');
        }
        self.contents.push_str(piece);

        Ok(())
    }

    pub fn pop(&mut self) -> bool {
        if let Some(pos) =
            self.contents.as_bytes().iter().rposition(|&ch| ch == b'/')
        {
            self.contents.truncate(pos);
            true
        } else {
            false
        }
    }

    pub fn suffix(&self, mut depth: usize) -> Option<Self> {
        let pos = self.contents.as_bytes().iter().position(|&ch| {
            if depth == 0 {
                true
            } else {
                if ch == b'/' {
                    depth -= 1;
                }
                false
            }
        })?;

        Some(Self { contents: String::from(&self.contents[pos ..]) })
    }

    pub fn append<I>(&mut self, other: I)
    where
        I: AsRef<Self>,
    {
        if other.as_ref().contents.len() != 0 {
            self.contents.push('/');
            self.contents.push_str(&other.as_ref().contents);
        }
    }

    pub fn pieces(&self) -> Pieces {
        Pieces { inner: self.contents.split('/') }
    }

    pub fn str_pieces(&self) -> StrPieces {
        StrPieces { inner: self.contents.split('/') }
    }
}

impl Component for Internal {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        for _ in 0 .. ctx.location().dir_depth() {
            fmt.write_str("../")?;
        }
        let encoded = percent_encode(self.contents.as_bytes(), CONTROLS)
            .collect::<String>();
        write!(fmt, "{}", ctx.renderer(&encoded))?;
        Ok(())
    }
}

impl fmt::Display for Internal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(&self.contents)
    }
}

#[derive(Debug, Clone)]
pub struct Pieces<'internal> {
    inner: str::Split<'internal, char>,
}

impl<'internal> Iterator for Pieces<'internal> {
    type Item = Internal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|s| Internal { contents: s.to_owned() })
    }
}

impl<'internal> DoubleEndedIterator for Pieces<'internal> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|s| Internal { contents: s.to_owned() })
    }
}

#[derive(Debug, Clone)]
pub struct StrPieces<'internal> {
    inner: str::Split<'internal, char>,
}

impl<'internal> Iterator for StrPieces<'internal> {
    type Item = &'internal str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'internal> DoubleEndedIterator for StrPieces<'internal> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl AsRef<Internal> for Internal {
    fn as_ref(&self) -> &Internal {
        self
    }
}
