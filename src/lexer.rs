use std::{iter::Peekable, str::Chars};

use token::{Token, TokenType};

mod error;
mod token;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    line: u32,
    column: u32,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let source = source.chars().peekable();

        Self {
            source,
            line: 0,
            column: 0,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> &[Token] {
        while let Some(next_char) = self.next() {
            match next_char {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '+' => self.add_token(TokenType::Plus, None),
                '-' => self.add_token(TokenType::Minus, None),
                '*' => self.add_token(TokenType::Star, None),
                '/' => {
                    if self.match_char('/') {
                        while *self.peek().unwrap() != '\n' {
                            _ = self.next()
                        }
                        self.line += 1;
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }
                ';' => self.add_token(TokenType::SemiColon, None),
                '=' => self.add_token_conditionally('=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.add_token_conditionally('=', TokenType::LessEqual, TokenType::Less),
                '>' => {
                    self.add_token_conditionally('=', TokenType::GreaterEqual, TokenType::Greater)
                }
                '!' => self.add_token_conditionally('=', TokenType::NegEqual, TokenType::Neg),
                '"' => self.lex_string(),
                c if c.is_ascii_digit() => self.lex_number(),
                c if c.is_ascii_alphanumeric() => self.lex_identifier(),
                c if c.is_whitespace() => {
                    if c == '\n' {
                        self.line += 1;
                    }
                }
                _ => {
                    // ERROR: Unsupported character
                }
            }
        }

        &self.tokens
    }

    fn lex_identifier(&mut self) {
        let mut literal = String::new();

        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_alphanumeric() || *c == '_' {
                    literal.push(self.next().unwrap());
                } else {
                    _ = self.next();
                    break;
                }
            }
        }

        // CHECK: if the literal corresponds to a keword

        self.add_token(TokenType::Identifier, Some(literal));
    }

    fn lex_number(&mut self) {
        let mut literal = String::new();

        loop {
            if let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    literal.push(self.next().unwrap());
                } else {
                    _ = self.next();
                    break;
                }
            }
        }

        self.add_token(TokenType::Number, Some(literal));
    }

    fn lex_string(&mut self) {
        let mut literal = String::new();

        loop {
            match self.peek() {
                Some(c) => {
                    if *c != '"' {
                        if *c == '\n' {
                            self.line += 1;
                        }
                        literal.push(self.next().unwrap());
                    } else {
                        _ = self.next();
                        break;
                    }
                }
                None => {
                    // ERROR: Unterminated string!
                }
            }
        }

        self.add_token(TokenType::Str, Some(literal));
    }

    fn add_token_conditionally(
        &mut self,
        expected: char,
        true_case: TokenType,
        false_case: TokenType,
    ) {
        if self.match_char(expected) {
            self.add_token(true_case, None);
        } else {
            self.add_token(false_case, None);
        }
    }

    fn add_token(&mut self, token_ty: TokenType, literal: Option<String>) {
        self.tokens.push(Token {
            r#type: token_ty,
            literal,
            line: self.line,
            column: self.column,
        });
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.source.next_if_eq(&expected).is_some() {
            self.column += 1;
            true
        } else {
            false
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.column += 1;
        self.source.next()
    }
}
