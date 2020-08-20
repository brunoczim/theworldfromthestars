use crate::component::{BlockComponent, Component, Context, InlineComponent};
use std::fmt;

pub type Entries<E> = Vec<Vec<Entry<E>>>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Table<T, E>
where
    T: Component<Kind = InlineComponent>,
    E: Component,
{
    pub title: T,
    pub entries: Entries<E>,
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
            "<div class=\"table-wrapper\"><span \
             class=\"table-title\">{title}</span><table class=\"table\">",
            title = ctx.renderer(&self.title),
        )?;

        for row in &self.entries {
            write!(fmt, "<tr>")?;
            for entry in row {
                write!(fmt, "<{}", entry.tag())?;
                if entry.rowspan != 1 {
                    write!(fmt, " rowspan=\"{}\"", entry.rowspan)?;
                }
                if entry.colspan != 1 {
                    write!(fmt, " colspan=\"{}\"", entry.colspan)?;
                }
                write!(
                    fmt,
                    ">{}</{}>",
                    ctx.renderer(&entry.data),
                    entry.tag()
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

    fn tag(&self) -> &'static str {
        match self.header {
            true => "th",
            false => "td",
        }
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
