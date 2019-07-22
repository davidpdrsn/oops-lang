mod lex;

use lex::lex;
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
    let program = fs::read_to_string(opt.file).unwrap();
    let program = &program[0..=program.len() - 2];
    let tokens = lex(&program);
    dbg!(tokens);
}
