use crate::{
    component::{Component, Context, InlineComponent},
    location::Location,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Audio(pub Location);

impl Component for Audio {
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<audio controls src={}>No browser support for audio.</audio>",
            ctx.renderer(&self.0)
        )
    }
}
