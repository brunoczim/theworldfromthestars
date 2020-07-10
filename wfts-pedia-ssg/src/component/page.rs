use crate::{
    component::{BlockComponent, Component, Context},
    location::InternalPath,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Section<T>
where
    T: Component,
{
    pub title: String,
    pub body: T,
}

impl<T> Component for Section<T>
where
    T: Component,
{
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<div class=\"section section-{section_level}\"><{title_tag} \
             class=\"title \
             title-{section_level}\">{title}</{title_tag}>{body}</div>",
            section_level = ctx.section_level(),
            title = ctx.renderer(&self.title),
            body = ctx.step_level().renderer(&self.body),
            title_tag = ctx.heading_level(),
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Page<T>
where
    T: Component,
{
    pub top_section: Section<T>,
}

impl<T> Component for Page<T>
where
    T: Component,
{
    type Kind = BlockComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(
            fmt,
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><meta \
             name=\"viewport\" content=\"width=device-width, \
             initial-scale=1.0\"><link rel=\"stylesheet\" type=\"text/css\" \
             href=\"{}\"><title>{}</title><body><div \
             id=\"page-wrapper\">{}</div></body></html>",
            ctx.renderer(InternalPath::parse("css/main.css").unwrap()),
            ctx.renderer(&self.top_section.title),
            ctx.renderer(&self.top_section)
        )?;
        Ok(())
    }
}
