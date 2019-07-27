#![allow(dead_code)]

use std::result::Result;
use crate::ast::*;

pub trait Visitor<'a> {
    type Error;

    fn visit_ast(&mut self, _: &'a Ast<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_stmt(&mut self, _: &'a Stmt<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_let_local(&mut self, _: &'a LetLocal<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_let_ivar(&mut self, _: &'a LetIVar<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_message_send_stmt(&mut self, _: &'a MessageSendStmt<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_return(&mut self, _: &'a Return<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_define_method(&mut self, _: &'a DefineMethod<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_define_class(&mut self, _: &'a DefineClass<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_expr(&mut self, _: &'a Expr<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_local(&mut self, _: &'a Local<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_ivar(&mut self, _: &'a IVar<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_message_send(&mut self, _: &'a MessageSend<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_class_new(&mut self, _: &'a ClassNew<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_class_name_selector(
        &mut self,
        _: &'a ClassNameSelector<'a>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_block(&mut self, _: &'a Block<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_number(&mut self, _: &'a Number) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_list(&mut self, _: &'a List<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_true(&mut self, _: &'a True) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_false(&mut self, _: &'a False) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_self(&mut self, _: &'a Self_) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn visit_ast<'a, V: Visitor<'a>>(v: &mut V, node: &'a Ast<'a>) -> Result<(), V::Error> {
    v.visit_ast(node)?;

    for stmt in node {
        visit_stmt(v, stmt)?;
    }

    Ok(())
}

fn visit_stmt<'a, V: Visitor<'a>>(v: &mut V, node: &'a Stmt<'a>) -> Result<(), V::Error> {
    v.visit_stmt(node)?;

    match node {
        Stmt::LetLocal(inner) => visit_let_local(v, inner)?,
        Stmt::LetIVar(inner) => visit_let_ivar(v, inner)?,
        Stmt::MessageSend(inner) => visit_message_send_stmt(v, inner)?,
        Stmt::Return(inner) => visit_return(v, inner)?,
        Stmt::DefineMethod(inner) => visit_define_method(v, inner)?,
        Stmt::DefineClass(inner) => visit_define_class(v, inner)?,
    }

    Ok(())
}

fn visit_let_local<'a, V: Visitor<'a>>(v: &mut V, node: &'a LetLocal<'a>) -> Result<(), V::Error> {
    v.visit_let_local(node)
}

fn visit_let_ivar<'a, V: Visitor<'a>>(v: &mut V, node: &'a LetIVar<'a>) -> Result<(), V::Error> {
    v.visit_let_ivar(node)
}

fn visit_message_send_stmt<'a, V: Visitor<'a>>(
    v: &mut V,
    node: &'a MessageSendStmt<'a>,
) -> Result<(), V::Error> {
    v.visit_message_send_stmt(node)
}

fn visit_return<'a, V: Visitor<'a>>(v: &mut V, node: &'a Return<'a>) -> Result<(), V::Error> {
    v.visit_return(node)
}

fn visit_define_method<'a, V: Visitor<'a>>(
    v: &mut V,
    node: &'a DefineMethod<'a>,
) -> Result<(), V::Error> {
    v.visit_define_method(node)
}

fn visit_define_class<'a, V: Visitor<'a>>(
    v: &mut V,
    node: &'a DefineClass<'a>,
) -> Result<(), V::Error> {
    v.visit_define_class(node)
}

fn visit_expr<'a, V: Visitor<'a>>(v: &mut V, node: &'a Expr<'a>) -> Result<(), V::Error> {
    v.visit_expr(node)?;

    match node {
        Expr::Local(inner) => visit_local(v, inner)?,
        Expr::IVar(inner) => visit_ivar(v, inner)?,
        Expr::MessageSend(inner) => visit_message_send(v, inner)?,
        Expr::ClassNew(inner) => visit_class_new(v, inner)?,
        Expr::ClassNameSelector(inner) => visit_class_name_selector(v, inner)?,
        Expr::Block(inner) => visit_block(v, inner)?,
        Expr::Number(inner) => visit_number(v, inner)?,
        Expr::List(inner) => visit_list(v, inner)?,
        Expr::True(inner) => visit_true(v, inner)?,
        Expr::False(inner) => visit_false(v, inner)?,
        Expr::Self_(inner) => visit_self(v, inner)?,
    }

    Ok(())
}

fn visit_local<'a, V: Visitor<'a>>(v: &mut V, node: &'a Local<'a>) -> Result<(), V::Error> {
    v.visit_local(node)
}

fn visit_ivar<'a, V: Visitor<'a>>(v: &mut V, node: &'a IVar<'a>) -> Result<(), V::Error> {
    v.visit_ivar(node)
}

fn visit_message_send<'a, V: Visitor<'a>>(
    v: &mut V,
    node: &'a MessageSend<'a>,
) -> Result<(), V::Error> {
    v.visit_message_send(node)
}

fn visit_class_new<'a, V: Visitor<'a>>(v: &mut V, node: &'a ClassNew<'a>) -> Result<(), V::Error> {
    v.visit_class_new(node)
}

fn visit_class_name_selector<'a, V: Visitor<'a>>(
    v: &mut V,
    node: &'a ClassNameSelector<'a>,
) -> Result<(), V::Error> {
    v.visit_class_name_selector(node)
}

fn visit_block<'a, V: Visitor<'a>>(v: &mut V, node: &'a Block<'a>) -> Result<(), V::Error> {
    v.visit_block(node)
}

fn visit_number<'a, V: Visitor<'a>>(v: &mut V, node: &'a Number) -> Result<(), V::Error> {
    v.visit_number(node)
}

fn visit_list<'a, V: Visitor<'a>>(v: &mut V, node: &'a List<'a>) -> Result<(), V::Error> {
    v.visit_list(node)
}

fn visit_true<'a, V: Visitor<'a>>(v: &mut V, node: &'a True) -> Result<(), V::Error> {
    v.visit_true(node)
}

fn visit_false<'a, V: Visitor<'a>>(v: &mut V, node: &'a False) -> Result<(), V::Error> {
    v.visit_false(node)
}

fn visit_self<'a, V: Visitor<'a>>(v: &mut V, node: &'a Self_) -> Result<(), V::Error> {
    v.visit_self(node)
}
