#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

pub mod syntax;

use codespan::ByteIndex;
use std::ops::{Deref, DerefMut, Range};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Span(codespan::Span);

impl Span {
    #[inline]
    pub fn new(lo: impl Into<ByteIndex>, hi: impl Into<ByteIndex>) -> Self {
        Self(codespan::Span::new(lo, hi))
    }

    #[inline]
    pub fn span<T>(self, val: T) -> Spanned<T> {
        Spanned(val, self)
    }

    #[inline]
    pub fn merge(self, other: Self) -> Self {
        self.0.merge(other.0).into()
    }

    #[inline]
    pub fn index<'s>(&self, slice: &'s str) -> &'s str {
        &slice[Range::<usize>::from(self.0)]
    }
}

impl Into<codespan::Span> for Span {
    fn into(self) -> codespan::Span {
        self.0
    }
}

impl From<codespan::Span> for Span {
    fn from(span: codespan::Span) -> Span {
        Span(span)
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Span {
        let range = range.start as u32..range.end as u32;
        Span(range.into())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Spanned<T>(T, Span);

impl<T> Spanned<T> {
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    #[inline]
    pub fn span(&self) -> Span {
        self.1
    }

    #[inline]
    pub fn span_ref(&self) -> &Span {
        &self.1
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Spanned<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
