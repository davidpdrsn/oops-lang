use super::{
    visitor::{visit_ast, Visitor},
    Ast, VTable,
};
use crate::{
    error::{Error, Result},
    parse::{self as ast, Ident},
    Span,
};
use std::collections::HashMap;

pub fn find_classes_and_methods<'a>(ast: &'a Ast<'a>) -> Result<'a, ClassVTable<'a>> {
    let class_vtable = find_classes(ast)?;
    find_methods(ast, class_vtable)
}

fn find_classes<'a>(ast: &'a Ast<'a>) -> Result<'a, ClassVTable<'a>> {
    let mut f = FindClasses::default();
    visit_ast(&mut f, ast)?;
    Ok(ClassVTable { table: f.table })
}

#[derive(Default)]
struct FindClasses<'a> {
    table: VTable<'a, Class<'a>>,
}

impl<'a> Visitor<'a> for FindClasses<'a> {
    type Error = Error<'a>;

    fn visit_define_class(&mut self, node: &'a ast::DefineClass<'a>) -> Result<'a, ()> {
        let name = &node.name.class_name.0;
        let key = name.name;

        self.check_for_existing_class_with_same_name(key, node)?;

        let fields = self.make_fields(node);

        let class = Class::new(name, fields, node.span);

        self.table.insert(key, class);

        Ok(())
    }
}

impl<'a> FindClasses<'a> {
    fn check_for_existing_class_with_same_name(
        &self,
        key: &'a str,
        node: &'a ast::DefineClass<'a>,
    ) -> Result<'a, ()> {
        if let Some(other) = self.table.get(key) {
            Err(Error::ClassAlreadyDefined {
                class: &key,
                first_span: other.span,
                second_span: node.span,
            })
        } else {
            Ok(())
        }
    }

    fn make_fields(&self, node: &'a ast::DefineClass<'a>) -> VTable<'a, Field<'a>> {
        node.fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let field = Field { name: ident };
                (ident.name, field)
            })
            .collect()
    }
}

struct FindMethods<'a> {
    class_vtable: ClassVTable<'a>,
}

fn find_methods<'a>(
    ast: &'a Ast<'a>,
    class_vtable: ClassVTable<'a>,
) -> Result<'a, ClassVTable<'a>> {
    let mut f = FindMethods { class_vtable };
    visit_ast(&mut f, ast)?;
    Ok(f.class_vtable)
}

impl<'a> Visitor<'a> for FindMethods<'a> {
    type Error = Error<'a>;

    fn visit_define_method(&mut self, node: &'a ast::DefineMethod<'a>) -> Result<'a, ()> {
        let method_name = &node.method_name.ident;
        let key = method_name.name;

        let class_name = &node.class_name.0.name;

        {
            let class = self
                .get_class(class_name)
                .ok_or_else(|| Error::ClassNotDefined {
                    class: class_name,
                    span: node.span,
                })?;
            self.check_for_existing_method_with_same_name(class, key, node)?;
        }

        let method = self.make_method(method_name, &node.block, node.span);

        let class = self
            .get_class_mut(class_name)
            .ok_or_else(|| Error::ClassNotDefined {
                class: class_name,
                span: node.span,
            })?;
        class.methods.insert(key, method);

        Ok(())
    }
}

impl<'a> FindMethods<'a> {
    fn get_class(&self, name: &str) -> Option<&Class<'a>> {
        self.class_vtable.table.get(name)
    }

    fn get_class_mut(&mut self, name: &str) -> Option<&mut Class<'a>> {
        self.class_vtable.table.get_mut(name)
    }

    fn check_for_existing_method_with_same_name(
        &self,
        class: &Class<'a>,
        key: &'a str,
        node: &'a ast::DefineMethod<'a>,
    ) -> Result<'a, ()> {
        if let Some(other) = class.methods.get(key) {
            return Err(Error::MethodAlreadyDefined {
                class: class.name.name,
                method: key,
                first_span: other.span,
                second_span: node.span,
            });
        } else {
            Ok(())
        }
    }

    fn make_method(
        &self,
        method_name: &'a Ident<'a>,
        block: &'a ast::Block<'a>,
        span: Span,
    ) -> Method<'a> {
        Method {
            name: method_name,
            parameters: &block.parameters,
            body: &block.body,
            span,
        }
    }
}

#[derive(Debug)]
pub struct ClassVTable<'a> {
    pub table: VTable<'a, Class<'a>>,
}

impl<'a> ClassVTable<'a> {
    fn get(&self, key: &str) -> Option<&'a Class> {
        self.table.get(key)
    }
}

#[derive(Debug)]
pub struct Class<'a> {
    pub name: &'a Ident<'a>,
    pub fields: VTable<'a, Field<'a>>,
    pub methods: VTable<'a, Method<'a>>,
    pub span: Span,
}

impl<'a> Class<'a> {
    fn new(name: &'a Ident<'a>, fields: VTable<'a, Field<'a>>, span: Span) -> Self {
        Self {
            name,
            fields,
            methods: VTable::new(),
            span,
        }
    }
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
    pub span: Span,
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::{lex::lex, parse::parse};

    #[test]
    fn finds_classes_and_methods() {
        let program = r#"
            [User def: #foo do: || { return 123; }];
            [Class subclass name: #User fields: [#id]];
        "#;
        let tokens = lex(&program).unwrap();
        let ast = parse(&tokens).unwrap();
        let class_vtable = find_classes_and_methods(&ast).unwrap();
        let class = class_vtable.get("User").unwrap();

        assert_eq!("User", class.name.name);

        assert_eq!(
            vec!["id"],
            class
                .fields
                .values()
                .map(|v| v.name.name)
                .collect::<Vec<_>>()
        );
        assert_eq!(vec![&"id"], class.fields.keys().collect::<Vec<_>>());

        assert_eq!(
            vec!["foo"],
            class
                .methods
                .values()
                .map(|v| v.name.name)
                .collect::<Vec<_>>()
        );
        assert_eq!(vec![&"foo"], class.methods.keys().collect::<Vec<_>>());
    }

    #[test]
    fn errors_if_class_is_defined_twice() {
        let program = r#"
            [Class subclass name: #User fields: [#foo]];
            [Class subclass name: #User fields: [#bar]];
        "#;
        let tokens = lex(&program).unwrap();
        let ast = parse(&tokens).unwrap();
        let result = find_classes_and_methods(&ast);

        assert_error!(result, Error::ClassAlreadyDefined { .. });
    }

    #[test]
    fn errors_if_method_is_defined_twice() {
        let program = r#"
            [Class subclass name: #User fields: [#foo]];
            [User def: #foo do: || { return 1; }];
            [User def: #foo do: || { return 2; }];
        "#;
        let tokens = lex(&program).unwrap();
        let ast = parse(&tokens).unwrap();
        let result = find_classes_and_methods(&ast);

        assert_error!(result, Error::MethodAlreadyDefined { .. });
    }

    #[test]
    fn errors_if_you_define_methods_on_classes_that_dont_exist() {
        let program = r#"
            [User def: #foo do: || { return 1; }];
        "#;
        let tokens = lex(&program).unwrap();
        let ast = parse(&tokens).unwrap();
        let result = find_classes_and_methods(&ast);

        assert_error!(result, Error::ClassNotDefined { .. });
    }
}
