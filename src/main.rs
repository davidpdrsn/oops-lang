// #![deny(unused_imports)]

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
    let source_text = ok_or_exit(fs::read_to_string(opt.file));

    let tokens = lex(&source_text);
    let ast = ok_or_exit(parse(tokens));

    println!("{:#?}", ast);
}

fn ok_or_exit<T, E: std::error::Error>(value: Result<T, E>) -> T {
    match value {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
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
