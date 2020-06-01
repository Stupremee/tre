use super::token::{Token, TokenType};
use std::{iter::Peekable, str::Chars};

const KEYWORDS: [(&'static str, TokenType); 10] = [
    ("def", TokenType::Def),
    ("let", TokenType::Let),
    ("loop", TokenType::Loop),
    ("while", TokenType::While),
    ("for", TokenType::For),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("or", TokenType::Or),
    ("and", TokenType::And),
    ("break", TokenType::Break),
];

macro_rules! match_char {
    ($self:ident, $ty:ident) => {{
        $self.next();
        Some($self.token(TokenType::$ty))
    }};
}

macro_rules! match_two_chars {
    ($self:ident, $ty:ident, $se:expr, $ty2:ident) => {{
        $self.next();
        match $self.peek() {
            Some($se) => {
                $self.next();
                Some($self.token(TokenType::$ty2))
            }
            Some(_) | None => Some($self.token(TokenType::$ty)),
        }
    }};
}

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
            '(' => match_char!(self, LeftParen),
            ')' => match_char!(self, RightParen),
            '{' => match_char!(self, LeftBracket),
            '}' => match_char!(self, RightBracket),
            '[' => match_char!(self, LeftBrace),
            ']' => match_char!(self, RightBrace),

            '!' => match_two_chars!(self, Bang, '=', NotEqual),
            '+' => match_char!(self, Plus),
            '-' => match_char!(self, Minus),
            '*' => match_two_chars!(self, Star, '*', StarStar),
            '/' => match_char!(self, Slash),
            ':' => match_char!(self, Colon),
            '.' => match_char!(self, Dot),
            ',' => match_char!(self, Comma),

            '=' => match_two_chars!(self, Equal, '=', EqualEqual),
            '>' => match_two_chars!(self, Greater, '=', GreaterEqual),
            '<' => match_two_chars!(self, Less, '=', LessEqual),

            '#' => {
                while self.peek().map_or(false, |c| c != &'\n') {
                    self.next();
                }
                self.next_token()
            }
            '"' => self.string(),
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

        self.keyword_or_ident()
    }

    fn keyword_or_ident(&mut self) -> Option<Token<'input>> {
        let mut token = self.token(TokenType::Identifier);
        token.ty = KEYWORDS
            .iter()
            .filter(|t| t.0 == token.repr)
            .map(|t| t.1)
            .next()
            .unwrap_or(TokenType::Identifier);
        Some(token)
    }

    fn number(&mut self) -> Option<Token<'input>> {
        // TODO: Add floating numbers
        // TODO: Add more number bases
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

    fn string(&mut self) -> Option<Token<'input>> {
        // Consume the `"` before the string
        assert_eq!(self.next().unwrap_or('\0'), '"');

        while self.peek().map_or(false, |c| c != &'"') {
            let c = self.next().unwrap_or('\0');
            if c == '\\' && self.peek().unwrap_or(&'\0') == &'"' {
                self.next();
            }
        }

        // Consume the `"` after the string
        if self.peek().unwrap_or(&'\0') == &'"' {
            assert_eq!(self.next().unwrap_or('\0'), '"');
        }

        Some(self.token(TokenType::String))
    }
}

impl<'input> IntoIterator for Lexer<'input> {
    type IntoIter = TokenStream<'input>;
    type Item = Token<'input>;

    fn into_iter(self) -> Self::IntoIter {
        TokenStream { lexer: self }
    }
}

#[derive(Debug)]
pub struct TokenStream<'lexer> {
    lexer: Lexer<'lexer>,
}

impl<'lexer> Iterator for TokenStream<'lexer> {
    type Item = Token<'lexer>;

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
        let s = "(){}[]";
        let tokens = lex_input(s);
        let expected = vec![
            token!(LeftParen, s, "("),
            token!(RightParen, s, ")"),
            token!(LeftBracket, s, "{"),
            token!(RightBracket, s, "}"),
            token!(LeftBrace, s, "["),
            token!(RightBrace, s, "]"),
        ];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_some_tokens() {
        let s = "!! ++ = != ** * * :: ., == != < <= >= > / - - # - / != ===";
        let tokens: Vec<_> = lex_input(s).into_iter().map(|t| t.ty).collect();
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
        let tokens: Vec<_> = lex_input(s).into_iter().map(|t| (t.ty, t.repr)).collect();
        let expected = vec![
            (TokenType::String, r#""Hello, world""#),
            (TokenType::String, r#""Does this work?""#),
            (TokenType::String, r#""I hope so""#),
            (TokenType::String, r#""Escaping: \"""#),
        ];
        assert_eq!(expected, tokens);
    }

    fn lex_input(input: &'_ str) -> Vec<Token<'_>> {
        let lexer = Lexer::new(input).into_iter();
        lexer.collect()
    }
}
