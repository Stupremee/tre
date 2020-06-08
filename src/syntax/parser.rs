use super::{
    lexer::{Lexer, TokenStream},
    token::{Token, TokenType},
};
use crate::{
    diagnostic::{Diagnostic, Label},
    Result, Span, Spanned,
};

use codespan::{FileId, Files};
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<'input> {
    file: FileId,
    tokens: Peekable<TokenStream<'input>>,
}

impl<'input> Parser<'input> {
    pub fn new(files: &Files<&'input str>, file_id: FileId) -> Result<Self> {
        let mut tokens = Lexer::new(files.source(file_id)).into_iter().peekable();
        if tokens.peek().is_none() {
            let label =
                Label::primary(file_id, Span::default()).with_message("Can't parse an emtpy file.");
            return Err(Diagnostic::error().with_labels(vec![label]));
        }

        let parser = Self {
            file: file_id,
            tokens,
        };
        Ok(parser)
    }
}
