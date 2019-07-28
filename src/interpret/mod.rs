use crate::prep::{self, Class, Field};
use crate::{
    ast::{visit_ast, Ast, Visitor, *},
    error::{Error, Result},
    Span,
};
use std::{
    collections::{hash_map::Keys, HashMap},
    rc::Rc,
};

pub type VTable<'a, T> = HashMap<&'a str, T>;

pub type ClassVTable<'a> = VTable<'a, Rc<Class<'a>>>;

pub fn interpret<'a>(interpreter: &'a mut Interpreter<'a>, ast: &'a Ast<'a>) -> Result<'a, ()> {
    visit_ast(interpreter, ast)?;
    dbg!(&interpreter.locals);
    Ok(())
}

pub struct Interpreter<'a> {
    classes: Rc<ClassVTable<'a>>,
    locals: VTable<'a, Value<'a>>,
    self_: Option<Value<'a>>,
    return_value: Option<Value<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(classes: prep::Classes<'a>) -> Self {
        Self {
            classes: Rc::new(classes),
            locals: HashMap::new(),
            self_: None,
            return_value: None,
        }
    }

    fn copy_for_method_call(
        &self,
        new_self: Value<'a>,
        locals: VTable<'a, Value<'a>>,
    ) -> Interpreter<'a> {
        Interpreter {
            classes: Rc::clone(&self.classes),
            locals,
            self_: Some(new_self),
            return_value: None,
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
        if self.return_value.is_some() {
            return Ok(());
        }

        let name = &node.ident.name;
        let value = node.body.eval(self)?;
        self.locals.insert(name, value);
        Ok(())
    }

    fn visit_let_ivar(&mut self, node: &'a LetIVar<'a>) -> Result<'a, ()> {
        if self.return_value.is_some() {
            return Ok(());
        }

        unimplemented!("TODO: visit_let_ivar")
    }

    fn visit_message_send_stmt(&mut self, node: &'a MessageSendStmt<'a>) -> Result<'a, ()> {
        if self.return_value.is_some() {
            return Ok(());
        }
        node.expr.eval(self)?;
        Ok(())
    }

    fn visit_return(&mut self, node: &'a Return<'a>) -> Result<'a, ()> {
        let value = node.expr.eval(self)?;
        self.return_value = Some(value);
        Ok(())
    }
}

#[derive(Debug)]
enum Value<'a> {
    Number(i32),
    True,
    False,
    Nil,
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
            Value::Nil => Value::Nil,
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
            Expr::Self_(inner) => inner.eval(interpreter),
            Expr::MessageSend(inner) => inner.eval(interpreter),
            Expr::IVar(inner) => inner.eval(interpreter),

            Expr::Block(_) => unimplemented!("eval Block"),
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

impl<'a> Eval<'a> for Self_ {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let self_ = interpreter
            .self_
            .as_ref()
            .ok_or_else(|| Error::NoSelf(self.0))?;
        Ok(self_.to_owned())
    }
}

impl<'a> Eval<'a> for ClassNew<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let class_name = self.class_name.0.name;
        let call_site = self.class_name.0.span;
        let class = interpreter.lookup_class(class_name, call_site)?;

        let parameters = class.fields.keys().copied().collect::<Vec<_>>();
        let ivars = eval_arguments(interpreter, parameters, call_site, &self.args)?;

        let instance = Instance { class, ivars };

        Ok(Value::Instance(Rc::new(instance)))
    }
}

fn eval_arguments<'a>(
    interpreter: &Interpreter<'a>,
    parameters: Vec<&'a str>,
    call_site: Span,
    args: &[Argument<'a>],
) -> Result<'a, VTable<'a, Value<'a>>> {
    let mut arg_values = VTable::with_capacity(args.len());
    for arg in args {
        let value = arg.expr.eval(interpreter)?;
        arg_values.insert(&arg.ident.name, (value, arg.span));
    }

    let mut ivars = VTable::with_capacity(args.len());
    for param in parameters {
        let (value, _) = arg_values
            .remove(param)
            .ok_or_else(|| Error::MissingArgument {
                name: param,
                span: call_site,
            })?;
        ivars.insert(param, value);
    }

    for (name, (_value, span)) in arg_values {
        Err(Error::UnexpectedArgument { name, span })?;
    }

    Ok(ivars)
}

impl<'a> Eval<'a> for MessageSend<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let receiver = self.receiver.eval(interpreter)?;
        let receiver = match receiver {
            Value::Instance(instance) => instance,
            _ => return Err(Error::MessageSentToNonInstance(self.span)),
        };

        let method = receiver.class.get_method_named(self.msg.name, self.span)?;

        let new_self = Value::Instance(Rc::clone(&receiver));

        let parameters = method
            .parameters
            .iter()
            .map(|param| param.ident.name)
            .collect::<Vec<_>>();
        let new_locals = eval_arguments(interpreter, parameters, self.span, &self.args)?;

        let mut method_interpreter = interpreter.copy_for_method_call(new_self, new_locals);

        visit_ast(&mut method_interpreter, method.body)?;

        let return_value = method_interpreter
            .return_value
            .unwrap_or_else(|| Value::Nil);
        Ok(return_value)
    }
}

impl<'a> Eval<'a> for IVar<'a> {
    fn eval(&self, interpreter: &Interpreter<'a>) -> Result<'a, Value<'a>> {
        let name = &self.ident.name;
        let span = self.span;

        let instance = match &interpreter.self_ {
            Some(inner) => match inner.to_owned() {
                Value::Instance(instance) => instance,
                _ => return Err(Error::MessageSentToNonInstance(self.span)),
            },
            None => return Err(Error::IVarAccessedOutsideMethod { name, span }),
        };

        let value = instance
            .ivars
            .get(name)
            .ok_or_else(|| Error::UndefinedIVar { name, span })?
            .to_owned();

        Ok(value)
    }
}
