mod ast;

use crate::{lex::Token, Pos};

pub use ast::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Stmt<'a>>, ParseError> {
    let mut stream = ParseStream::new(tokens);
    let ast = stream.parse::<LetLocal>()?;
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

    fn parse<T: Parse<'a>>(&mut self) -> Result<T, ParseError> {
        T::parse(self)
    }

    /// Get the next token and advance the current position
    fn consume(&mut self) -> &Token<'a> {
        // TODO: handle out of bounds
        let token = &self.tokens[self.current_position];
        self.current_position += 1;
        token
    }

    /// Get the next token and without advancing the current position
    fn peek(&mut self) -> &Token<'a> {
        // TODO: handle out of bounds
        let token = &self.tokens[self.current_position];
        token
    }
}

#[derive(Debug)]
pub enum ParseError {
    AnyError(String),
}

trait Parse<'a>: Sized {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError>;
}

impl<'a> Parse<'a> for LetLocal<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        // TODO: Make this pattern nicer
        let start_pos = match stream.consume() {
            Token::Let(let_pos) => *let_pos,
            other => {
                return parse_error(format!("expected 'let' but got '{:?}'", other))
            }
        };

        let ident = stream.parse::<Ident>()?;

        match stream.consume() {
            Token::Eq(_) => {},
            other => {
                return parse_error(format!("expected '=' but got '{:?}'", other))
            }
        };

        let body = stream.parse::<Expr>()?;

        let end_pos = match stream.consume() {
            Token::Semicolon(semicolon_pos) => *semicolon_pos,
            other => {
                return parse_error(format!("expected ';' but got '{:?}'", other))
            }
        };

        Ok(LetLocal {
            ident,
            body,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Ident<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        if let Token::Name((name, pos)) = stream.consume() {
            let ident = Ident { name, pos: *pos };
            Ok(ident)
        } else {
            parse_error("expected name")
        }
    }
}

fn parse_error<T, S: Into<String>>(s: S) -> Result<T, ParseError> {
    Err(ParseError::AnyError("expected name".into()))
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        if let Token::Digit((digit, pos)) = stream.consume() {
            let digit = Digit {
                digit: *digit,
                pos: *pos,
            };
            Ok(Expr::Digit(digit))
        } else {
            Err(ParseError::AnyError("expected digit".to_string()))
        }
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

        // TODO: Validate that the indexes are current
        // Does it make most sense for the indexes to be inclusive or exclusive?
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
