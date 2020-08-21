use crate::morphology::Morpheme;
use std::fmt;
use wfts_pedia_ssg::component::{
    list::UnorderedList,
    text::{Bold, Italic},
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
        write!(
            fmt,
            "<div class=\"definition-head\">{} (inflected for ",
            ctx.renderer(Bold(WithStarAlphabet(&self.name))),
        )?;

        let mut first = true;
        for key in &self.inflected_for {
            if first {
                first = false;
            } else {
                write!(fmt, ",")?;
            }
            write!(fmt, "{}", ctx.renderer(Bold(key)))?;
        }

        write!(fmt, ")</div>")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct PronunciationKey {
    name: String,
    pronunciation: String,
}

impl Component for PronunciationKey {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"pronunciation-name\">{}</div>:<div \
             class=\"pronunciation-val\">{}</div>",
            ctx.renderer(Italic(&self.name)),
            ctx.renderer(&self.pronunciation)
        )
    }
}

#[derive(Debug, Clone)]
pub struct Pronunciation(pub Morpheme);

impl Component for Pronunciation {
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        let mut list = vec![PronunciationKey {
            name: "Phonemic".to_owned(),
            pronunciation: format!("/{}/", self.0.to_broad_ipa()),
        }];
        if let Morpheme::Word(word) = &self.0 {
            list.push(PronunciationKey {
                name: "Early CSL Accents".to_owned(),
                pronunciation: format!("[{}]", word.to_early_narrow_ipa()),
            });
            list.push(PronunciationKey {
                name: "Some Late CSL Accents".to_owned(),
                pronunciation: format!("[{}]", word.to_late_narrow_ipa()),
            });
        }
        write!(fmt, "{}", ctx.renderer(UnorderedList(list)))
    }
}
