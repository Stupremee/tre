use super::{
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
        let token = self.tokens.next().ok_or_else(|| {
            new_error(
                "unexpected end of input",
                Label::primary(self.file, self.span),
            )
        });
        self.span = token.as_ref().map(|t| t.span()).unwrap_or(self.span);
        token
    }

    fn peek(&mut self) -> Option<&Token> {
        let token = self.tokens.peek();
        // just to be sure set the span here
        self.span = token.map(|t| t.span()).unwrap_or(self.span);
        token
    }

    fn eat(&mut self, ty: TokenType) -> Result<Token> {
        let actual = self.peek().cloned();
        match actual {
            Some(ref token) if token.data() == &ty => self.next(),
            Some(ref token) => Err(new_error(
                "unexpected token",
                Label::primary(self.file, self.span).with_message(format!(
                    "expected '{:?}' but got '{:?}'",
                    ty,
                    token.data()
                )),
            )),
            None => Err(new_error(
                "unexpected end of input",
                Label::primary(self.file, self.span),
            )),
        }
    }

    pub fn next_item(&mut self) -> Result<Token> {
        self.eat(TokenType::Def)
    }
}
