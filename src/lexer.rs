use std::{iter::Peekable, str::Chars};

mod error;
mod token;

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    line: u32,
    column: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let source = source.chars().peekable();

        Self {
            source,
            line: 0,
            column: 0,
        }
    }
}
