use crate::phonetics::Variation;
use std::{fmt, rc::Rc, sync::Arc};
use wfts_pedia_ssg::component::{Component, Context, InlineComponent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Reconstructed<T>(pub T);

impl<T> fmt::Display for Reconstructed<T>
where
    T: fmt::Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "*{}", self.0)
    }
}

impl<T> Component for Reconstructed<T>
where
    T: Component,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, ctx: Context) -> fmt::Result {
        write!(fmt, "*{}", ctx.renderer(&self.0))
    }
}

pub trait WriteOrthography {
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result;

    fn orthography_ref(&self) -> Orthography<&Self> {
        self.orthography()
    }

    fn orthography(self) -> Orthography<Self>
    where
        Self: Sized,
    {
        Orthography(self)
    }
}

impl<'this, T> WriteOrthography for &'this T
where
    T: WriteOrthography + ?Sized,
{
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).orthography_fmt(fmt)
    }
}

impl<T> WriteOrthography for Box<T>
where
    T: WriteOrthography + ?Sized,
{
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).orthography_fmt(fmt)
    }
}

impl<T> WriteOrthography for Rc<T>
where
    T: WriteOrthography + ?Sized,
{
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).orthography_fmt(fmt)
    }
}

impl<T> WriteOrthography for Arc<T>
where
    T: WriteOrthography + ?Sized,
{
    fn orthography_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).orthography_fmt(fmt)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Orthography<T>(pub T)
where
    T: WriteOrthography;

impl<T> fmt::Display for Orthography<T>
where
    T: WriteOrthography,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.orthography_fmt(fmt)
    }
}

impl<T> Component for Orthography<T>
where
    T: WriteOrthography + fmt::Debug,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, _ctx: Context) -> fmt::Result {
        self.0.orthography_fmt(fmt)
    }
}

pub trait WriteBroadPronunc {
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result;

    fn broad_pronunc_ref(&self) -> BroadPronunc<&Self> {
        self.broad_pronunc()
    }

    fn broad_pronunc(self) -> BroadPronunc<Self>
    where
        Self: Sized,
    {
        BroadPronunc(self)
    }
}

impl<'this, T> WriteBroadPronunc for &'this T
where
    T: WriteBroadPronunc + ?Sized,
{
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).broad_pronunc_fmt(fmt)
    }
}

impl<T> WriteBroadPronunc for Box<T>
where
    T: WriteBroadPronunc + ?Sized,
{
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).broad_pronunc_fmt(fmt)
    }
}

impl<T> WriteBroadPronunc for Rc<T>
where
    T: WriteBroadPronunc + ?Sized,
{
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).broad_pronunc_fmt(fmt)
    }
}

impl<T> WriteBroadPronunc for Arc<T>
where
    T: WriteBroadPronunc + ?Sized,
{
    fn broad_pronunc_fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        (**self).broad_pronunc_fmt(fmt)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BroadPronunc<T>(pub T)
where
    T: WriteBroadPronunc;

impl<T> fmt::Display for BroadPronunc<T>
where
    T: WriteBroadPronunc,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.broad_pronunc_fmt(fmt)
    }
}

impl<T> Component for BroadPronunc<T>
where
    T: WriteBroadPronunc + fmt::Debug,
{
    type Kind = InlineComponent;

    fn to_html(&self, fmt: &mut fmt::Formatter, _ctx: Context) -> fmt::Result {
        self.0.broad_pronunc_fmt(fmt)
    }
}

pub trait NarrowPronunc {
    fn narrow_pronunc(&self) -> Variation;
}

impl<'this, T> NarrowPronunc for &'this T
where
    T: NarrowPronunc + ?Sized,
{
    fn narrow_pronunc(&self) -> Variation {
        (**self).narrow_pronunc()
    }
}

impl<T> NarrowPronunc for Box<T>
where
    T: NarrowPronunc + ?Sized,
{
    fn narrow_pronunc(&self) -> Variation {
        (**self).narrow_pronunc()
    }
}

impl<T> NarrowPronunc for Rc<T>
where
    T: NarrowPronunc + ?Sized,
{
    fn narrow_pronunc(&self) -> Variation {
        (**self).narrow_pronunc()
    }
}

impl<T> NarrowPronunc for Arc<T>
where
    T: NarrowPronunc + ?Sized,
{
    fn narrow_pronunc(&self) -> Variation {
        (**self).narrow_pronunc()
    }
}
