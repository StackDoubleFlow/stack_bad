use crate::error::{Result, Error};

#[derive(Debug)]
pub enum TokenData {
    Stack([u32; 5]),
    Bad([u32; 3]),
}

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub col: usize,
    pub data: TokenData,
}

impl Token {
    fn error(&self) -> Error {
        Error {
            line: self.line,
            col: self.col,
        }
    }
}

pub struct TokenPair {
    pub line: usize,
    pub col: usize,
    pub data: [u32; 8]
}

impl TokenPair {
    fn new(stack: Token, bad: Token) -> Result<TokenPair> {
        let mut data = [0; 8];

        match stack.data {
            TokenData::Stack(d) => {
                data[0] = d[0];
                data[1] = d[1];
                data[2] = d[2];
                data[3] = d[3];
                data[4] = d[4];
            }
            _ => return Err(stack.error())
        }

        match bad.data {
            TokenData::Bad(d) => {
                data[5] = d[0];
                data[6] = d[1];
                data[7] = d[2];
            }
            _ => return Err(bad.error())
        }

        Ok(TokenPair {
            line: stack.line,
            col: stack.col,
            data,
        })
    }
}
