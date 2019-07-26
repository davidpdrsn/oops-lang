mod class;
mod visitor;

use self::class::{find_classes_and_methods, ClassVTable};
use crate::{
    error::{Error, Result},
    parse::{self as ast, Ast},
};
use std::{collections::HashMap, fmt};
use visitor::{visit_ast, Visitor};

type VTable<'a, T> = HashMap<&'a str, T>;

pub fn interpret<'a>(ast: &'a Ast<'a>) -> Result<'a, ()> {
    let class_vtable = find_classes_and_methods(ast)?;

    let mut interpreter = Interpreter {
        class_vtable,
        locals: HashMap::new(),
    };

    visit_ast(&mut interpreter, ast)?;

    dbg!(&interpreter.locals);

    Ok(())
}

#[derive(Debug)]
enum Value {
    Nil,
}

struct Interpreter<'a> {
    class_vtable: ClassVTable<'a>,
    locals: VTable<'a, Value>,
}

impl<'a> Visitor<'a> for Interpreter<'a> {
    type Error = Error<'a>;

    fn visit_let_local(&mut self, node: &'a ast::LetLocal<'a>) -> Result<'a, ()> {
        let name = &node.ident.name;
        let value = node.body.eval(&self)?;
        self.locals.insert(name, value);
        Ok(())
    }

    fn visit_let_ivar(&mut self, _: &'a ast::LetIVar<'a>) -> Result<'a, ()> {
        unimplemented!("visit_let_ivar");
        Ok(())
    }

    fn visit_message_send_stmt(&mut self, _: &'a ast::MessageSendStmt<'a>) -> Result<'a, ()> {
        unimplemented!("visit_message_send_stmt");
        Ok(())
    }

    fn visit_return(&mut self, _: &'a ast::Return<'a>) -> Result<'a, ()> {
        unimplemented!("visit_return");
        Ok(())
    }
}

trait Eval<'a> {
    // Should this method return owned Values or borrowed?
    // Evaling a `Local` has to be a reference, otherwise we have to clone it
    // However `Digit` has to be owned, otherwise we'll return a reference to data owned by the
    // current function...
    //
    // Do we have to give the values to the interpreter and then get references back?
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value>;
}

impl<'a> Eval<'a> for ast::Expr<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        match self {
            ast::Expr::Local(inner) => inner.eval(interpreter),
            _ => unimplemented!(),
            // ast::Expr::IVar(inner) => inner.eval(interpreter),
            // ast::Expr::MessageSend(inner) => inner.eval(interpreter),
            // ast::Expr::ClassNew(inner) => inner.eval(interpreter),
            // ast::Expr::Selector(inner) => inner.eval(interpreter),
            // ast::Expr::ClassNameSelector(inner) => inner.eval(interpreter),
            // ast::Expr::Block(inner) => inner.eval(interpreter),
            // ast::Expr::Digit(inner) => inner.eval(interpreter),
            // ast::Expr::List(inner) => inner.eval(interpreter),
            // ast::Expr::True(inner) => inner.eval(interpreter),
            // ast::Expr::False(inner) => inner.eval(interpreter),
            // ast::Expr::Self_(inner) => inner.eval(interpreter),
        }
    }
}

impl<'a> Eval<'a> for ast::Local<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value> {
        let name = self.0.name;
        let value = interpreter.locals.get(name).unwrap();
        Ok(value)
    }
}
