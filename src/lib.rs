#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]

pub mod syntax;

use codespan::{ByteIndex, FileId};
use codespan_reporting::diagnostic::Diagnostic;
use std::{
    fmt,
    ops::{Deref, DerefMut, Range},
};

pub type Result<T> = std::result::Result<T, Diagnostic<FileId>>;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
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

    #[inline]
    pub fn start(self) -> ByteIndex {
        self.0.start()
    }

    #[inline]
    pub fn end(self) -> ByteIndex {
        self.0.end()
    }
}

impl Into<codespan::Span> for Span {
    fn into(self) -> codespan::Span {
        self.0
    }
}

impl Into<Range<usize>> for Span {
    fn into(self) -> Range<usize> {
        Range::from(self.0)
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Spanned<T>(T, Span);

impl<T> Spanned<T> {
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    #[inline]
    pub fn data(&self) -> &T {
        &self.0
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

impl<T> fmt::Display for Spanned<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // probably we should print the position somehow
        write!(f, "{}", self.0)
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

pub mod diagnostic {
    pub use codespan::FileId;
    pub use codespan_reporting::diagnostic::{LabelStyle, Severity};

    pub type Files<'s> = codespan::Files<&'s str>;
    pub type Diagnostic = codespan_reporting::diagnostic::Diagnostic<codespan::FileId>;
    pub type Label = codespan_reporting::diagnostic::Label<codespan::FileId>;

    pub fn emit(files: &Files<'_>, diagnostic: &Diagnostic) {
        use codespan_reporting::term::{self, termcolor};

        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        let config = term::Config::default();
        term::emit(&mut stdout, &config, files, diagnostic).expect("failed to emit diagnostics");
    }
}
