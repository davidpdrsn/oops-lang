mod visitor;

pub use visitor::{Visitor, visit_ast};

use crate::parse::{Parse, ParseStream};
use crate::{
    error::{Error, Result},
    lex, Span,
};

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

pub type Ast<'a> = Vec<Stmt<'a>>;

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
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct LetIVar<'a> {
    pub ident: Ident<'a>,
    pub body: Expr<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DefineMethod<'a> {
    pub class_name: ClassName<'a>,
    pub method_name: Selector<'a>,
    pub block: Block<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MessageSendStmt<'a> {
    pub expr: MessageSend<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Return<'a> {
    pub expr: Expr<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DefineClass<'a> {
    pub name: ClassNameSelector<'a>,
    pub fields: Vec<Selector<'a>>,
    pub span: Span,
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
    Number(Number),
    List(List<'a>),
    True(True),
    False(False),
    Self_(Self_),
}

impl<'a> Expr<'a> {
    pub fn span(&self) -> Span {
        match self {
            Expr::Local(inner) => inner.0.span,
            Expr::IVar(inner) => inner.span,
            Expr::MessageSend(inner) => inner.span,
            Expr::ClassNew(inner) => inner.span,
            Expr::Selector(inner) => inner.span,
            Expr::ClassNameSelector(inner) => inner.span,
            Expr::Block(inner) => inner.span,
            Expr::Number(inner) => inner.span,
            Expr::List(inner) => inner.span,
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
impl_into!(Expr, Number);
impl_into!(Expr, List<'a>);
impl_into!(Expr, True);
impl_into!(Expr, False);
impl_into!(Expr, Self_);

impl<'a> From<Box<MessageSend<'a>>> for Expr<'a> {
    fn from(inner: Box<MessageSend<'a>>) -> Expr<'a> {
        Expr::MessageSend(inner)
    }
}

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct ClassName<'a>(pub Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct Local<'a>(pub Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct IVar<'a> {
    pub ident: Ident<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Number {
    pub number: i32,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct List<'a> {
    pub items: Vec<Expr<'a>>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct True(pub Span);

#[derive(Eq, PartialEq, Debug)]
pub struct False(pub Span);

#[derive(Eq, PartialEq, Debug)]
pub struct Self_(pub Span);

#[derive(Eq, PartialEq, Debug)]
pub struct Selector<'a> {
    pub ident: Ident<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ClassNameSelector<'a> {
    pub class_name: ClassName<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Block<'a> {
    pub parameters: Vec<Parameter<'a>>,
    pub body: Vec<Stmt<'a>>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Parameter<'a> {
    pub ident: Ident<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MessageSend<'a> {
    pub receiver: Expr<'a>,
    pub msg: Ident<'a>,
    pub args: Vec<Argument<'a>>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Argument<'a> {
    pub ident: Ident<'a>,
    pub expr: Expr<'a>,
    pub span: Span,
}

#[derive(Eq, PartialEq, Debug)]
pub struct ClassNew<'a> {
    pub class_name: ClassName<'a>,
    pub args: Vec<Argument<'a>>,
    pub span: Span,
}

//
// Misc
//

#[derive(Eq, PartialEq, Debug, Hash)]
pub struct Ident<'a> {
    pub name: &'a str,
    pub span: Span,
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
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        try_parse_node!(DefineClass, stream);
        try_parse_node!(DefineMethod, stream);
        try_parse_node!(LetLocal, stream);
        try_parse_node!(LetIVar, stream);
        try_parse_node!(MessageSendStmt, stream);
        try_parse_node!(Return, stream);

        Err(Error::ParseError("stmt parse failed".to_string()))
    }
}

impl<'a> Parse<'a> for LetLocal<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::Let>()?.span;
        let ident = stream.parse_node::<Ident>()?;
        stream.parse_token::<lex::Eq>()?;
        let body = stream.parse_node::<Expr>()?;
        let end = stream.parse_token::<lex::Semicolon>()?.span;

        Ok(LetLocal {
            ident,
            body,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for LetIVar<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::Let>()?.span;
        stream.parse_token::<lex::At>()?;
        let ident = stream.parse_node::<Ident>()?;
        stream.parse_token::<lex::Eq>()?;
        let body = stream.parse_node::<Expr>()?;
        let end = stream.parse_token::<lex::Semicolon>()?.span;

        Ok(LetIVar {
            ident,
            body,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for Return<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::Return>()?.span;
        let expr = stream.parse_node::<Expr<'a>>()?;
        let end = stream.parse_token::<lex::Semicolon>()?.span;

        Ok(Return {
            expr,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for MessageSendStmt<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let expr = stream.parse_node::<MessageSend<'a>>()?;
        let start = expr.span;
        let end = stream.parse_token::<lex::Semicolon>()?.span;

        Ok(MessageSendStmt {
            expr,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for DefineMethod<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::OBracket>()?.span;

        let class_name = stream.parse_node::<ClassName>()?;

        stream.parse_specific_ident("def")?;
        stream.parse_token::<lex::Colon>()?;
        let method_name = stream.parse_node::<Selector>()?;

        stream.parse_specific_ident("do")?;
        stream.parse_token::<lex::Colon>()?;

        let block = stream.parse_node::<Block>()?;

        stream.parse_token::<lex::CBracket>()?;

        let end = stream.parse_token::<lex::Semicolon>()?.span;

        Ok(DefineMethod {
            class_name,
            method_name,
            block,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for DefineClass<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::OBracket>()?.span;

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

        let end = stream.parse_token::<lex::Semicolon>()?.span;

        Ok(DefineClass {
            name,
            fields,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for Ident<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::Name { name, span } = stream.parse_token()?;
        let ident = Ident { name, span: *span };
        Ok(ident)
    }
}

impl<'a> Parse<'a> for Expr<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        try_parse_node!(ClassNameSelector, stream);
        try_parse_node!(ClassNew, stream);
        try_parse_node!(Local, stream);
        try_parse_node!(IVar, stream);
        try_parse_node!(Selector, stream);
        try_parse_node!(Block, stream);
        try_parse_node!(Number, stream);
        try_parse_node!(List, stream);
        try_parse_node!(True, stream);
        try_parse_node!(False, stream);
        try_parse_node!(Self_, stream);

        if let Some(inner) = stream.try_parse_node::<MessageSend>() {
            return Ok(Box::new(inner).into());
        }

        Err(Error::ParseError("expr parse failed".to_string()))
    }
}

impl<'a> Parse<'a> for Number {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::Number { number, span } = stream.parse_token()?;
        Ok(Number {
            number: *number,
            span: *span,
        })
    }
}

impl<'a> Parse<'a> for ClassName<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::ClassName { name, span } = stream.parse_token()?;
        Ok(ClassName(Ident { name, span: *span }))
    }
}

impl<'a> Parse<'a> for Local<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::Name { name, span } = stream.parse_token()?;
        Ok(Local(Ident { name, span: *span }))
    }
}

impl<'a> Parse<'a> for IVar<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::At>()?.span;
        let ident = stream.parse_node::<Ident>()?;
        let end = ident.span;

        Ok(IVar {
            ident,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for Selector<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::Hash>()?.span;
        let ident = stream.parse_node::<Ident>()?;
        let end = ident.span;

        Ok(Selector {
            ident,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for ClassNameSelector<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::Hash>()?.span;
        let class_name = stream.parse_node::<ClassName>()?;
        let end = class_name.0.span;

        Ok(ClassNameSelector {
            class_name,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for True {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::True { span } = stream.parse_token()?;
        Ok(True(*span))
    }
}

impl<'a> Parse<'a> for False {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::False { span } = stream.parse_token()?;
        Ok(False(*span))
    }
}

impl<'a> Parse<'a> for Self_ {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let lex::Self_ { span } = stream.parse_token()?;
        Ok(Self_(*span))
    }
}

impl<'a> Parse<'a> for List<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::OBracket>()?.span;
        let items = stream.parse_many_delimited::<Expr<'a>, lex::Comma>();
        let end = stream.parse_token::<lex::CBracket>()?.span;
        Ok(List {
            items,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for MessageSend<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::OBracket>()?.span;

        let receiver = stream.parse_node::<Expr>()?;
        let msg = stream.parse_node::<Ident>()?;

        let args = stream.parse_many::<Argument>();

        let end = stream.parse_token::<lex::CBracket>()?.span;

        Ok(MessageSend {
            receiver,
            msg,
            args,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for Argument<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let ident = stream.parse_node::<Ident>()?;
        let start = ident.span;
        stream.parse_token::<lex::Colon>()?;

        let expr = stream.parse_node::<Expr<'a>>()?;
        let end = expr.span();

        Ok(Argument {
            ident,
            expr,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for Block<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::Pipe>()?.span;
        let parameters = stream.parse_many::<Parameter>();
        stream.parse_token::<lex::Pipe>()?;

        stream.parse_token::<lex::OBrace>()?;
        let body = stream.parse_many::<Stmt>();
        let end = stream.parse_token::<lex::CBrace>()?.span;

        Ok(Block {
            parameters,
            body,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for Parameter<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let ident = stream.parse_node::<Ident>()?;
        let start = ident.span;

        let end = stream.parse_token::<lex::Colon>()?.span;

        Ok(Parameter {
            ident,
            span: Span::new(start.from, end.to),
        })
    }
}

impl<'a> Parse<'a> for ClassNew<'a> {
    fn parse(stream: &mut ParseStream<'a>) -> Result<'a, Self> {
        let start = stream.parse_token::<lex::OBracket>()?.span;

        let class_name = stream.parse_node::<ClassName>()?;
        stream.parse_specific_ident("new")?;
        let args = stream.parse_many::<Argument>();

        let end = stream.parse_token::<lex::CBracket>()?.span;

        Ok(ClassNew {
            class_name,
            args,
            span: Span::new(start.from, end.to),
        })
    }
}
