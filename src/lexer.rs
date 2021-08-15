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
                        let ch = self.next()?;
                        self.col += 1;
                        match ch {
                            's' => data[0] += 1,
                            't' => data[1] += 1,
                            'a' => data[2] += 1,
                            'c' => data[3] += 1,
                            'k' => data[4] += 1,
                            ' ' if prev_ch == 'k' => break,
                            '\n' if prev_ch == 'k' => {
                                self.col = 1;
                                self.line += 1;
                                break;
                            }
                            _ => return Err(self.error()),
                        }
                        prev_ch = ch;
                    }
                    self.push_token(col, TokenData::Stack(data));
                }
                'b' => {
                    let col = self.col;
                    let mut data = [1, 0, 0];
                    let mut prev_ch = 'b';
                    loop {
                        if self.peek_next().is_none() && prev_ch == 'd' {
                            break;
                        }
                        let ch = self.next()?;
                        self.col += 1;
                        match ch {
                            'b' => data[0] += 1,
                            'a' => data[1] += 1,
                            'd' => data[2] += 1,
                            ' ' if prev_ch == 'd' => break,
                            '\n' if prev_ch == 'd' => {
                                self.col = 1;
                                self.line += 1;
                                break;
                            }
                            _ => return Err(self.error()),
                        }
                        prev_ch = ch;
                    }
                    self.push_token(col, TokenData::Bad(data));
                }
                '#' => {
                    while self.next()? != '\n' {}
                    self.col = 1;
                    self.line += 1;
                }
                '\n' => {
                    self.col = 1;
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
