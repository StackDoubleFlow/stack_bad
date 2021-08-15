use crate::token::{Token, TokenData};
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
struct LexerError {
    line: usize,
    col: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "stack bad")
    }
}

impl Error for LexerError {}

type Result<T> = std::result::Result<T, LexerError>;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    line: usize,
    col: usize,
    source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            line: 1,
            col: 0,
            source: src.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<()> {
        loop {
            if self.peek_next().is_none() {
                return Ok(());
            }
            let ch = self.next()?;
            self.col += 1;
        }
        Ok(())
    }

    fn error(&self) -> LexerError {
        LexerError {
            line: self.line,
            col: self.col,
        }
    }

    fn push_token(&mut self, td: TokenData) {
        self.tokens.push(Token {
            line: self.line,
            col: self.col,
            data: td,
        });
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn next(&mut self) -> Result<char> {
        match self.source.next() {
            Some(ch) => Ok(ch),
            None => Err(self.error()),
        }
    }
}
