// #![deny(unused_imports)]
#![deny(unused_must_use)]

#[macro_use]
mod error;
mod interpret;
mod lex;
mod parse;
mod ast;
mod prep;

use interpret::interpret;
use lex::lex;
use parse::parse;
use std::path::PathBuf;
use std::{fmt, fs};
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
    let source_text = fs::read_to_string(opt.file).unwrap();

    let tokens = ok_or_exit(lex(&source_text));
    let ast = ok_or_exit(parse(&tokens));
    ok_or_exit(interpret(&ast));
}

fn ok_or_exit<T>(result: error::Result<'_, T>) -> T {
    match result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Span({}..{})", self.from, self.to)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to {}", self.from, self.to)
    }
}

impl Span {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    #[cfg(test)]
    pub fn from_with(start: usize, s: &str) -> Self {
        Self::new(start, start + s.len())
    }
}
