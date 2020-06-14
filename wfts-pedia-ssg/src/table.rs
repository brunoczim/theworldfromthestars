use crate::component::{BlockComponent, Component, Context, InlineComponent};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Table<T, E>
where
    T: Component<Kind = InlineComponent>,
    E: Component,
{
    pub title: T,
    pub entries: Vec<Vec<Entry<E>>>,
}

impl<T, E> Component for Table<T, E>
where
    T: Component<Kind = InlineComponent>,
    E: Component,
{
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"table-wrapper\"><{tag} \
             class=\"table-title\">{title}</{tag}><table class=\"table\">",
            tag = ctx.heading_level(),
            title = ctx.renderer(&self.title),
        )?;

        for row in &self.entries {
            write!(fmt, "<tr>")?;
            for entry in row {
                write!(fmt, "<{}", if entry.header { "th" } else { "td" })?;
                if entry.rowspan != 1 {
                    write!(fmt, " rowspan=\"{}\"", entry.rowspan)?;
                }
                if entry.colspan != 1 {
                    write!(fmt, " colspan=\"{}\"", entry.colspan)?;
                }
                write!(
                    fmt,
                    ">{}</{}>",
                    ctx.step_level().renderer(&entry.data),
                    if entry.header { "th" } else { "td" }
                )?;
            }
            write!(fmt, "</tr>")?;
        }

        write!(fmt, "</table></div>")?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entry<T>
where
    T: Component,
{
    pub rowspan: u32,
    pub colspan: u32,
    pub header: bool,
    pub data: T,
}

impl<T> Entry<T>
where
    T: Component,
{
    pub fn new(data: T) -> Self {
        Self { data, rowspan: 1, colspan: 1, header: false }
    }
}

impl<T> Default for Entry<T>
where
    T: Component + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}
