use crate::prep::ClassVTable;
use crate::{
    ast::{visit_ast, Ast, Visitor, *},
    error::{Error, Result},
};
use std::collections::HashMap;

pub type VTable<'a, T> = HashMap<&'a str, T>;

pub fn interpret<'a>(mut interpreter: &'a mut Interpreter<'a>, ast: &'a Ast<'a>) -> Result<'a, ()> {
    visit_ast(&mut interpreter, ast)?;
    dbg!(&interpreter.locals);

    Ok(())
}

#[derive(Debug)]
enum Value<'a> {
    Nil,
    Rest(&'a Value<'a>),
}

pub struct Interpreter<'a> {
    class_vtable: ClassVTable<'a>,
    locals: VTable<'a, Value<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(class_vtable: ClassVTable<'a>) -> Self {
        Self {
            class_vtable,
            locals: HashMap::new(),
        }
    }
}

impl<'a> Visitor<'a> for &'a mut Interpreter<'a> {
    type Error = Error<'a>;

    fn visit_let_local(&mut self, node: &'a LetLocal<'a>) -> Result<'a, ()> {
        let name = &node.ident.name;
        let value = node.body.eval(self)?;
        self.locals.insert(name, value);
        Ok(())
    }
}

trait Eval<'a> {
    fn eval(&self, interpreter: &'a Interpreter<'a>) -> Result<'a, Value<'a>>;
}

impl<'a> Eval<'a> for Expr<'a> {
    fn eval(&self, interpreter: &'a Interpreter<'a>) -> Result<'a, Value<'a>> {
        match self {
            Expr::Local(inner) => inner.eval(interpreter),
            _ => unimplemented!(),
        }
    }
}

impl<'a> Eval<'a> for Local<'a> {
    fn eval(&self, interpreter: &'a Interpreter<'a>) -> Result<'a, Value<'a>> {
        unimplemented!()
        // let name = self.0.name;
        // let value = interpreter.locals.get(name).unwrap();
        // Ok(value)
    }
}
