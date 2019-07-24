use crate::Pos;

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
