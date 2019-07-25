#![allow(dead_code)]

use crate::parse::*;

pub trait Visitor<'a> {
    fn visit_ast(&mut self, _: &'a Ast<'a>) {}

    fn visit_stmt(&mut self, _: &'a Stmt<'a>) {}

    fn visit_let_local(&mut self, _: &'a LetLocal<'a>) {}

    fn visit_let_ivar(&mut self, _: &'a LetIVar<'a>) {}

    fn visit_message_send_stmt(&mut self, _: &'a MessageSendStmt<'a>) {}

    fn visit_return(&mut self, _: &'a Return<'a>) {}

    fn visit_define_method(&mut self, _: &'a DefineMethod<'a>) {}

    fn visit_define_class(&mut self, _: &'a DefineClass<'a>) {}

    fn visit_expr(&mut self, _: &'a Expr<'a>) {}

    fn visit_local(&mut self, _: &'a Local<'a>) {}

    fn visit_ivar(&mut self, _: &'a IVar<'a>) {}

    fn visit_message_send(&mut self, _: &'a MessageSend<'a>) {}

    fn visit_class_new(&mut self, _: &'a ClassNew<'a>) {}

    fn visit_selector(&mut self, _: &'a Selector<'a>) {}

    fn visit_class_name_selector(&mut self, _: &'a ClassNameSelector<'a>) {}

    fn visit_block(&mut self, _: &'a Block<'a>) {}

    fn visit_digit(&mut self, _: &'a Digit) {}

    fn visit_list(&mut self, _: &'a List<'a>) {}

    fn visit_true(&mut self, _: &'a True) {}

    fn visit_false(&mut self, _: &'a False) {}

    fn visit_self(&mut self, _: &'a Self_) {}
}

pub fn visit_ast<'a, V: Visitor<'a>>(v: &mut V, node: &'a Ast<'a>) {
    v.visit_ast(node);

    for stmt in node {
        visit_stmt(v, stmt);
    }
}

fn visit_stmt<'a, V: Visitor<'a>>(v: &mut V, node: &'a Stmt<'a>) {
    v.visit_stmt(node);

    match node {
        Stmt::LetLocal(inner) => visit_let_local(v, inner),
        Stmt::LetIVar(inner) => visit_let_ivar(v, inner),
        Stmt::MessageSend(inner) => visit_message_send_stmt(v, inner),
        Stmt::Return(inner) => visit_return(v, inner),
        Stmt::DefineMethod(inner) => visit_define_method(v, inner),
        Stmt::DefineClass(inner) => visit_define_class(v, inner),
    }
}

fn visit_let_local<'a, V: Visitor<'a>>(v: &mut V, node: &'a LetLocal<'a>) {
    v.visit_let_local(node)
}

fn visit_let_ivar<'a, V: Visitor<'a>>(v: &mut V, node: &'a LetIVar<'a>) {
    v.visit_let_ivar(node)
}

fn visit_message_send_stmt<'a, V: Visitor<'a>>(v: &mut V, node: &'a MessageSendStmt<'a>) {
    v.visit_message_send_stmt(node)
}

fn visit_return<'a, V: Visitor<'a>>(v: &mut V, node: &'a Return<'a>) {
    v.visit_return(node)
}

fn visit_define_method<'a, V: Visitor<'a>>(v: &mut V, node: &'a DefineMethod<'a>) {
    v.visit_define_method(node)
}

fn visit_define_class<'a, V: Visitor<'a>>(v: &mut V, node: &'a DefineClass<'a>) {
    v.visit_define_class(node)
}

fn visit_expr<'a, V: Visitor<'a>>(v: &mut V, node: &'a Expr<'a>) {
    v.visit_expr(node);

    match node {
        Expr::Local(inner) => visit_local(v, inner),
        Expr::IVar(inner) => visit_ivar(v, inner),
        Expr::MessageSend(inner) => visit_message_send(v, inner),
        Expr::ClassNew(inner) => visit_class_new(v, inner),
        Expr::Selector(inner) => visit_selector(v, inner),
        Expr::ClassNameSelector(inner) => visit_class_name_selector(v, inner),
        Expr::Block(inner) => visit_block(v, inner),
        Expr::Digit(inner) => visit_digit(v, inner),
        Expr::List(inner) => visit_list(v, inner),
        Expr::True(inner) => visit_true(v, inner),
        Expr::False(inner) => visit_false(v, inner),
        Expr::Self_(inner) => visit_self(v, inner),
    }
}

fn visit_local<'a, V: Visitor<'a>>(v: &mut V, node: &'a Local<'a>) {
    v.visit_local(node)
}

fn visit_ivar<'a, V: Visitor<'a>>(v: &mut V, node: &'a IVar<'a>) {
    v.visit_ivar(node)
}

fn visit_message_send<'a, V: Visitor<'a>>(v: &mut V, node: &'a MessageSend<'a>) {
    v.visit_message_send(node)
}

fn visit_class_new<'a, V: Visitor<'a>>(v: &mut V, node: &'a ClassNew<'a>) {
    v.visit_class_new(node)
}

fn visit_selector<'a, V: Visitor<'a>>(v: &mut V, node: &'a Selector<'a>) {
    v.visit_selector(node)
}

fn visit_class_name_selector<'a, V: Visitor<'a>>(v: &mut V, node: &'a ClassNameSelector<'a>) {
    v.visit_class_name_selector(node)
}

fn visit_block<'a, V: Visitor<'a>>(v: &mut V, node: &'a Block<'a>) {
    v.visit_block(node)
}

fn visit_digit<'a, V: Visitor<'a>>(v: &mut V, node: &'a Digit) {
    v.visit_digit(node)
}

fn visit_list<'a, V: Visitor<'a>>(v: &mut V, node: &'a List<'a>) {
    v.visit_list(node)
}

fn visit_true<'a, V: Visitor<'a>>(v: &mut V, node: &'a True) {
    v.visit_true(node)
}

fn visit_false<'a, V: Visitor<'a>>(v: &mut V, node: &'a False) {
    v.visit_false(node)
}

fn visit_self<'a, V: Visitor<'a>>(v: &mut V, node: &'a Self_) {
    v.visit_self(node)
}
