use crate::prep::{self, Class};
use crate::{
    ast::{visit_ast, Ast, Visitor, *},
    error::{Error, Result},
    Span,
};
use std::{collections::HashMap, rc::Rc};

pub type VTable<'a, T> = HashMap<&'a str, T>;

pub type ClassVTable<'a> = VTable<'a, Rc<Class<'a>>>;

pub fn interpret<'a>(interpreter: &'a mut Interpreter<'a>, ast: &'a Ast<'a>) -> Result<'a, ()> {
    visit_ast(interpreter, ast)?;
    dbg!(&interpreter.locals);

    Ok(())
}

pub struct Interpreter<'a> {
    classes: ClassVTable<'a>,
    locals: VTable<'a, Value<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(classes: prep::Classes<'a>) -> Self {
        let classes = classes
            .into_iter()
            .map(|(name, class)| (name, Rc::new(class)))
            .collect();

        Self {
            classes,
            locals: HashMap::new(),
        }
    }

    fn lookup_class(&self, name: &'a str, call_site: Span) -> Result<'a, Rc<Class<'a>>> {
        let class = self
            .classes
            .get(name)
            .ok_or_else(|| Error::ClassNotDefined {
                class: name,
                span: call_site,
            })?;
        Ok(Rc::clone(&class))
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
enum Value<'a> {
    Number(i32),
    True,
    False,
    List(Rc<Vec<Value<'a>>>),
    Instance(Rc<Instance<'a>>),
}

impl<'a> Value<'a> {
    fn to_owned(&self) -> Self {
        match self {
            Value::Number(n) => Value::Number(*n),
            Value::List(values) => Value::List(Rc::clone(values)),
            Value::True => Value::True,
            Value::False => Value::False,
            Value::Instance(instance) => Value::Instance(Rc::clone(instance)),
        }
    }
}

#[derive(Debug)]
struct Instance<'a> {
    class: Rc<Class<'a>>,
    ivars: VTable<'a, Value<'a>>,
}

trait Eval<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>>;
}

impl<'a> Eval<'a> for Expr<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        match self {
            Expr::Local(inner) => inner.eval(interpreter),
            Expr::Number(inner) => inner.eval(interpreter),
            Expr::List(inner) => inner.eval(interpreter),
            Expr::True(inner) => inner.eval(interpreter),
            Expr::False(inner) => inner.eval(interpreter),
            Expr::ClassNew(inner) => inner.eval(interpreter),

            Expr::IVar(_) => unimplemented!("eval IVar"),
            Expr::MessageSend(_) => unimplemented!("eval MessageSend"),
            Expr::Selector(_) => unimplemented!("eval Selector"),
            Expr::ClassNameSelector(_) => unimplemented!("eval ClassNameSelector"),
            Expr::Block(_) => unimplemented!("eval Block"),
            Expr::Self_(_) => unimplemented!("eval Self_"),
        }
    }
}

impl<'a> Eval<'a> for Local<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
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
    fn eval(&self, _: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let number = self.number;
        Ok(Value::Number(number))
    }
}

impl<'a> Eval<'a> for List<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let items = &self.items;
        let values: Result<'a, Vec<Value<'a>>> =
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
    fn eval(&self, _: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        Ok(Value::True)
    }
}

impl<'a> Eval<'a> for False {
    fn eval(&self, _: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        Ok(Value::False)
    }
}

impl<'a> Eval<'a> for ClassNew<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let class_name = self.class_name.0.name;
        let call_site = self.class_name.0.span;
        let class = interpreter.lookup_class(class_name, call_site)?;

        let ivars = eval_arguments(interpreter, &class, call_site, &self.args)?;

        let instance = Instance { class, ivars };

        Ok(Value::Instance(Rc::new(instance)))
    }
}

fn eval_arguments<'a>(
    interpreter: &Interpreter<'a>,
    class: &Class<'a>,
    call_site: Span,
    args: &[Argument<'a>],
) -> Result<'a, VTable<'a, Value<'a>>> {
    let mut args = args
        .iter()
        .map(|arg| (&arg.ident.name, arg))
        .collect::<HashMap<_, _>>();

    // TODO: Convert to `try_fold`
    let mut ivars = VTable::new();
    for field_name in class.fields.keys() {
        let arg = args
            .remove(field_name)
            .ok_or_else(|| Error::MissingArgument {
                name: field_name,
                span: call_site,
            })?;
        let value = arg.expr.eval(interpreter)?;
        ivars.insert(field_name, value);
    }

    for (name, arg) in args {
        Err(Error::UnexpectedArgument {
            name,
            span: arg.span,
        })?;
    }

    Ok(ivars)
}
