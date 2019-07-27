use crate::prep::ClassVTable;
use crate::{
    ast::{visit_ast, Ast, Visitor, *},
    error::{Error, Result},
};
use std::{collections::HashMap, rc::Rc};

pub type VTable<'a, T> = HashMap<&'a str, T>;

pub fn interpret<'a>(interpreter: &'a mut Interpreter<'a>, ast: &'a Ast<'a>) -> Result<'a, ()> {
    visit_ast(interpreter, ast)?;
    dbg!(&interpreter.locals);

    Ok(())
}

pub struct Interpreter<'a> {
    class_vtable: ClassVTable<'a>,
    locals: VTable<'a, Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(class_vtable: ClassVTable<'a>) -> Self {
        Self {
            class_vtable,
            locals: HashMap::new(),
        }
    }
}

impl<'a> Visitor<'a> for Interpreter<'a> {
    type Error = Error<'a>;

    fn visit_let_local(&mut self, node: &'a LetLocal<'a>) -> Result<'a, ()> {
        let name = &node.ident.name;
        let value = node.body.eval(self)?;
        self.locals.insert(name, value);
        Ok(())
    }
}

#[derive(Debug)]
enum Value {
    Number(i32),
    Ref(Rc<Value>),
}

impl Value {
    fn into_owned(&self) -> Self {
        match self {
            Value::Number(n) => Value::Number(*n),
            Value::Ref(r) => Value::Ref(Rc::clone(r)),
        }
    }
}

trait Eval<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value>;
}

impl<'a> Eval<'a> for Expr<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        match self {
            Expr::Local(inner) => inner.eval(interpreter),
            Expr::Digit(inner) => inner.eval(interpreter),

            Expr::IVar(_) => unimplemented!("eval IVar"),
            Expr::MessageSend(_) => unimplemented!("eval MessageSend"),
            Expr::ClassNew(_) => unimplemented!("eval ClassNew"),
            Expr::Selector(_) => unimplemented!("eval Selector"),
            Expr::ClassNameSelector(_) => unimplemented!("eval ClassNameSelector"),
            Expr::Block(_) => unimplemented!("eval Block"),
            Expr::List(_) => unimplemented!("eval List"),
            Expr::True(_) => unimplemented!("eval True"),
            Expr::False(_) => unimplemented!("eval False"),
            Expr::Self_(_) => unimplemented!("eval Self_"),
        }
    }
}

impl<'a> Eval<'a> for Local<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        let name = self.0.name;
        let value = interpreter.locals.get(name).ok_or_else(|| {
            Error::UndefinedLocal { name, span: self.0.span }
        })?;
        Ok(value.into_owned())
    }
}

impl<'a> Eval<'a> for Digit {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        let digit = self.digit;
        let value = Value::Number(digit);
        Ok(value)
    }
}
