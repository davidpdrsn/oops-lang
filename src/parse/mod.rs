mod ast;

use crate::{
    lex::{self, Token},
    Pos,
};
use std::fmt::{self, Debug};

pub use ast::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Vec<Stmt<'a>>, ParseError> {
    let mut stream = ParseStream::new(tokens);
    let acc = stream.parse_many::<Stmt>();

    if !stream.at_end() {
        Err(ParseError::Error("Expected EOF, but wasn't".to_string()))
    } else {
        Ok(acc)
    }
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

    /// Get the next token and advance the current position
    fn parse_token<T: lex::Parse<'a>>(&mut self) -> Result<&T, ParseError> {
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

    fn try_parse_token<T: lex::Parse<'a>>(&mut self) -> Option<&T> {
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

    fn parse_specific_ident(&mut self, name: &str) -> Result<Ident<'a>, ParseError> {
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

    fn parse_specific_class_name(&mut self, name: &str) -> Result<ClassName<'a>, ParseError> {
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

    fn parse_many<T: Debug + Parse<'a>>(&mut self) -> Vec<T> {
        let mut acc = vec![];
        loop {
            if self.at_end() {
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

    fn parse_many_delimited<Node: Parse<'a>, Token: lex::Parse<'a>>(&mut self) -> Vec<Node> {
        let mut acc = vec![];
        loop {
            if self.at_end() {
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

    fn at_end(&self) -> bool {
        self.current_position >= self.tokens.len()
    }
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
        try_parse_node!(DefineClass, stream);
        try_parse_node!(DefineMethod, stream);
        try_parse_node!(LetLocal, stream);
        try_parse_node!(LetIVar, stream);
        try_parse_node!(MessageSendStmt, stream);
        try_parse_node!(Return, stream);

        Err(ParseError::Error("stmt parse failed".to_string()))
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

impl<'a> Parse<'a> for LetIVar<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::Let>()?.pos;
        stream.parse_token::<lex::At>()?;
        let ident = stream.parse_node::<Ident>()?;
        stream.parse_token::<lex::Eq>()?;
        let body = stream.parse_node::<Expr>()?;
        let end_pos = stream.parse_token::<lex::Semicolon>()?.pos;

        Ok(LetIVar {
            ident,
            body,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Return<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::Return>()?.pos;
        let expr = stream.parse_node::<Expr<'a>>()?;
        let end_pos = stream.parse_token::<lex::Semicolon>()?.pos;

        Ok(Return {
            expr,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for MessageSendStmt<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let expr = stream.parse_node::<MessageSend<'a>>()?;
        let start_pos = expr.pos;
        let end_pos = stream.parse_token::<lex::Semicolon>()?.pos;

        Ok(MessageSendStmt {
            expr,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for DefineMethod<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::OBracket>()?.pos;

        let receiver = stream.parse_node::<ClassName>()?;

        stream.parse_specific_ident("def")?;
        stream.parse_token::<lex::Colon>()?;
        let method_name = stream.parse_node::<Selector>()?;

        stream.parse_specific_ident("do")?;
        stream.parse_token::<lex::Colon>()?;

        let block = stream.parse_node::<Block>()?;

        stream.parse_token::<lex::CBracket>()?;

        let end_pos = stream.parse_token::<lex::Semicolon>()?.pos;

        Ok(DefineMethod {
            receiver,
            method_name,
            block,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for DefineClass<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::OBracket>()?.pos;

        stream.parse_specific_class_name("Class")?;
        stream.parse_specific_ident("subclass")?;

        stream.parse_specific_ident("name")?;
        stream.parse_token::<lex::Colon>()?;
        let name = stream.parse_node::<ClassNameSelector>()?;

        stream.parse_specific_ident("fields")?;
        stream.parse_token::<lex::Colon>()?;
        stream.parse_token::<lex::OBracket>()?;
        let fields = stream.parse_many::<Selector>();
        stream.parse_token::<lex::CBracket>()?;

        stream.parse_token::<lex::CBracket>()?;

        let end_pos = stream.parse_token::<lex::Semicolon>()?.pos;

        Ok(DefineClass {
            name,
            fields,
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
        try_parse_node!(ClassNameSelector, stream);
        try_parse_node!(ClassNew, stream);
        try_parse_node!(Local, stream);
        try_parse_node!(IVar, stream);
        try_parse_node!(Selector, stream);
        try_parse_node!(Block, stream);
        try_parse_node!(Digit, stream);
        try_parse_node!(List, stream);
        try_parse_node!(True, stream);
        try_parse_node!(False, stream);
        try_parse_node!(Self_, stream);

        if let Some(inner) = stream.try_parse_node::<MessageSend>() {
            return Ok(Box::new(inner).into());
        }

        Err(ParseError::Error("expr parse failed".to_string()))
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

impl<'a> Parse<'a> for ClassName<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::ClassName { name, pos } = stream.parse_token()?;
        Ok(ClassName(Ident { name, pos: *pos }))
    }
}

impl<'a> Parse<'a> for Local<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let lex::Name { name, pos } = stream.parse_token()?;
        Ok(Local(Ident { name, pos: *pos }))
    }
}

impl<'a> Parse<'a> for IVar<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::At>()?.pos;
        let ident = stream.parse_node::<Ident>()?;
        let end_pos = ident.pos;

        Ok(IVar {
            ident,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Selector<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::Hash>()?.pos;
        let ident = stream.parse_node::<Ident>()?;
        let end_pos = ident.pos;

        Ok(Selector {
            ident,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for ClassNameSelector<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::Hash>()?.pos;
        let class_name = stream.parse_node::<ClassName>()?;
        let end_pos = class_name.0.pos;

        Ok(ClassNameSelector {
            class_name,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
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

impl<'a> Parse<'a> for List<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::OBracket>()?.pos;
        let items = stream.parse_many_delimited::<Expr<'a>, lex::Comma>();
        let end_pos = stream.parse_token::<lex::CBracket>()?.pos;
        Ok(List {
            items,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for MessageSend<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::OBracket>()?.pos;

        let receiver = stream.parse_node::<Expr>()?;
        let msg = stream.parse_node::<Ident>()?;

        let args = stream.parse_many::<Argument>();

        let end_pos = stream.parse_token::<lex::CBracket>()?.pos;

        Ok(MessageSend {
            receiver,
            msg,
            args,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Argument<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let ident = stream.parse_node::<Ident>()?;
        let start_pos = ident.pos;
        stream.parse_token::<lex::Colon>()?;

        let expr = stream.parse_node::<Expr<'a>>()?;
        let end_pos = expr.pos();

        Ok(Argument {
            ident,
            expr,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Block<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::Pipe>()?.pos;
        let parameters = stream.parse_many::<Parameter>();
        stream.parse_token::<lex::Pipe>()?;

        stream.parse_token::<lex::OBrace>()?;
        let body = stream.parse_many::<Stmt>();
        let end_pos = stream.parse_token::<lex::CBrace>()?.pos;

        Ok(Block {
            parameters,
            body,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for Parameter<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let ident = stream.parse_node::<Ident>()?;
        let start_pos = ident.pos;

        let end_pos = stream.parse_token::<lex::Colon>()?.pos;

        Ok(Parameter {
            ident,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
    }
}

impl<'a> Parse<'a> for ClassNew<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let start_pos = stream.parse_token::<lex::OBracket>()?.pos;

        let class_name = stream.parse_node::<ClassName>()?;
        stream.parse_specific_ident("new")?;
        let args = stream.parse_many::<Argument>();

        let end_pos = stream.parse_token::<lex::CBracket>()?.pos;

        Ok(ClassNew {
            class_name,
            args,
            pos: Pos::new(start_pos.from, end_pos.to),
        })
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
