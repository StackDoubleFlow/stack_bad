mod ast;
mod error;
mod lexer;
mod parser;
mod token;

use clap::{AppSettings, Clap};
use error::Result;
use lexer::Lexer;
use parser::Parser;
use std::fs;
use token::TokenPair;

#[derive(Clap)]
#[clap(version = "0.1", author = "StackDoubleFlow <ojaslandge@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    // Input source file.
    input: String,
}

fn compile(src: &str) -> Result<()> {
    let tokens = Lexer::new(src).lex()?;
    let pairs: Result<Vec<TokenPair>> = tokens
        .chunks(2)
        .map(|ts| (ts[0].clone(), ts[1].clone()))
        .map(|(s, b)| TokenPair::new(s, b))
        .collect();
    let mut pairs = pairs?;
    let magic = pairs.remove(0);
    match magic.data {
        [0, 0, 0, 0, 0, 0, 0, 0] => {}
        _ => return Err(magic.error()),
    }
    let ast = Parser::new(pairs).parse()?;
    dbg!(ast);
    Ok(())
}

fn main() {
    let opts = Opts::parse();

    let src = fs::read_to_string(&opts.input).unwrap();
    if let Err(err) = compile(&src) {
        println!("--> {}:{}:{}", opts.input, err.line, err.col);
        println!("{}", src.lines().nth(err.line - 1).unwrap());
        println!("{}^ {}", " ".repeat(err.col - 1), err);
    }
}
