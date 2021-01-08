use std::{fmt, hash::Hash, str};
use thiserror::Error;
use wfts_pedia_ssg::{
    component::{Component, Context, InlineComponent},
    location::{Fragment, Id},
};

#[derive(Debug, Clone, Error)]
#[error("Invalid language code")]
pub struct InvalidCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LangCode {
    lang: [u8; LangCode::LANG_LEN],
    variety: [u8; LangCode::VARIETY_LEN],
}

impl LangCode {
    pub const LANG_LEN: usize = 3;
    pub const VARIETY_LEN: usize = 3;

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

    pub fn lang(&self) -> &str {
        str::from_utf8(&self.lang).unwrap()
    }

    pub fn variety(&self) -> &str {
        str::from_utf8(&self.variety).unwrap()
    }

    pub fn to_fragment(&self) -> Fragment {
        Fragment::new(format!("{}", self)).unwrap()
    }

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
