use crate::{
    component::{BlockComponent, Component, Context, InlineComponent},
    location::Location,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Bold<T, K>(pub T)
where
    T: Component<Kind = K>;

impl<T> Component for Bold<T, InlineComponent>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<b class=\"bold\">{}</b>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

impl<T> Component for Bold<T, BlockComponent>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<div class=\"bold\">{}</div>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Italic<T, K>(pub T)
where
    T: Component<Kind = K>;

impl<T> Component for Italic<T, InlineComponent>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<i class=\"italic\">{}</i>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

impl<T> Component for Italic<T, BlockComponent>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<div class=\"italic\">{}</div>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Preformatted<T, K>(pub T)
where
    T: Component<Kind = K>;

impl<T> Component for Preformatted<T, InlineComponent>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<pre class=\"pre\">{}</pre>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

impl<T> Component for Preformatted<T, BlockComponent>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<div class=\"pre\">{}</div>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Paragraph<T>(pub Vec<T>)
where
    T: Component<Kind = InlineComponent>;

impl<T> Component for Paragraph<T>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = T::Kind;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "<p class=\"paragraph\">{}</p>", ctx.renderer(&self.0))?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Link<T>
where
    T: Component<Kind = InlineComponent>,
{
    pub text: T,
    pub location: Location,
}

impl<T> Component for Link<T>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<a href=\"{}\", class=\"link\">{}</a>",
            ctx.renderer(&self.location),
            ctx.renderer(&self.text),
        )?;
        Ok(())
    }
}
