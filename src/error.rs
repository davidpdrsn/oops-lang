use crate::Span;
use std::{fmt, io};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    LexError {
        at: usize,
    },
    IoError(io::Error),
    ParseError(String),
    ClassNotDefined {
        class: String,
        span: Span,
    },
    ClassAlreadyDefined {
        class: String,
        first_span: Span,
        second_span: Span,
    },
    MethodAlreadyDefined {
        class: String,
        method: String,
        first_span: Span,
        second_span: Span,
    },
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::IoError(other)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LexError { at } => write!(f, "Unexpected token at {}", at),
            Error::IoError(other) => write!(f, "{}", other),
            Error::ParseError(other) => write!(f, "{}", other),
            Error::ClassNotDefined {
                class,
                span: _,
            } => write!(
                f,
                "The class {} is not defined",
                class,
            ),
            Error::ClassAlreadyDefined {
                class,
                first_span,
                second_span,
            } => write!(
                f,
                "The class {} was defined more than once. First time at {}, second time at {}",
                class, first_span, second_span
            ),
            Error::MethodAlreadyDefined {
                class,
                method,
                first_span,
                second_span,
            } => write!(
                f,
                "The method {class}#{method} was defined more than once. First time at {first}, second time at {second}",
                class = class,
                method = method,
                first = first_span,
                second = second_span,
            ),
        }
    }
}

impl std::error::Error for Error {}

macro_rules! assert_error {
    ($result:expr, $pat:pat) => {
        match $result {
            Err($pat) => {}
            other => panic!("\n\nExpected an error but got\n\n{:?}\n\n", other),
        }
    };
}
