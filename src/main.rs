mod lexer;
mod token;
mod ast;
mod parser;
mod error;

use clap::{AppSettings, Clap};
use std::fs;
use lexer::Lexer;
use error::Result;

#[derive(Clap)]
#[clap(version = "0.1", author = "StackDoubleFlow <ojaslandge@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    input: String
}

fn compile(src: &str) -> Result<()> {
    let tokens = Lexer::new(src).lex()?;
    dbg!(tokens);
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
