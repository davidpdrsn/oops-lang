mod ast;

use crate::lex::Token;
pub use ast::*;

pub fn parse<'a>(tokens: &'a Vec<Token<'a>>) -> Vec<Statement<'a>> {
    vec![]
}
