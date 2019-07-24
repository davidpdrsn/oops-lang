use super::ast::*;
use crate::lex::{self, Token};
use std::fmt::{self, Debug};

pub struct ParseStream<'a> {
    tokens: Vec<Token<'a>>,
    current_position: usize,
}

impl<'a> ParseStream<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current_position: 0,
        }
    }

    pub fn parse_token<T: lex::Parse<'a>>(&mut self) -> Result<&T, ParseError> {
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

    pub fn parse_node<T: Parse<'a>>(&mut self) -> Result<T, ParseError> {
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

    pub fn parse_specific_ident(&mut self, name: &str) -> Result<Ident<'a>, ParseError> {
        let ident = self.parse_node::<Ident>()?;

        if ident.name == name {
            Ok(ident)
        } else {
            Err(ParseError::Error(format!(
                "Expected class named '{}' but got '{}'",
                name, ident.name
            )))
        }
    }

    pub fn parse_specific_class_name(&mut self, name: &str) -> Result<ClassName<'a>, ParseError> {
        let class_name = self.parse_node::<ClassName>()?;

        if class_name.0.name == name {
            Ok(class_name)
        } else {
            Err(ParseError::Error(format!(
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
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError>;
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
