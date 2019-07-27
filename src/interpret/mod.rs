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
    True,
    False,
    List(Rc<Vec<Value>>),
}

impl Value {
    fn to_owned(&self) -> Self {
        match self {
            Value::Number(n) => Value::Number(*n),
            Value::List(values) => Value::List(Rc::clone(values)),
            Value::True => Value::True,
            Value::False => Value::False,
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
            Expr::Number(inner) => inner.eval(interpreter),
            Expr::List(inner) => inner.eval(interpreter),
            Expr::True(inner) => inner.eval(interpreter),
            Expr::False(inner) => inner.eval(interpreter),

            Expr::IVar(_) => unimplemented!("eval IVar"),
            Expr::MessageSend(_) => unimplemented!("eval MessageSend"),
            Expr::ClassNew(_) => unimplemented!("eval ClassNew"),
            Expr::Selector(_) => unimplemented!("eval Selector"),
            Expr::ClassNameSelector(_) => unimplemented!("eval ClassNameSelector"),
            Expr::Block(_) => unimplemented!("eval Block"),
            Expr::Self_(_) => unimplemented!("eval Self_"),
        }
    }
}

impl<'a> Eval<'a> for Local<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        let name = self.0.name;
        let value = interpreter
            .locals
            .get(name)
            .ok_or_else(|| Error::UndefinedLocal {
                name,
                span: self.0.span,
            })?;
        Ok(value.to_owned())
    }
}

impl<'a> Eval<'a> for Number {
    fn eval(&self, _: &Interpreter<'a>) -> Result<'a, Value> {
        let number = self.number;
        Ok(Value::Number(number))
    }
}

impl<'a> Eval<'a> for List<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        let items = &self.items;
        let values: Result<'a, Vec<Value>> =
            items
                .iter()
                .try_fold(Vec::with_capacity(items.len()), |mut acc, expr| {
                    let value = expr.eval(interpreter)?;
                    acc.push(value);
                    Ok(acc)
                });
        let values = values?;
        Ok(Value::List(Rc::new(values)))
    }
}

impl<'a> Eval<'a> for True {
    fn eval(&self, _: &Interpreter<'a>) -> Result<'a, Value> {
        Ok(Value::True)
    }
}

impl<'a> Eval<'a> for False {
    fn eval(&self, _: &Interpreter<'a>) -> Result<'a, Value> {
        Ok(Value::False)
    }
}
