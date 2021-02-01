//! This crate specifies means of constructing languages, as well the
//! constructed languages.

pub mod proto_divine;
pub mod phonetics;

use std::{fmt, hash::Hash, str};
use thiserror::Error;
use wfts_pedia_ssg::{
    component::{Component, Context, InlineComponent},
    location::{Fragment, Id, InternalPath},
    site::{Directory, Site},
};

/// Error issued when an invalid language code was attempted to be parsed.
#[derive(Debug, Clone, Error)]
#[error("Invalid language code")]
pub struct InvalidCode;

/// Code of a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LangCode {
    lang: [u8; LangCode::LANG_LEN],
    variety: [u8; LangCode::VARIETY_LEN],
}

impl LangCode {
    /// Length of the language piece of the code.
    pub const LANG_LEN: usize = 3;
    /// Length of the variety piece of the code.
    pub const VARIETY_LEN: usize = 3;

    /// Tries to parse a language code. Format is XYZ-ABC, where XYZ is the
    /// language piece, and ABC is the variety piece. Only alphanumeric
    /// characters and '_' are allowed for XYZ and ABC. X must be alphabetic.
    pub fn parse<S>(string: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let string = string.as_ref();
        let bytes = string.as_bytes();

        if string.len() != Self::LANG_LEN + Self::VARIETY_LEN + 1 {
            Err(InvalidCode)?;
        }
        if bytes[Self::LANG_LEN] != b'-' {
            Err(InvalidCode)?;
        }

        let mut lang = [0; Self::LANG_LEN];
        let mut variety = [0; Self::VARIETY_LEN];

        for (i, &ch) in bytes[.. Self::LANG_LEN].iter().enumerate() {
            lang[i] = ch;
        }

        for (i, &ch) in bytes[Self::LANG_LEN + 1 ..].iter().enumerate() {
            variety[i] = ch;
        }

        Self::new(lang, variety)
    }

    /// Creates a new language code from its pieces. Only alphanumeric (or '_')
    /// characters are alowed for both lang and variety. First character of
    /// language must be alphabetic.
    pub fn new(
        lang: [u8; Self::LANG_LEN],
        variety: [u8; Self::VARIETY_LEN],
    ) -> anyhow::Result<Self> {
        if !lang[0].is_ascii_alphabetic() {
            Err(InvalidCode)?;
        }
        for &ch in lang.iter().chain(variety.iter()) {
            if !ch.is_ascii_alphanumeric() && ch != b'_' {
                Err(InvalidCode)?;
            }
        }

        Ok(Self { lang, variety })
    }

    /// The language piece of the language code as a raw array of bytes.
    pub fn lang_raw(&self) -> [u8; Self::LANG_LEN] {
        self.lang
    }

    /// The variety piece of the language code as a raw array of bytes.
    pub fn variety_raw(&self) -> [u8; Self::VARIETY_LEN] {
        self.variety
    }

    /// The language piece of the language code as a string.
    pub fn lang(&self) -> &str {
        str::from_utf8(&self.lang).unwrap()
    }

    /// The variety piece of the language code.
    pub fn variety(&self) -> &str {
        str::from_utf8(&self.variety).unwrap()
    }

    /// Converts this language code into the fragment of a path.
    pub fn to_fragment(&self) -> Fragment {
        Fragment::new(format!("{}", self)).unwrap()
    }

    /// Converts this language code into an ID of a location.
    pub fn to_id(&self) -> Id {
        Id::new(format!("{}", self)).unwrap()
    }
}

impl fmt::Display for LangCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for &ch in &self.lang {
            write!(fmt, "{}", ch as char)?;
        }
        write!(fmt, "-")?;
        for &ch in &self.variety {
            write!(fmt, "{}", ch as char)?;
        }
        Ok(())
    }
}

impl Component for LangCode {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, _ctx: Context) -> fmt::Result {
        write!(fmt, "{}", self)
    }
}

/// This trait specifies the behaviour of languages related to the encyclopedia
/// site.
pub trait Lang: Sized {
    /// Returns the code of this language.
    fn code(&self) -> LangCode;

    /// Returns the directory containing the subsite related to this language,
    /// the subsite of the encyclopedia.
    fn subsite(&self) -> Directory;

    /// Returns the path to this language in the encyclopedia site.
    fn path(&self) -> InternalPath {
        InternalPath::parse(format!("langs/{}", self.code())).unwrap()
    }

    /// Adds this language's subsite to the encyclopedia site.
    fn add_to_site(&self, site: &mut Site) {
        site.root.insert(self.path(), self.subsite().into());
    }
}
