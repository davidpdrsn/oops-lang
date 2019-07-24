mod ast;

use crate::{
    lex::{self, ParseToken, Token},
    Pos,
};
use std::fmt;

pub use ast::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Stmt<'a>>, ParseError> {
    let mut stream = ParseStream::new(tokens);
    let ast = stream.parse_node::<LetLocal>()?;
    Ok(vec![Stmt::LetLocal(ast)])
}

struct ParseStream<'a> {
    tokens: Vec<Token<'a>>,
    current_position: usize,
}

impl<'a> ParseStream<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current_position: 0,
        }
    }

    fn parse_node<T: Parse<'a>>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }

    /// Get the next token and advance the current position
    fn parse_token<T: ParseToken<'a>>(&mut self) -> Result<&T, ParseError> {
        let token = &self.tokens[self.current_position];
        self.current_position += 1;

        let node = T::from_token(token);

        node.ok_or_else(|| {
            ParseError::Error(format!(
                "Expected '{}' but got '{}'",
                T::debug_name(),
                token
            ))
        })
    }

    // /// Get the next token and without advancing the current position
    // fn peek(&mut self) -> &Token<'a> {
    //     &self.tokens[self.current_position]
    // }
}

#[derive(Debug)]
pub enum ParseError {
    Error(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Error(reason) => write!(f, "Parse error: {}", reason),
        }
    }
}

impl std::error::Error for ParseError {}

trait Parse<'a>: Sized {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError>;
}

impl<'a> Parse<'a> for LetLocal<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::Let>()?.pos;
        let ident = stream.parse_node::<Ident>()?;
        stream.parse_token::<lex::Eq>()?;
        let body = stream.parse_node::<Expr>()?;
        let end_pos = stream.parse_token::<lex::Semicolon>()?.pos;

        Ok(LetLocal {
            ident,
            body,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Ident<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::Name { name, pos } = stream.parse_token()?;
        let ident = Ident { name, pos: *pos };
        Ok(ident)
    }
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::Digit { digit, pos } = stream.parse_token()?;
        let digit = Digit {
            digit: *digit,
            pos: *pos,
        };
        Ok(Expr::Digit(digit))
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::{lex::lex, Pos};

    #[test]
    fn basic() {
        let program = "let number = 1;";
        let tokens = lex(&program);
        let ast = parse(tokens).expect("parse error");

        assert_eq!(
            ast,
            vec![Stmt::LetLocal(LetLocal {
                ident: Ident {
                    name: "number",
                    pos: Pos::new(4, 10)
                },
                body: Expr::Digit(Digit {
                    digit: 1,
                    pos: Pos::new(13, 14)
                }),
                pos: Pos::new(0, 15),
            })]
        );
    }
}
