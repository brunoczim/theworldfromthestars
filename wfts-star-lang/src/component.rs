use std::fmt;
use wfts_pedia_ssg::component::{
    text::Bold,
    BlockComponent,
    Component,
    Context,
    InlineComponent,
};

#[derive(Debug, Clone, Copy)]
pub struct WithStarAlphabet<T, K>(pub T)
where
    T: Component<Kind = K>;

impl<T> Component for WithStarAlphabet<T, InlineComponent>
where
    T: Component<Kind = InlineComponent>,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<span class=\"star-alphabet\">{}</span>",
            ctx.renderer(&self.0)
        )
    }
}

impl<T> Component for WithStarAlphabet<T, BlockComponent>
where
    T: Component<Kind = BlockComponent>,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"star-alphabet\">{}</div>",
            ctx.renderer(&self.0)
        )
    }
}

#[derive(Debug, Clone)]
pub struct DefinitionHead {
    pub name: String,
    pub inflected_for: Vec<String>,
}

impl Component for DefinitionHead {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        let inflected_for = self.inflected_for.join(", ");
        write!(
            fmt,
            "<div class=\"definition-head\">{} (inflected for {})</div>",
            ctx.renderer(Bold(WithStarAlphabet(&self.name))),
            ctx.renderer(inflected_for)
        )
    }
}
