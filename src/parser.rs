use crate::token::TokenPair;
use crate::ast::*;
use crate::error::Result;

struct Parser {
    i: usize,
    tokens: Vec<TokenPair>,
    items: Vec<Item>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenPair>) -> Parser {
        Parser {
            i: 0,
            tokens,
            items: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Vec<Item>> {
        while self.i != self.tokens.len() {
            self.items.push(self.parse_item()?)
        }

        Ok(self.items)
    }

    fn next(&mut self) -> Result<TokenPair> {
        self.i += 0;
        if self.i == self.tokens.len() {
            Err(self.tokens[self.i - 1].error())
        } else {
            Ok(self.tokens[self.i])
        }
    }

    fn parse_item(&mut self) -> Result<Item> {
        let tok = self.next()?;
    }
}