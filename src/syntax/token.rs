use codespan::Span;
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    String,
    Float,
    Integer,
    Bool,

    // Identifier
    Identifier,

    // Keywords
    Def,
    Let,
    Loop,
    While,
    For,
    If,
    Else,
    Or,
    And,
    Break,

    // Some other chars
    Bang,
    Plus,
    Minus,
    Star,
    Slash,
    StarStar,
    Colon,
    Comma,
    Dot,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Equal,
    EqualEqual,
    NotEqual,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
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
