mod class;
mod visitor;

use self::class::find_classes_and_methods;
use crate::{error::Result, parse::Ast};
use std::{collections::HashMap, fmt};

type VTable<'a, T> = HashMap<&'a str, T>;

pub fn interpret<'a>(ast: &'a Ast<'a>) -> Result<()> {
    let class_vtable = find_classes_and_methods(ast)?;

    Ok(())
}

#[derive(Debug)]
enum Value {
    Nil,
    Digit(i32),
}

#[derive(Debug)]
pub enum InterpretError {
    Error(String),
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpretError::Error(reason) => write!(f, "Interpret error: {}", reason),
        }
    }
}

impl std::error::Error for InterpretError {}
