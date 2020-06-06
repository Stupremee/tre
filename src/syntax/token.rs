use crate::Spanned;

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
    If,
    Else,
    Break,
    Continue,

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
    LeftCurly,
    RightCurly,

    Equal,
    EqualEqual,
    NotEqual,
    GreaterEqual,
    Greater,
    LessEqual,
    Less,
}

pub type Token = Spanned<TokenType>;
