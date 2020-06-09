use super::{
    ast,
    lexer::{Lexer, TokenStream},
    token::{Token, TokenType},
};
use crate::{
    diagnostic::{Diagnostic, Files, Label},
    Result, Span,
};

use codespan::FileId;
use std::iter::Peekable;

fn new_error(msg: impl Into<String>, primary_label: Label) -> Diagnostic {
    Diagnostic::error()
        .with_message(msg)
        .with_labels(vec![primary_label])
}

#[derive(Debug)]
pub struct Parser<'input> {
    file: FileId,
    tokens: Peekable<TokenStream<'input>>,
    span: Span,
}

impl<'input> Parser<'input> {
    pub fn new(files: &Files<'input>, file_id: FileId) -> Result<Self> {
        let mut tokens = Lexer::new(files.source(file_id)).into_iter().peekable();
        if tokens.peek().is_none() {
            let label =
                Label::primary(file_id, Span::default()).with_message("Can't parse an emtpy file.");
            return Err(Diagnostic::error().with_labels(vec![label]));
        }

        let parser = Self {
            file: file_id,
            tokens,
            span: Span::default(),
        };
        Ok(parser)
    }

    fn next(&mut self) -> Result<Token> {
        match self.tokens.next() {
            Some(token) => {
                self.span = token.span();
                Ok(token)
            }
            None => Err(new_error(
                "unexpected end of input",
                Label::primary(self.file, self.span),
            )),
        }
    }

    fn peek(&mut self) -> Result<&Token> {
        match self.tokens.peek() {
            Some(token) => {
                self.span = token.span();
                Ok(token)
            }
            None => Err(new_error(
                "unexpected end of input",
                Label::primary(self.file, self.span),
            )),
        }
    }

    fn next_is(&mut self, ty: TokenType) -> bool {
        self.peek().map_or(false, |t| t.data() == &ty)
    }

    fn eat(&mut self, ty: TokenType) -> Result<Token> {
        let actual = self.peek()?.clone();
        match actual {
            ref token if token.data() == &ty => self.next(),
            ref token => Err(new_error(
                "unexpected token",
                Label::primary(self.file, self.span).with_message(format!(
                    "expected '{:?}' but got '{:?}'",
                    ty,
                    token.data()
                )),
            )),
        }
    }

    fn binding_power(&self, t: TokenType, prefix: bool) -> Option<(u8, u8)> {
        let power = match t {
            TokenType::Identifier | TokenType::Integer => (99u8, 100u8),
            TokenType::LeftParen => (99, 0),
            TokenType::Plus | TokenType::Minus if prefix => (99, 9),
            TokenType::Plus | TokenType::Minus => (5, 6),
            TokenType::Star | TokenType::Slash => (7, 8),
            TokenType::StarStar => (9, 10),
            TokenType::Bang => (11, 100),
            _ => return None,
        };
        Some(power)
    }

    fn next_expr(&mut self, op: Option<ast::BinaryOperation>) -> Result<ast::Expr> {
        let mut primary = self.next_primary_expr()?;
        todo!()
    }

    fn next_primary_expr(&mut self) -> Result<ast::Expr> {
        match self.peek() {}
    }

    pub fn next_item(&mut self) -> Result<Token> {
        if self.next_is(TokenType::Def) {
            self.next()
        } else {
            Err(new_error(
                "unexpected token",
                Label::primary(self.file, self.span).with_message(format!("expected 'def'",)),
            ))
        }
    }
}
