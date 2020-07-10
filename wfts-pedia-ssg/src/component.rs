pub mod text;
pub mod img;
pub mod page;
pub mod table;
pub mod list;

use crate::{location::InternalPath, site::Site};
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

#[derive(Debug, Clone, Copy)]
pub struct Context<'loc, 'pages, 'site> {
    location: &'loc InternalPath,
    site: &'site Site<'pages>,
    section_level: u32,
}

impl<'loc, 'pages, 'site> Context<'loc, 'pages, 'site> {
    pub fn new(
        location: &'loc InternalPath,
        site: &'site Site<'pages>,
    ) -> Self {
        Self { location, site, section_level: 0 }
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

    pub fn location(self) -> &'loc InternalPath {
        self.location
    }

    pub fn subpages(self) -> &'loc InternalPath {
        self.location
    }

    pub fn step_level(self) -> Self {
        Self { section_level: self.section_level.saturating_add(1), ..self }
    }

    pub fn renderer<T>(self, component: T) -> Renderer<'loc, 'pages, 'site, T>
    where
        T: Component,
    {
        Renderer { component, context: self }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Renderer<'loc, 'pages, 'site, T>
where
    T: Component,
{
    pub component: T,
    pub context: Context<'loc, 'pages, 'site>,
}

impl<'loc, 'pages, 'site, T> fmt::Display for Renderer<'loc, 'pages, 'site, T>
where
    T: Component,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.component.to_html(fmt, self.context)
    }
}

pub type DynComponent<'obj> =
    dyn Component<Kind = BlockComponent> + Send + Sync + 'obj;

pub trait Component: fmt::Debug {
    type Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result;

    fn to_dyn<'obj>(self) -> Arc<DynComponent<'obj>>
    where
        Self: Sized + Send + Sync + 'obj,
    {
        Arc::new(Blocking(self))
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Blocking<T>(pub T)
where
    T: Component;

impl<T> Component for Blocking<T>
where
    T: Component,
{
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        self.0.to_html(fmt, ctx)
    }
}
