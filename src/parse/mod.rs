mod ast;

use crate::{
    lex::{self, ParseToken, Token},
    Pos,
};
use std::fmt;

pub use ast::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Stmt<'a>>, ParseError> {
    let mut stream = ParseStream::new(tokens);

    let mut acc = vec![];

    loop {
        if stream.at_end() {
            break;
        } else {
            acc.push(stream.parse_node::<Stmt>()?);
        }
    }

    Ok(acc)
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

    fn at_end(&self) -> bool {
        self.current_position >= self.tokens.len()
    }

    fn parse_node<T: Parse<'a>>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }

    fn try_parse_node<T: Parse<'a>>(&mut self) -> Option<T> {
        let start_position = self.current_position;

        if let Ok(node) = T::parse(self) {
            Some(node)
        } else {
            self.current_position = start_position;
            None
        }
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

macro_rules! try_parse_node {
    ( $ty:ty, $stream:expr ) => {
        if let Some(inner) = $stream.try_parse_node::<$ty>() {
            return Ok(inner.into());
        }
    };
}

impl<'a> Parse<'a> for Stmt<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        try_parse_node!(LetLocal, stream);
        // try_parse_node!(LetIVar, stream);
        // try_parse_node!(MessageSend, stream);

        unimplemented!()
    }
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
        try_parse_node!(Local, stream);
        // try_parse_node!(IVar, stream);
        // try_parse_node!(MessageSend, stream);
        // try_parse_node!(Selector, stream);
        // try_parse_node!(Block, stream);
        try_parse_node!(Digit, stream);
        // try_parse_node!(List, stream);
        try_parse_node!(True, stream);
        try_parse_node!(False, stream);
        try_parse_node!(Self_, stream);

        panic!("expr parse failed")
    }
}

impl<'a> Parse<'a> for Digit {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::Digit { digit, pos } = stream.parse_token()?;
        Ok(Digit {
            digit: *digit,
            pos: *pos,
        })
    }
}

impl<'a> Parse<'a> for Local<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::Name { name, pos } = stream.parse_token()?;
        Ok(Local(Ident { name, pos: *pos }))
    }
}

impl<'a> Parse<'a> for True {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::True { pos } = stream.parse_token()?;
        Ok(True(*pos))
    }
}

impl<'a> Parse<'a> for False {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::False { pos } = stream.parse_token()?;
        Ok(False(*pos))
    }
}

impl<'a> Parse<'a> for Self_ {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::Self_ { pos } = stream.parse_token()?;
        Ok(Self_(*pos))
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::{lex::lex, Pos};

    #[test]
    fn let_digit() {
        let program = "let number = 1;";
        let tokens = lex(&program);
        let ast = ok_or_panic(parse(tokens));

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

    #[test]
    fn let_name() {
        let program = "let a = b;";
        let tokens = lex(&program);
        let ast = ok_or_panic(parse(tokens));

        assert_eq!(
            ast,
            vec![Stmt::LetLocal(LetLocal {
                ident: Ident {
                    name: "a",
                    pos: Pos::new(4, 5)
                },
                body: Expr::Local(Local(Ident {
                    name: "b",
                    pos: Pos::new(8, 9),
                })),
                pos: Pos::new(0, 10),
            })]
        );
    }

    fn ok_or_panic<T, E: std::error::Error>(value: Result<T, E>) -> T {
        match value {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}\n", e);
                panic!("error!")
            }
        }
    }
}
