pub mod text;
pub mod img;
pub mod page;
pub mod table;
pub mod list;

use crate::location;
use std::{borrow::Cow, fmt, rc::Rc, sync::Arc};

fn html_escape(ch: char) -> Option<&'static str> {
    match ch {
        '&' => Some("&amp;"),
        '<' => Some("&lt;"),
        '>' => Some("&gt;"),
        '"' => Some("&quot;"),
        '\'' => Some("&#36;"),
        '\\' => Some("&#92;"),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlineComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Context {
    section_level: u32,
    dir_depth: u32,
}

impl Context {
    pub fn new(location: &location::Internal) -> Self {
        let depth = location.as_str().chars().filter(|&ch| ch == '/').count();
        Self { section_level: 0, dir_depth: depth as u32 }
    }

    pub fn section_level(self) -> u32 {
        self.section_level
    }

    pub fn heading_level(self) -> &'static str {
        match self.section_level() {
            0 => "h1",
            1 => "h2",
            2 => "h3",
            3 => "h4",
            4 => "h5",
            _ => "h6",
        }
    }

    pub fn dir_depth(self) -> u32 {
        self.dir_depth
    }

    pub fn step_level(self) -> Self {
        Self { section_level: self.section_level.saturating_add(1), ..self }
    }
}

impl Context {
    pub fn renderer<T>(self, component: T) -> Renderer<T>
    where
        T: Component,
    {
        Renderer { component, context: self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Renderer<T>
where
    T: Component,
{
    pub component: T,
    pub context: Context,
}

impl<T> fmt::Display for Renderer<T>
where
    T: Component,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.component.to_html(fmt, self.context)
    }
}

pub trait Component: fmt::Debug {
    type Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result;
}

impl<'this, T> Component for &'this T
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Component for Box<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Component for Rc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Component for Arc<T>
where
    T: Component + ?Sized,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<'cow, T> Component for Cow<'cow, T>
where
    T: Component + ToOwned + ?Sized,
    T::Owned: fmt::Debug,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}

impl<T> Component for Vec<T>
where
    T: Component,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        for elem in self {
            elem.to_html(fmt, ctx)?;
        }
        Ok(())
    }
}

impl<T> Component for Option<T>
where
    T: Component,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        if let Some(component) = self {
            component.to_html(fmt, ctx)?;
        }
        Ok(())
    }
}

impl Component for str {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, _ctx: Context) -> fmt::Result {
        let mut start = 0;
        let iter = self
            .char_indices()
            .filter_map(|(i, ch)| html_escape(ch).map(|s| (i, s)));

        for (end, escape) in iter {
            fmt.write_str(&self[start .. end])?;
            fmt.write_str(escape)?;
            start = end + 1;
        }

        fmt.write_str(&self[start ..])?;
        Ok(())
    }
}

impl Component for String {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        (**self).to_html(fmt, ctx)
    }
}
