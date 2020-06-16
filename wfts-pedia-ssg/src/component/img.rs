use crate::{
    component::{BlockComponent, Component, Context},
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
            "<img src=\"{}\" alt=\"{}\", class=\"image\">",
            ctx.renderer(&self.src),
            ctx.renderer(&self.alt),
        )?;
        Ok(())
    }
}
