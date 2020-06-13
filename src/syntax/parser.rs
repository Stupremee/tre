use super::{
    ast,
    lexer::{Lexer, TokenStream},
    token::{Token, TokenType},
};
use crate::{
    diagnostic::{Diagnostic, Files, Label},
    Span,
};

use codespan::FileId;
use std::iter::Peekable;

pub type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug)]
pub enum SyntaxError {}

#[derive(Debug)]
pub struct Parser<'input> {
    file: FileId,
    tokens: Peekable<TokenStream<'input>>,
    span: Span,
}

impl<'input> Parser<'input> {
    pub fn new(files: &Files<'input>, file_id: FileId) -> Self {
        let tokens = Lexer::new(files.source(file_id)).into_iter().peekable();
        Self {
            file: file_id,
            tokens,
            span: Span::default(),
        }
    }

    fn next_expression(&mut self) -> Result<ast::Expr> {
        todo!()
    }
}
