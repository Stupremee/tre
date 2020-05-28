use codespan::Span;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Error,

    Integer,

    Identifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'repr> {
    pub ty: TokenType,
    pub repr: &'repr str,
    pub span: Span,
}

impl<'repr> Token<'repr> {
    pub fn new(ty: TokenType, repr: &'repr str, span: Range<usize>) -> Self {
        Self {
            ty,
            repr,
            span: Span::new(span.start as u32, span.end as u32),
        }
    }
}
