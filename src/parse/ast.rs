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

#[derive(Eq, PartialEq, Debug)]
pub enum Stmt<'a> {
    LetLocal(LetLocal<'a>),
    LetIVar(LetIVar<'a>),
    MessageSend(MessageSend<'a>),
}

impl_into!(Stmt, LetLocal<'a>);
impl_into!(Stmt, LetIVar<'a>);
impl_into!(Stmt, MessageSend<'a>);

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
pub struct Ident<'a> {
    pub name: &'a str,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Expr<'a> {
    Local(Local<'a>),
    IVar(IVar<'a>),
    MessageSend(Box<MessageSend<'a>>),
    Selector(Selector<'a>),
    Block(Block<'a>),
    Digit(Digit),
    List(List<'a>),
    True(True),
    False(False),
    Self_(Self_),
}

impl_into!(Expr, Local<'a>);
impl_into!(Expr, IVar<'a>);
impl_into!(Expr, Selector<'a>);
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
pub struct Local<'a>(pub Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct IVar<'a>(pub Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct Digit {
    pub digit: i32,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct List<'a>(pub Vec<Expr<'a>>);

#[derive(Eq, PartialEq, Debug)]
pub struct True(pub Pos);

#[derive(Eq, PartialEq, Debug)]
pub struct False(pub Pos);

#[derive(Eq, PartialEq, Debug)]
pub struct Self_(pub Pos);

#[derive(Eq, PartialEq, Debug)]
pub struct Selector<'a>(Ident<'a>);

#[derive(Eq, PartialEq, Debug)]
pub struct Block<'a> {
    pub args: Arguments<'a>,
    pub body: Vec<Stmt<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct DefineMethod<'a> {
    pub rec: Expr<'a>,
    pub method_name: Selector<'a>,
    pub block: Block<'a>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MessageSend<'a> {
    pub rec: Expr<'a>,
    pub msg: Ident<'a>,
    pub args: Arguments<'a>,
    pub pos: Pos,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Arguments<'a> {
    values: Vec<(Ident<'a>, Expr<'a>)>,
}
