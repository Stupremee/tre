use super::token::{Token, TokenType};
use crate::Span;
use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub struct Lexer<'input> {
    input: &'input str,
    iter: Peekable<Chars<'input>>,
    start_pos: usize,
    pos: usize,
}

// New method and utility methods
impl<'input> Lexer<'input> {
    pub fn new(source: &'input str) -> Self {
        Self {
            iter: source.chars().peekable(),
            input: source,
            start_pos: 0,
            pos: 0,
        }
    }

    #[inline]
    fn next(&mut self) -> Option<char> {
        let c = self.iter.next();
        if c.is_some() {
            self.pos += 1;
        }
        c
    }

    #[inline]
    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    #[inline]
    fn token(&self, ty: TokenType) -> Token {
        let span = Span::from(self.start_pos..self.pos);
        span.span(ty)
    }

    #[inline]
    fn current_slice(&self) -> &'input str {
        &self.input[self.start_pos..self.pos]
    }
}

// Lexing methods
impl<'input> Lexer<'input> {
    pub fn next_token(&mut self) -> Option<Token> {
        self.start_pos = self.pos;
        let kind = match self.next()? {
            '!' => match self.peek()? {
                '=' => {
                    assert_eq!(self.next().unwrap_or('\0'), '=');
                    TokenType::NotEqual
                }
                _ => TokenType::Bang,
            },
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => match self.peek()? {
                '*' => {
                    assert_eq!(self.next().unwrap_or('\0'), '*');
                    TokenType::StarStar
                }
                _ => TokenType::Star,
            },
            '/' => TokenType::Slash,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftCurly,
            '}' => TokenType::RightCurly,

            '=' => match self.peek()? {
                '=' => {
                    assert_eq!(self.next().unwrap_or('\0'), '=');
                    TokenType::EqualEqual
                }
                _ => TokenType::Equal,
            },
            '<' => match self.peek()? {
                '=' => {
                    assert_eq!(self.next().unwrap_or('\0'), '=');
                    TokenType::LessEqual
                }
                _ => TokenType::Less,
            },
            '>' => match self.peek()? {
                '=' => {
                    assert_eq!(self.next().unwrap_or('\0'), '=');
                    TokenType::GreaterEqual
                }
                _ => TokenType::Greater,
            },

            '"' => return self.string(),
            c if is_identifier(&c) => return self.identifier(),
            c if c.is_digit(10) => return self.number(),
            c if c.is_whitespace() => return self.next_token(),
            _ => return None,
        };

        Some(self.token(kind))
    }

    fn identifier(&mut self) -> Option<Token> {
        while self.peek().map_or(false, |c| is_identifier(&c)) {
            self.next();
        }

        self.keyword_or_ident()
    }

    fn keyword_or_ident(&mut self) -> Option<Token> {
        let mut token = self.token(TokenType::Identifier);
        token.0 = match self.current_slice() {
            "def" => TokenType::Def,
            "let" => TokenType::Let,
            "loop" => TokenType::Loop,
            "while" => TokenType::While,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "true" | "false" => TokenType::Bool,
            _ => TokenType::Identifier,
        };
        Some(token)
    }

    fn number(&mut self) -> Option<Token> {
        while self.peek().map_or(false, |c| is_digit(c, 10)) {
            self.next();
        }

        if self.peek().map_or(false, |c| c == &'.') {
            self.next();
            while self.peek().map_or(false, |c| is_digit(c, 10)) {
                self.next();
            }
            return Some(self.token(TokenType::Float));
        }

        Some(self.token(TokenType::Integer))
    }

    fn string(&mut self) -> Option<Token> {
        while self.peek().map_or(false, |c| c != &'"') {
            let c = self.next().unwrap_or('\0');
            if c == '\\' && self.peek().unwrap_or(&'\0') == &'"' {
                self.next();
            }
        }

        // Consume the `"` after the string
        let has_quote = self.next().unwrap_or('\0') == '"';

        // We need to create a custom token here because we have
        // to remove the double quotes in the front and in the back.
        // If the string does not end with an quote, we have to include the last character.
        let range = (self.start_pos + 1)..(self.pos - (has_quote as usize));
        let range = Span::from(range);
        Some(range.span(TokenType::String))
    }
}

impl<'input> IntoIterator for Lexer<'input> {
    type IntoIter = TokenStream<'input>;
    type Item = Token;

    fn into_iter(self) -> Self::IntoIter {
        TokenStream { lexer: self }
    }
}

#[derive(Debug)]
pub struct TokenStream<'lexer> {
    lexer: Lexer<'lexer>,
}

impl<'lexer> Iterator for TokenStream<'lexer> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next_token()
    }
}

fn is_digit(c: &char, radix: u32) -> bool {
    c.is_digit(radix) || c == &'_'
}

fn is_identifier(c: &char) -> bool {
    match c {
        'A'..='Z' | 'a'..='z' | '_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! token {
        ($t:ident, $s:ident, $r:expr) => {{
            let r = $r;
            let idx = $s.find($r).expect("couldn't find pattern in str");
            let range = idx..(idx + r.len());
            let range = Span::from(range);
            range.span(TokenType::$t)
        }};
    }

    #[test]
    fn test_identifier() {
        let s = "_ABC_DEF some_thing\nmore_IdeNt true false";
        let tokens = lex_input(s);
        let expected = vec![
            token!(Identifier, s, "_ABC_DEF"),
            token!(Identifier, s, "some_thing"),
            token!(Identifier, s, "more_IdeNt"),
            token!(Bool, s, "true"),
            token!(Bool, s, "false"),
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_integer() {
        let s = "1337 1234_5678\n12321";
        let tokens = lex_input(s);
        let expected = vec![
            token!(Integer, s, "1337"),
            token!(Integer, s, "1234_5678"),
            token!(Integer, s, "12321"),
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_float() {
        let s = "13.10 12.34 0.9999 78.";
        let tokens = lex_input(s);
        let expected = vec![
            token!(Float, s, "13.10"),
            token!(Float, s, "12.34"),
            token!(Float, s, "0.9999"),
            token!(Float, s, "78."),
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_paren() {
        let s = "(){}";
        let tokens = lex_input(s);
        let expected = vec![
            token!(LeftParen, s, "("),
            token!(RightParen, s, ")"),
            token!(LeftCurly, s, "{"),
            token!(RightCurly, s, "}"),
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_some_tokens() {
        let s = "!! ++ = != ** * * :: ., == != < <= >= > / - - # - / != ===";
        let tokens: Vec<_> = lex_input(s).into_iter().map(|t| t.into_inner()).collect();
        let expected = vec![
            TokenType::Bang,
            TokenType::Bang,
            TokenType::Plus,
            TokenType::Plus,
            TokenType::Equal,
            TokenType::NotEqual,
            TokenType::StarStar,
            TokenType::Star,
            TokenType::Star,
            TokenType::Colon,
            TokenType::Colon,
            TokenType::Dot,
            TokenType::Comma,
            TokenType::EqualEqual,
            TokenType::NotEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::GreaterEqual,
            TokenType::Greater,
            TokenType::Slash,
            TokenType::Minus,
            TokenType::Minus,
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_strings() {
        let s = r#" "Hello, world" "Does this work?" "I hope so" "Escaping: \"" "#;
        let tokens: Vec<_> = lex_input(s)
            .into_iter()
            .map(|t| (t.0, t.span().index(s)))
            .collect();
        let expected = vec![
            (TokenType::String, r#"Hello, world"#),
            (TokenType::String, r#"Does this work?"#),
            (TokenType::String, r#"I hope so"#),
            (TokenType::String, r#"Escaping: \""#),
        ];
        assert_eq!(expected, tokens);
    }

    fn lex_input(input: &'_ str) -> Vec<Token> {
        let lexer = Lexer::new(input).into_iter();
        lexer.collect()
    }
}
