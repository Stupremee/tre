use crate::Spanned;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    String,
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
    Colon,
    Semicolon,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            TokenType::String => "<string>",
            TokenType::Integer => "<int>",
            TokenType::Bool => "<bool>",
            TokenType::Identifier => "<identifier>",
            TokenType::Def => "def",
            TokenType::Let => "let",
            TokenType::Loop => "loop",
            TokenType::While => "while",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Break => "break",
            TokenType::Continue => "continue",
            TokenType::Bang => "!",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Colon => ":",
            TokenType::Semicolon => ";",
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftCurly => "{",
            TokenType::RightCurly => "}",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::NotEqual => "!=",
            TokenType::GreaterEqual => ">=",
            TokenType::Greater => ">",
            TokenType::LessEqual => "<=",
            TokenType::Less => "<",
        };
        write!(f, "{}", val)
    }
}

pub type Token = Spanned<TokenType>;
