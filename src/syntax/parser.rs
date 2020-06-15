use super::{
    ast,
    lexer::{Lexer, TokenStream},
    token::{Token, TokenType},
};
use crate::{
    diagnostic::{Diagnostic, Files, Label},
    Result, Span, Spanned,
};

use codespan::FileId;
use std::iter::Peekable;

#[macro_export]
macro_rules! binary_op {
    ($name:ident, $next:ident, $($ty:path),*) => {
        fn $name(&mut self) -> Result<ast::Expr> {
            let mut expr = self.$next()?;

            while self.next_one_of([
                $($ty),*
            ]) {
                let op = self.next().unwrap().data().into();
                let right = self.$next()?;
                let span = expr.span_ref().merge(right.span());
                expr = span.span(ast::ExprKind::Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                })
            }

            Ok(expr)
        }
    };
}

#[derive(Debug)]
pub enum SyntaxError {
    Expected {
        expected: TokenType,
        found: TokenType,
    },
    ExpectedOneOf {
        expected: Vec<TokenType>,
        found: TokenType,
    },
    ExpectedExpr,
    InvalidInteger(lexical::Error),
    UnexpectedEof(TokenType),
}

#[derive(Debug)]
pub struct Parser<'input> {
    file: FileId,
    files: &'input Files<'input>,
    tokens: Peekable<TokenStream<'input>>,
    span: Span,
}

impl<'input> Parser<'input> {
    pub fn new(files: &'input Files<'input>, file_id: FileId) -> Self {
        let tokens = Lexer::new(files.source(file_id)).into_iter().peekable();
        Self {
            file: file_id,
            tokens,
            span: Span::default(),
            files,
        }
    }

    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.next();
        self.span = token.as_ref().map_or(self.span, |token| token.span());
        token
    }

    fn peek(&mut self) -> Option<&Token> {
        let token = self.tokens.peek();
        self.span = token.map_or(self.span, |token| token.span());
        token
    }

    fn eat(&mut self, ty: TokenType) -> Result<Token> {
        let token = self.peek().cloned();
        match token {
            Some(ref token) if token.data() == &ty => Ok(self.next().unwrap()),
            Some(ref token) => Err(self.make_diagnostic(SyntaxError::Expected {
                expected: ty,
                found: *token.clone().data(),
            })),
            None => Err(self.make_diagnostic(SyntaxError::UnexpectedEof(ty))),
        }
    }

    fn next_is(&mut self, ty: TokenType) -> bool {
        self.peek().map_or(false, |t| t.data() == &ty)
    }

    fn next_one_of<T: AsRef<[TokenType]>>(&mut self, types: T) -> bool {
        types
            .as_ref()
            .into_iter()
            .map(|ty| self.next_is(*ty))
            .fold(false, |a, b| a || b)
    }

    fn synchronize(&mut self) {
        self.next();
        match self.peek().map(Spanned::data) {
            Some(TokenType::Semicolon)
            | Some(TokenType::Def)
            | Some(TokenType::Let)
            | Some(TokenType::Loop)
            | Some(TokenType::While)
            | Some(TokenType::If) => return,
            Some(_) => self.next(),
            None => None,
        };
    }

    // Error utilities

    fn make_diagnostic(&self, err: SyntaxError) -> Diagnostic {
        match err {
            SyntaxError::Expected { expected, found } => self.new_error(
                "unexpected token",
                self.new_label(format!("expected '{}', but got '{}'", expected, found)),
            ),
            SyntaxError::ExpectedOneOf { expected, found } => self.new_error(
                "unexpected token",
                self.new_label(format!(
                    "expected one of '{}', but got '{}'",
                    expected
                        .into_iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(" "),
                    found
                )),
            ),
            SyntaxError::ExpectedExpr => {
                self.new_error("expected expression", self.new_label("expected expression"))
            }
            SyntaxError::InvalidInteger(err) => self.new_error(
                "invalid integer",
                self.new_label(format!("invalid integer: {:?}", err.code)),
            ),
            SyntaxError::UnexpectedEof(ty) => {
                let span = self.span.end();
                let label = Label::primary(self.file, Span::new(span, span))
                    .with_message(format!("expected '{}', but found eof", ty));
                self.new_error("unexpected eof", label)
            }
        }
    }

    fn new_error(&self, msg: impl Into<String>, primary_label: Label) -> Diagnostic {
        Diagnostic::error()
            .with_message(msg)
            .with_labels(vec![primary_label])
    }

    fn new_label(&self, msg: impl Into<String>) -> Label {
        Label::primary(self.file, self.span).with_message(msg)
    }
}

/// Expression parsing
impl<'input> Parser<'input> {
    pub fn next_expression(&mut self) -> Result<ast::Expr> {
        self.next_equality()
    }

    binary_op!(
        next_equality,
        next_comparison,
        TokenType::NotEqual,
        TokenType::EqualEqual
    );

    binary_op!(
        next_comparison,
        next_addition,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Less,
        TokenType::LessEqual
    );

    binary_op!(
        next_addition,
        next_multiplication,
        TokenType::Plus,
        TokenType::Minus
    );

    binary_op!(
        next_multiplication,
        next_unary,
        TokenType::Star,
        TokenType::Slash
    );

    fn next_unary(&mut self) -> Result<ast::Expr> {
        if self.next_one_of([TokenType::Bang, TokenType::Minus]) {
            let token = self.next().unwrap();
            let span = token.span();
            let op: ast::UnaryOperation = token.into_inner().into();

            let right = self.next_unary()?;
            let span = span.merge(right.span());
            return Ok(span.span(ast::ExprKind::Unary {
                op,
                expr: Box::new(right),
            }));
        }
        self.next_primary()
    }

    fn next_primary(&mut self) -> Result<ast::Expr> {
        match self.next() {
            Some(token) => match token.data() {
                TokenType::String => self.next_string(token),
                TokenType::Integer => self.next_integer(token),
                TokenType::Bool => self.next_bool(token),
                TokenType::LeftParen => {
                    let expr = self.next_expression()?;
                    self.eat(TokenType::RightParen)?;
                    Ok(token.span().span(ast::ExprKind::Grouping(Box::new(expr))))
                }
                ty => Err(self.make_diagnostic(SyntaxError::ExpectedOneOf {
                    found: *ty,
                    expected: vec![
                        TokenType::LeftParen,
                        TokenType::Integer,
                        TokenType::String,
                        TokenType::Bool,
                    ],
                })),
            },
            None => Err(self.make_diagnostic(SyntaxError::ExpectedExpr)),
        }
    }

    fn next_string(&mut self, token: Token) -> Result<ast::Expr> {
        let src = self.files.source(self.file);
        let string = token.span_ref().index(src).to_string();
        Ok(token.span().span(ast::ExprKind::String(string)))
    }

    fn next_integer(&mut self, token: Token) -> Result<ast::Expr> {
        let src = self.files.source(self.file);
        let num = token.span_ref().index(src);
        let num = lexical::parse::<i64, _>(num)
            .map_err(|err| self.make_diagnostic(SyntaxError::InvalidInteger(err)))?;
        Ok(token.span_ref().span(ast::ExprKind::Int(num)))
    }

    fn next_bool(&mut self, token: Token) -> Result<ast::Expr> {
        let src = self.files.source(self.file);
        let src = token.span_ref().index(src);
        match src {
            "true" => Ok(token.span().span(ast::ExprKind::Bool(true))),
            "false" => Ok(token.span().span(ast::ExprKind::Bool(false))),
            _ => unreachable!(),
        }
    }
}
