use super::{
    visitor::{visit_ast, Visitor},
    Ast, VTable,
};
use crate::parse::{self as ast, Ident};
use std::collections::HashMap;

pub fn find_classes_and_methods<'a>(ast: &'a Ast<'a>) -> ClassVTable<'a> {
    let class_vtable = find_classes(ast);
    find_methods(ast, class_vtable)
}

fn find_classes<'a>(ast: &'a Ast<'a>) -> ClassVTable<'a> {
    let mut f = FindClasses::default();
    visit_ast(&mut f, ast);
    ClassVTable { table: f.table }
}

#[derive(Default)]
struct FindClasses<'a> {
    table: VTable<'a, Class<'a>>,
}

impl<'a> Visitor<'a> for FindClasses<'a> {
    fn visit_define_class(&mut self, node: &'a ast::DefineClass<'a>) {
        let name = &node.name.class_name.0;

        let fields = node
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let field = Field { name: ident };
                (ident.name, field)
            })
            .collect::<HashMap<_, _>>();

        let class = Class {
            name,
            fields,
            methods: VTable::new(),
        };

        self.table.insert(name.name, class);
    }
}

struct FindMethods<'a> {
    class_vtable: ClassVTable<'a>,
}

fn find_methods<'a>(ast: &'a Ast<'a>, class_vtable: ClassVTable<'a>) -> ClassVTable<'a> {
    let mut f = FindMethods { class_vtable };
    visit_ast(&mut f, ast);
    f.class_vtable
}

impl<'a> Visitor<'a> for FindMethods<'a> {
    fn visit_define_method(&mut self, node: &'a ast::DefineMethod<'a>) {
        let method_name = &node.method_name.ident;

        let block = &node.block;
        let parameters = &block.parameters;
        let body = &block.body;

        let method = Method {
            name: method_name,
            parameters,
            body,
        };

        let class_name = &node.class_name.0.name;
        // TODO: Conver this to Result
        let class = self
            .class_vtable
            .table
            .get_mut(class_name)
            .expect("Undefined class");
        class.methods.insert(method_name.name, method);
    }
}

#[derive(Debug)]
pub struct ClassVTable<'a> {
    pub table: VTable<'a, Class<'a>>,
}

#[derive(Debug)]
pub struct Class<'a> {
    pub name: &'a Ident<'a>,
    pub fields: VTable<'a, Field<'a>>,
    pub methods: VTable<'a, Method<'a>>,
}

#[derive(Debug)]
pub struct Field<'a> {
    pub name: &'a Ident<'a>,
}

#[derive(Debug)]
pub struct Method<'a> {
    pub name: &'a Ident<'a>,
    pub parameters: &'a Vec<ast::Parameter<'a>>,
    pub body: &'a Vec<ast::Stmt<'a>>,
}
