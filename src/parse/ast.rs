use crate::Pos;

#[derive(Debug)]
pub struct Ident<'a> {
    pub name: &'a str,
    pub pos: Pos,
}

#[derive(Debug)]
pub enum Statement<'a> {
    LetLocal { ident: Ident<'a>, body: Expr<'a> },
    LetIVar { ident: Ident<'a>, body: Expr<'a> },
    MessageSend(MessageSend<'a>),
}

#[derive(Debug)]
pub enum Expr<'a> {
    Local(Ident<'a>),
    IVar(Ident<'a>),
    MessageSend(Box<MessageSend<'a>>),
    Selector(Selector<'a>),
    Block(Block<'a>),
    Digit((i32, Pos)),
    List(Vec<Expr<'a>>),
    True(Pos),
    False(Pos),
    Self_(Pos),
}

#[derive(Debug)]
pub struct Selector<'a>(Ident<'a>);

#[derive(Debug)]
pub struct Block<'a> {
    args: Vec<(Ident<'a>, Expr<'a>)>,
    body: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub struct DefineMethod<'a> {
    pub rec: Expr<'a>,
    pub method_name: Selector<'a>,
    pub block: Block<'a>,
}

#[derive(Debug)]
pub struct MessageSend<'a> {
    pub rec: Expr<'a>,
    pub msg: Ident<'a>,
    pub args: Vec<(Ident<'a>, Expr<'a>)>,
}
