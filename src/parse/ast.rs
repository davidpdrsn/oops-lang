use super::parse_stream::{Parse, ParseError, ParseStream};
use crate::{lex, Pos};

macro_rules! impl_into {
    ( $into:ident, $variant:ident, $name:ident<'a> ) => {
        impl<'a> From<$name<'a>> for $into<'a> {
            fn from(inner: $name<'a>) -> $into<'a> {
                $into::$variant(inner)
            }
        }
    };

    ( $into:ident, $name:ident<'a> ) => {
        impl<'a> From<$name<'a>> for $into<'a> {
            fn from(inner: $name<'a>) -> $into<'a> {
                $into::$name(inner)
            }
        }
    };

    ( $into:ident, $name:ident ) => {
        impl<'a> From<$name> for $into<'a> {
            fn from(inner: $name) -> $into<'a> {
                $into::$name(inner)
            }
        }
    };
}

//
// Statements
//

#[derive(Eq, PartialEq, Debug)]
pub enum Stmt<'a> {
    LetLocal(LetLocal<'a>),
    LetIVar(LetIVar<'a>),
    MessageSend(MessageSendStmt<'a>),
    Return(Return<'a>),
    DefineMethod(DefineMethod<'a>),
    DefineClass(DefineClass<'a>),
}

impl_into!(Stmt, LetLocal<'a>);
impl_into!(Stmt, LetIVar<'a>);
impl_into!(Stmt, MessageSend, MessageSendStmt<'a>);
impl_into!(Stmt, Return<'a>);
impl_into!(Stmt, DefineMethod<'a>);
impl_into!(Stmt, DefineClass<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct LetLocal<'a> {
    pub ident: Ident<'a>,
    pub body: Expr<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LetIVar<'a> {
    pub ident: Ident<'a>,
    pub body: Expr<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DefineMethod<'a> {
    pub receiver: ClassName<'a>,
    pub method_name: Selector<'a>,
    pub block: Block<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MessageSendStmt<'a> {
    pub expr: MessageSend<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Return<'a> {
    pub expr: Expr<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DefineClass<'a> {
    pub name: ClassNameSelector<'a>,
    pub fields: Vec<Selector<'a>>,
    pub pos: Pos,
}

//
// Expressions
//

#[derive(Eq, PartialEq, Debug)]
pub enum Expr<'a> {
    Local(Local<'a>),
    IVar(IVar<'a>),
    MessageSend(Box<MessageSend<'a>>),
    ClassNew(ClassNew<'a>),
    Selector(Selector<'a>),
    ClassNameSelector(ClassNameSelector<'a>),
    Block(Block<'a>),
    Digit(Digit),
    List(List<'a>),
    True(True),
    False(False),
    Self_(Self_),
}

impl<'a> Expr<'a> {
    pub fn pos(&self) -> Pos {
        match self {
            Expr::Local(inner) => inner.0.pos,
            Expr::IVar(inner) => inner.pos,
            Expr::MessageSend(inner) => inner.pos,
            Expr::ClassNew(inner) => inner.pos,
            Expr::Selector(inner) => inner.pos,
            Expr::ClassNameSelector(inner) => inner.pos,
            Expr::Block(inner) => inner.pos,
            Expr::Digit(inner) => inner.pos,
            Expr::List(inner) => inner.pos,
            Expr::True(inner) => inner.0,
            Expr::False(inner) => inner.0,
            Expr::Self_(inner) => inner.0,
        }
    }
}

impl_into!(Expr, Local<'a>);
impl_into!(Expr, IVar<'a>);
impl_into!(Expr, Selector<'a>);
impl_into!(Expr, ClassNameSelector<'a>);
impl_into!(Expr, ClassNew<'a>);
impl_into!(Expr, Block<'a>);
impl_into!(Expr, Digit);
impl_into!(Expr, List<'a>);
impl_into!(Expr, True);
impl_into!(Expr, False);
impl_into!(Expr, Self_);

impl<'a> From<Box<MessageSend<'a>>> for Expr<'a> {
    fn from(inner: Box<MessageSend<'a>>) -> Expr<'a> {
        Expr::MessageSend(inner)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ClassName<'a>(pub Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct Local<'a>(pub Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct IVar<'a> {
    pub ident: Ident<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Digit {
    pub digit: i32,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct List<'a> {
    pub items: Vec<Expr<'a>>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct True(pub Pos);

#[derive(Eq, PartialEq, Debug)]
pub struct False(pub Pos);

#[derive(Eq, PartialEq, Debug)]
pub struct Self_(pub Pos);

#[derive(Eq, PartialEq, Debug)]
pub struct Selector<'a> {
    pub ident: Ident<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ClassNameSelector<'a> {
    pub class_name: ClassName<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Block<'a> {
    pub parameters: Vec<Parameter<'a>>,
    pub body: Vec<Stmt<'a>>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Parameter<'a> {
    pub ident: Ident<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MessageSend<'a> {
    pub receiver: Expr<'a>,
    pub msg: Ident<'a>,
    pub args: Vec<Argument<'a>>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Argument<'a> {
    pub ident: Ident<'a>,
    pub expr: Expr<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ClassNew<'a> {
    pub class_name: ClassName<'a>,
    pub args: Vec<Argument<'a>>,
    pub pos: Pos,
}

//
// Misc
//

#[derive(Eq, PartialEq, Debug)]
pub struct Ident<'a> {
    pub name: &'a str,
    pub pos: Pos,
}

//
// Parse impls
//

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
