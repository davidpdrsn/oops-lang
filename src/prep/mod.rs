use crate::ast::{visit_ast, Ast, Visitor};
use crate::{
    ast::{self, Ident},
    error::{Error, Result},
    interpret::{ClassVTable, VTable},
    Span,
};
use std::{collections::HashMap, rc::Rc};

pub type Classes<'a> = VTable<'a, Rc<Class<'a>>>;

pub fn find_classes_and_methods<'a>(
    ast: &'a Ast<'a>,
    built_in_classes: Classes<'a>,
) -> Result<'a, Classes<'a>> {
    let classes = find_classes(ast, built_in_classes)?;
    find_methods(ast, classes)
}

fn find_classes<'a>(ast: &'a Ast<'a>, built_in_classes: Classes<'a>) -> Result<'a, Classes<'a>> {
    let mut f = FindClasses {
        table: built_in_classes,
    };
    visit_ast(&mut f, ast)?;
    f.setup_super_classes()?;
    Ok(f.table)
}

struct FindClasses<'a> {
    table: Classes<'a>,
}

impl<'a> Visitor<'a> for FindClasses<'a> {
    type Error = Error<'a>;

    fn visit_define_class(&mut self, node: &'a ast::DefineClass<'a>) -> Result<'a, ()> {
        let name = &node.name.class_name.0;
        let key = name.name;

        self.check_for_existing_class_with_same_name(key, node)?;

        let fields = self.make_fields(node);

        let super_class_name = &node.super_class.class_name.0;
        let class = Class::new(name, super_class_name, fields, node.span);

        self.table.insert(key, Rc::new(class));

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

    fn setup_super_classes(&mut self) -> Result<'a, ()> {
        let mut acc = HashMap::new();

        for (class_name, class) in &self.table {
            let super_class_name = &class.super_class_name;

            // Object isn't supposed to have a super class
            if class_name == &"Object" {
                continue;
            }

            let super_class =
                self.table
                    .get(&super_class_name.name)
                    .ok_or_else(|| Error::ClassNotDefined {
                        class: super_class_name.name,
                        span: class.span,
                    })?;
            let super_class = Rc::clone(&super_class);

            acc.insert(*class_name, (super_class, class.span));
        }

        for (class_name, (super_class, span)) in acc {
            let mut class =
                self.table
                    .get_mut(class_name)
                    .ok_or_else(|| Error::ClassNotDefined {
                        class: class_name,
                        span,
                    })?;

            Rc::get_mut(&mut class)
                .expect("Internal error: Rc borrowed mut more than once")
                .super_class = Some(super_class);
        }

        Ok(())
    }
}

struct FindMethods<'a> {
    classes: Classes<'a>,
}

fn find_methods<'a>(ast: &'a Ast<'a>, classes: Classes<'a>) -> Result<'a, Classes<'a>> {
    let mut f = FindMethods { classes };
    visit_ast(&mut f, ast)?;
    Ok(f.classes)
}

impl<'a> Visitor<'a> for FindMethods<'a> {
    type Error = Error<'a>;

    fn visit_define_method(&mut self, node: &'a ast::DefineMethod<'a>) -> Result<'a, ()> {
        let method_name = &node.method_name.ident;
        let key = method_name.name;

        let class_name = &node.class_name.0.name;

        {
            let class = self
                .classes
                .get(class_name)
                .ok_or_else(|| Error::ClassNotDefined {
                    class: class_name,
                    span: node.span,
                })?;
            self.check_for_existing_method_with_same_name(class, key, node)?;
        }

        let method = self.make_method(method_name, &node.block, node.span);

        let mut class = self
            .classes
            .get_mut(class_name)
            .ok_or_else(|| Error::ClassNotDefined {
                class: class_name,
                span: node.span,
            })?;
        let class = Rc::get_mut(&mut class)
            .expect("Internal error: FindMethods.classes borrowed mut more than once");
        class.methods.insert(key, method);

        Ok(())
    }
}

impl<'a> FindMethods<'a> {
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
pub struct Class<'a> {
    pub name: &'a Ident<'a>,
    pub super_class_name: &'a Ident<'a>,
    pub super_class: Option<Rc<Class<'a>>>,
    pub fields: VTable<'a, Field<'a>>,
    pub methods: VTable<'a, Method<'a>>,
    pub span: Span,
}

impl<'a> Class<'a> {
    fn new(
        name: &'a Ident<'a>,
        super_class_name: &'a Ident<'a>,
        fields: VTable<'a, Field<'a>>,
        span: Span,
    ) -> Self {
        Self {
            name,
            fields,
            super_class_name,
            super_class: None,
            methods: VTable::new(),
            span,
        }
    }

    pub fn get_method_named(
        &self,
        method_name: &'a str,
        call_site: Span,
    ) -> Result<'a, &Method<'a>> {
        let method = self.methods.get(method_name);

        if let Some(method) = method {
            return Ok(method);
        }

        if let Some(super_class) = &self.super_class {
            // TODO: Change method name of returned error
            // Otherwise it'll always be "Object"
            return super_class.get_method_named(method_name, call_site);
        }

        Err(Error::UndefinedMethod {
            class: &self.name.name,
            method: method_name,
            span: call_site,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
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

// TODO: Bring back
// #[cfg(test)]
// mod test {
//     #[allow(unused_imports)]
//     use super::*;
//     use crate::{lex::lex, parse::parse};

//     #[test]
//     fn finds_classes_and_methods() {
//         let program = r#"
//             [User def: #foo do: || { return 123; }];
//             [Class subclass name: #User fields: [#id]];
//         "#;
//         let tokens = lex(&program).unwrap();
//         let ast = parse(&tokens).unwrap();
//         let classes = find_classes_and_methods(&ast).unwrap();
//         let class = classes.get("User").unwrap();

//         assert_eq!("User", class.name.name);

//         assert_eq!(
//             vec!["id"],
//             class
//                 .fields
//                 .values()
//                 .map(|v| v.name.name)
//                 .collect::<Vec<_>>()
//         );
//         assert_eq!(vec![&"id"], class.fields.keys().collect::<Vec<_>>());

//         assert_eq!(
//             vec!["foo"],
//             class
//                 .methods
//                 .values()
//                 .map(|v| v.name.name)
//                 .collect::<Vec<_>>()
//         );
//         assert_eq!(vec![&"foo"], class.methods.keys().collect::<Vec<_>>());
//     }

//     #[test]
//     fn errors_if_class_is_defined_twice() {
//         let program = r#"
//             [Class subclass name: #User fields: [#foo]];
//             [Class subclass name: #User fields: [#bar]];
//         "#;
//         let tokens = lex(&program).unwrap();
//         let ast = parse(&tokens).unwrap();
//         let result = find_classes_and_methods(&ast);

//         assert_error!(result, Error::ClassAlreadyDefined { .. });
//     }

//     #[test]
//     fn errors_if_method_is_defined_twice() {
//         let program = r#"
//             [Class subclass name: #User fields: [#foo]];
//             [User def: #foo do: || { return 1; }];
//             [User def: #foo do: || { return 2; }];
//         "#;
//         let tokens = lex(&program).unwrap();
//         let ast = parse(&tokens).unwrap();
//         let result = find_classes_and_methods(&ast);

//         assert_error!(result, Error::MethodAlreadyDefined { .. });
//     }

//     #[test]
//     fn errors_if_you_define_methods_on_classes_that_dont_exist() {
//         let program = r#"
//             [User def: #foo do: || { return 1; }];
//         "#;
//         let tokens = lex(&program).unwrap();
//         let ast = parse(&tokens).unwrap();
//         let result = find_classes_and_methods(&ast);

//         assert_error!(result, Error::ClassNotDefined { .. });
//     }
// }
