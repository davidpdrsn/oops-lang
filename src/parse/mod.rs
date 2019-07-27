use crate::ast::*;
use crate::{
    error::{Error, Result},
    lex::{self, Token},
};
use std::fmt::Debug;

pub fn parse<'a>(tokens: &'a Vec<Token<'a>>) -> Result<Vec<Stmt<'a>>> {
    let mut stream = ParseStream::new(tokens);
    let acc = stream.parse_many::<Stmt>();

    if !stream.at_eof() {
        Err(Error::ParseError("Expected EOF, but wasn't".to_string()))
    } else {
        Ok(acc)
    }
}

pub struct ParseStream<'a> {
    tokens: &'a Vec<Token<'a>>,
    current_position: usize,
}

impl<'a> ParseStream<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current_position: 0,
        }
    }

    pub fn parse_token<T: lex::Parse<'a>>(&mut self) -> Result<'a, &T> {
        let token = &self.tokens[self.current_position];
        self.current_position += 1;
        let node = T::from_token(token);

        node.ok_or_else(|| {
            Error::ParseError(format!(
                "Expected '{}' but got '{}'",
                T::debug_name(),
                token
            ))
        })
    }

    pub fn try_parse_token<T: lex::Parse<'a>>(&mut self) -> Option<&T> {
        let start_position = self.current_position;

        let token = &self.tokens[self.current_position];
        self.current_position += 1;
        let node = T::from_token(token);

        if let Some(node) = node {
            Some(node)
        } else {
            self.current_position = start_position;
            None
        }
    }

    pub fn parse_node<T: Parse<'a>>(&mut self) -> Result<'a, T> {
        T::parse(self)
    }

    pub fn try_parse_node<T: Parse<'a>>(&mut self) -> Option<T> {
        let start_position = self.current_position;

        if let Ok(node) = T::parse(self) {
            Some(node)
        } else {
            self.current_position = start_position;
            None
        }
    }

    pub fn parse_specific_ident(&mut self, name: &str) -> Result<'a, Ident<'a>> {
        let ident = self.parse_node::<Ident>()?;

        if ident.name == name {
            Ok(ident)
        } else {
            Err(Error::ParseError(format!(
                "Expected class named '{}' but got '{}'",
                name, ident.name
            )))
        }
    }

    pub fn parse_specific_class_name(&mut self, name: &str) -> Result<'a, ClassName<'a>> {
        let class_name = self.parse_node::<ClassName>()?;

        if class_name.0.name == name {
            Ok(class_name)
        } else {
            Err(Error::ParseError(format!(
                "Expected class named '{}' but got '{}'",
                name, class_name.0.name
            )))
        }
    }

    pub fn parse_many<T: Debug + Parse<'a>>(&mut self) -> Vec<T> {
        let mut acc = vec![];
        loop {
            if self.at_eof() {
                break;
            }

            if let Some(node) = self.try_parse_node::<T>() {
                acc.push(node)
            } else {
                break;
            }
        }
        acc
    }

    pub fn parse_many_delimited<Node: Parse<'a>, Token: lex::Parse<'a>>(&mut self) -> Vec<Node> {
        let mut acc = vec![];
        loop {
            if self.at_eof() {
                break;
            }

            if let Some(node) = self.try_parse_node::<Node>() {
                acc.push(node)
            } else {
                break;
            }

            if self.try_parse_token::<Token>().is_none() {
                break;
            }
        }
        acc
    }

    pub fn at_eof(&self) -> bool {
        self.current_position >= self.tokens.len()
    }
}

pub trait Parse<'a>: Sized {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self>;
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::{lex::lex, Span};

    #[test]
    fn let_number() {
        let program = "let number = 1;";
        let tokens = lex(&program).unwrap();
        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            vec![Stmt::LetLocal(LetLocal {
                ident: Ident {
                    name: "number",
                    span: Span::new(4, 10)
                },
                body: Expr::Number(Number {
                    number: 1,
                    span: Span::new(13, 14)
                }),
                span: Span::new(0, 15),
            })]
        );
    }

    #[test]
    fn let_name() {
        let program = "let a = b;";
        let tokens = lex(&program).unwrap();
        let ast = parse(&tokens).unwrap();

        assert_eq!(
            ast,
            vec![Stmt::LetLocal(LetLocal {
                ident: Ident {
                    name: "a",
                    span: Span::new(4, 5)
                },
                body: Expr::Local(Local(Ident {
                    name: "b",
                    span: Span::new(8, 9),
                })),
                span: Span::new(0, 10),
            })]
        );
    }
}
