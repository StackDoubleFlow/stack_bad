use crate::error::{Error, Result};
use crate::token::{Token, TokenData};
use std::iter::Peekable;
use std::str::Chars;

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

    pub fn lex(mut self) -> Result<Vec<Token>> {
        loop {
            if self.peek_next().is_none() {
                return Ok(self.tokens);
            }
            let ch = self.next()?;

            self.col += 1;
            match ch {
                's' => {
                    let col = self.col;
                    let mut data = [1, 0, 0, 0, 0];
                    let mut prev_ch = 's';
                    loop {
                        match self.peek_next() {
                            Some(' ' | '\n') if prev_ch == 'k' => break,
                            None => return Err(self.error()),
                            _ => {}
                        }
                        let ch = self.next()?;
                        match ch {
                            's' => data[0] += 1,
                            't' => data[1] += 1,
                            'a' => data[2] += 1,
                            'c' => data[3] += 1,
                            'k' => data[4] += 1,
                            _ => return Err(self.error()),
                        }
                        prev_ch = ch;
                    }
                    let sum = data.iter().sum::<u32>() - 1;
                    self.col += sum as usize;
                    self.push_token(col, TokenData::Stack(data));
                }
                'b' => {
                    let col = self.col;
                    let mut data = [1, 0, 0];
                    let mut prev_ch = 'b';
                    loop {
                        match self.peek_next() {
                            Some(' ' | '\n') | None if prev_ch == 'd' => break,
                            _ => {}
                        }
                        let ch = self.next()?;
                        match ch {
                            'b' => data[0] += 1,
                            'a' => data[1] += 1,
                            'd' => data[2] += 1,
                            _ => return Err(self.error()),
                        }
                        prev_ch = ch;
                    }
                    let sum = data.iter().sum::<u32>() - 1;
                    self.col += sum as usize;
                    self.push_token(col, TokenData::Bad(data));
                }
                '#' => {
                    while matches!(self.peek_next(), Some(c) if c != '\n') {
                        self.next()?;
                        self.col += 1;
                    }
                }
                '\n' => {
                    self.col = 0;
                    self.line += 1;
                }
                ' ' => {}
                _ => return Err(self.error()),
            }
        }
    }

    fn error(&self) -> Error {
        Error {
            line: self.line,
            col: self.col,
        }
    }

    fn push_token(&mut self, col: usize, td: TokenData) {
        self.tokens.push(Token {
            line: self.line,
            col,
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
