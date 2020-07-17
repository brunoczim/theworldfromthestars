use crate::{
    component::{Context, DynComponent},
    location::{Id, InternalLoc, InternalPath},
    site::Site,
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Section {
    pub title: String,
    pub body: DynComponent,
    pub id: Id,
    pub children: Vec<Section>,
}

#[derive(Debug, Clone, Copy)]
struct RenderSection<'section, 'loc, 'site> {
    section: &'section Section,
    level: u32,
    ctx: Context<'loc, 'site>,
}

impl<'section, 'loc, 'site> fmt::Display
    for RenderSection<'section, 'loc, 'site>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "<div id={id} class=\"section section-{level}\"><{title_tag} \
             class=\"header\"><a class=\"header-link\" \
             href=\"{link}\">{title}</a></{title_tag}><div \
             class=\"section-body\">{body}",
            level = self.level,
            title_tag = heading_level(self.level),
            title = self.ctx.renderer(&self.section.title),
            body = self.ctx.renderer(&self.section.body),
            id = self.ctx.renderer(&self.section.id),
            link = self.ctx.renderer(InternalLoc {
                path: self.ctx.location().clone(),
                id: Some(self.section.id.clone())
            })
        )?;

        for section in &self.section.children {
            write!(
                fmt,
                "{}",
                RenderSection { level: self.level + 1, section, ..*self }
            )?;
        }

        write!(fmt, "</div></div>")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub body: DynComponent,
    pub sections: Vec<Section>,
}

impl AsRef<Page> for Page {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<Page> for Page {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RenderPage<'page, 'loc, 'site> {
    pub page: &'page Page,
    pub location: &'loc InternalPath,
    pub site: &'site Site,
}

impl<'page, 'loc, 'site> fmt::Display for RenderPage<'page, 'loc, 'site> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let ctx = Context::new(self.location, self.site);
        write!(
            fmt,
            "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><meta \
             name=\"viewport\" content=\"width=device-width, \
             initial-scale=1.0\"><link rel=\"stylesheet\" type=\"text/css\" \
             href=\"{css}\"><title>{title}</title><body><div \
             id=\"page-top\"><h1>{title}</h1><div id=\"body-wrapper\">{body}",
            css = ctx.renderer(InternalPath::parse("css/main.css").unwrap()),
            title = ctx.renderer(&self.page.title),
            body = ctx.renderer(&self.page.body)
        )?;

        for section in &self.page.sections {
            write!(fmt, "{}", RenderSection { level: 1, ctx, section })?;
        }

        write!(fmt, "</div></div></body></html>")?;
        Ok(())
    }
}

fn heading_level(section_level: u32) -> &'static str {
    match section_level {
        0 => "h1",
        1 => "h2",
        2 => "h3",
        3 => "h4",
        4 => "h5",
        _ => "h6",
    }
}
