use crate::phonology::Morpheme;
use std::fmt;
use wfts_pedia_ssg::component::{
    list::UnorderedList,
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

#[derive(Debug, Clone)]
pub struct Pronunciation(pub Morpheme);

impl Component for Pronunciation {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        let mut list = vec![format!("Phonemic: /{}/", self.0.to_broad_ipa())];
        if let Morpheme::Word(word) = &self.0 {
            list.push(format!(
                "Early CSL Accents: [{}]",
                word.to_early_narrow_ipa()
            ));
            list.push(format!(
                "Some Late CSL Accents: [{}]",
                word.to_late_narrow_ipa()
            ));
        }
        write!(fmt, "{}", ctx.renderer(UnorderedList(list)))
    }
}
