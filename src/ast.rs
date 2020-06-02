use crate::parser::token::Token;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'src> {
    Int(i32),
    Float(f32),
    String(&'src str),
}

impl<'src> fmt::Display for Literal<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(val) => write!(f, "({})", val),
            Literal::Float(val) => write!(f, "({})", val),
            Literal::String(val) => write!(f, "\"{}\"", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'src> {
    Binary(Box<Expr<'src>>, Token<'src>, Box<Expr<'src>>),
    Unary(Token<'src>, Box<Expr<'src>>),
    Literal(Literal<'src>),
}

impl<'src> fmt::Display for Expr<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Expr::Unary(op, expr) => write!(f, "({}{})", op, expr),
            Expr::Literal(val) => write!(f, "({})", val),
        }
    }
}
