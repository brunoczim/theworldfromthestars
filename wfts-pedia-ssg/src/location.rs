use crate::component::{Component, Context, InlineComponent};
use percent_encoding::{percent_encode, CONTROLS};
use std::fmt;
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

impl Internal {
    pub fn new<S>(raw: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let mut contents = String::new();
        let mut first = true;
        for piece in raw.as_ref().split('/') {
            if piece == "." || piece == ".." {
                Err(InvalidPiece { piece: piece.to_owned() })?;
            }
            if piece.len() != 0 {
                if first {
                    first = false;
                } else {
                    contents.push('/');
                }
                contents.push_str(piece);
            }
        }
        Ok(Self { contents })
    }

    pub fn as_str(&self) -> &str {
        &self.contents
    }
}

impl Component for Internal {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        for _ in 0 .. ctx.dir_depth() {
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
