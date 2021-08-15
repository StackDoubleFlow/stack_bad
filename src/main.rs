mod ast;
mod codegen;
mod error;
mod lexer;
mod parser;
mod token;

use clap::{AppSettings, Clap};
use codegen::Codegen;
use error::Result;
use lexer::Lexer;
use parser::Parser;
use std::fs;
use std::path::Path;
use token::TokenPair;

#[derive(Clap)]
#[clap(version = "0.1", author = "StackDoubleFlow <ojaslandge@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Input source file path.
    input: String,
    /// Output object file path.
    #[clap(short)]
    output: Option<String>,
}

fn compile(src: &str, output_path: &str) -> Result<()> {
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
    dbg!(&ast);
    let path = Path::new(output_path);
    Codegen::compile(ast, path);
    Ok(())
}

fn main() {
    let opts = Opts::parse();

    let output_path = opts.output.clone().unwrap_or_else(|| {
        let input_path = Path::new(&opts.input);
        input_path
            .with_extension("o")
            .into_os_string()
            .into_string()
            .unwrap()
    });

    let src = fs::read_to_string(&opts.input).unwrap();
    if let Err(err) = compile(&src, &output_path) {
        println!("--> {}:{}:{}", opts.input, err.line, err.col);
        println!("{}", src.lines().nth(err.line - 1).unwrap());
        println!("{}^ {}", " ".repeat(err.col - 1), err);
    }
}
