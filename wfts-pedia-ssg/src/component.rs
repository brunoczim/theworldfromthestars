pub mod text;
pub mod img;
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
        '\'' => Some("&#39;"),
        '\\' => Some("&#92;"),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockComponent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlineComponent;

#[derive(Debug, Clone, Copy)]
pub struct Context<'loc, 'site> {
    location: &'loc InternalPath,
    site: &'site Site,
}

impl<'loc, 'site> Context<'loc, 'site> {
    pub(crate) fn new(location: &'loc InternalPath, site: &'site Site) -> Self {
        Self { location, site }
    }

    pub fn location(self) -> &'loc InternalPath {
        self.location
    }

    pub fn subpages(self) -> &'loc InternalPath {
        self.location
    }

    pub fn renderer<T>(self, component: T) -> Renderer<'loc, 'site, T>
    where
        T: Component,
    {
        Renderer { component, context: self }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Renderer<'loc, 'site, T>
where
    T: Component,
{
    pub component: T,
    pub context: Context<'loc, 'site>,
}

impl<'loc, 'site, T> fmt::Display for Renderer<'loc, 'site, T>
where
    T: Component,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.component.to_html(fmt, self.context)
    }
}

pub type DynComponent<Kind = BlockComponent> =
    Arc<dyn Component<Kind = Kind> + Send + Sync>;

pub trait Component: fmt::Debug {
    type Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result;

    fn blocking(self) -> Blocking<Self>
    where
        Self: Sized,
    {
        Blocking(self)
    }

    fn to_dyn(self) -> DynComponent<Self::Kind>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Arc::new(self)
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
