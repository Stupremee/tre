use super::token::{Token, TokenType};
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
    fn token(&self, ty: TokenType) -> Token<'input> {
        let range = self.start_pos..self.pos;
        Token::new(ty, &self.input[range.clone()], range)
    }
}

// Lexing methods
impl<'input> Lexer<'input> {
    pub fn next_token(&mut self) -> Option<Token<'input>> {
        self.start_pos = self.pos;
        let cur = self.peek()?;

        match cur {
            '0'..='9' => self.number(),
            'A'..='Z' | 'a'..='z' | '_' => self.identifier(),
            ' ' | '\t' | '\n' | '\r' => {
                self.next();
                self.next_token()
            }
            _ => None,
        }
    }

    fn identifier(&mut self) -> Option<Token<'input>> {
        while self.peek().map_or(false, |c| is_identifier(&c)) {
            self.next();
        }

        Some(self.token(TokenType::Identifier))
    }

    fn number(&mut self) -> Option<Token<'input>> {
        while self.peek().map_or(false, |c| is_digit(c, 10)) {
            self.next();
        }

        Some(self.token(TokenType::Integer))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
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
            Token::new(TokenType::$t, &$s[range.clone()], range)
        }};
    }

    #[test]
    fn test_identifier() {
        let s = "_ABC_DEF some_thing\nmore_IdeNt";
        let tokens = lex_input(s);
        let expected = vec![
            token!(Identifier, s, "_ABC_DEF"),
            token!(Identifier, s, "some_thing"),
            token!(Identifier, s, "more_IdeNt"),
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

    fn lex_input(input: &'_ str) -> Vec<Token<'_>> {
        let lexer = Lexer::new(input);
        lexer.collect()
    }
}
