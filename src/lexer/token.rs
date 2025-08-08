use std::fmt::Display;

pub struct Token {
    pub r#type: TokenType,
    pub literal: Option<String>,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    // Literals
    Number,
    Str,
    Identifier,
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    SemiColon,
    Plus,
    Minus,
    Slash,
    Star,
    // One or multiple tokens
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    EqualEqual,
    Neg,
    NegEqual,
    // Reserved Keywords
    Keyword,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(l) => write!(f, "tok {:?} : {}", self.r#type, l),
            None => write!(f, "tok {:?}", self.r#type),
        }
    }
}
