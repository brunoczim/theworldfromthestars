use crate::{
    component::{BlockComponent, Component, Context, InlineComponent},
    location::Location,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Image {
    pub src: Location,
    pub alt: String,
}

impl Component for Image {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<img src=\"{}\" alt=\"{}\" class=\"image\">",
            ctx.renderer(&self.src),
            ctx.renderer(&self.alt),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Figure<L>
where
    L: Component<Kind = InlineComponent>,
{
    pub img: Image,
    pub legend: L,
}

impl<L> Component for Figure<L>
where
    L: Component<Kind = InlineComponent>,
{
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"fig-wrapper\">{}<div \
             class=\"fig-legend\">{}</div></div>",
            ctx.renderer(&self.img),
            ctx.renderer(&self.legend),
        )?;
        Ok(())
    }
}
