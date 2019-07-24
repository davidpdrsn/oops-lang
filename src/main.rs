mod lex;
mod parse;

use lex::lex;
use parse::parse;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

/// OOPS language interpreter
#[derive(StructOpt, Debug)]
#[structopt(name = "oops")]
struct Opt {
    /// File to run
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    // TODO: Don't `unwrap`, just eprintln with the error and exit(1)
    let source_text = fs::read_to_string(opt.file).unwrap();

    let tokens = lex(&source_text);
    println!("Lex:\n{:?}\n", tokens);

    // TODO: Don't `expect`, just eprintln with the error and exit(1)
    let ast = parse(tokens).expect("parse error");
    println!("Parse:\n{:?}\n", ast);
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Pos {
    pub from: usize,
    pub to: usize,
}

impl Pos {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    #[cfg(test)]
    pub fn from_with(start: usize, s: &str) -> Self {
        Self::new(start, start + s.len())
    }
}
