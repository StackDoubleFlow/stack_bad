use crate::error::{Error, Result};

#[derive(Clone, Debug)]
pub enum TokenData {
    Stack([u32; 5]),
    Bad([u32; 3]),
}

#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub struct TokenPair {
    pub line: usize,
    pub col: usize,
    pub data: [u32; 8],
}

impl TokenPair {
    pub fn new(stack: Token, bad: Token) -> Result<TokenPair> {
        let mut data = [0; 8];

        match stack.data {
            TokenData::Stack(d) => {
                data[0] = d[0] - 1;
                data[1] = d[1] - 1;
                data[2] = d[2] - 1;
                data[3] = d[3] - 1;
                data[4] = d[4] - 1;
            }
            _ => return Err(stack.error()),
        }

        match bad.data {
            TokenData::Bad(d) => {
                data[5] = d[0] - 1;
                data[6] = d[1] - 1;
                data[7] = d[2] - 1;
            }
            _ => return Err(bad.error()),
        }

        Ok(TokenPair {
            line: stack.line,
            col: stack.col,
            data,
        })
    }

    pub fn error(&self) -> Error {
        Error {
            line: self.line,
            col: self.col,
        }
    }
}
