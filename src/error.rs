use crate::Span;
use std::{fmt, io};

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

#[derive(Debug)]
pub enum Error<'a> {
    LexError {
        at: usize,
    },
    IoError(io::Error),
    // TODO: Add typed fields here instead of just a String
    ParseError(String),
    ClassNotDefined {
        class: &'a str,
        span: Span,
    },
    ClassAlreadyDefined {
        class: &'a str,
        first_span: Span,
        second_span: Span,
    },
    MethodAlreadyDefined {
        class: &'a str,
        method: &'a str,
        first_span: Span,
        second_span: Span,
    },
    UndefinedLocal {
        name: &'a str,
        span: Span,
    },
    MissingArgument {
        name: &'a str,
        span: Span,
    },
    UnexpectedArgument {
        name: &'a str,
        span: Span,
    },
    NoSelf(Span),
}

impl From<io::Error> for Error<'_> {
    fn from(other: io::Error) -> Self {
        Error::IoError(other)
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LexError { at } => write!(f, "Unexpected token at {}", at),
            Error::IoError(other) => write!(f, "{}", other),
            Error::ParseError(other) => write!(f, "{}", other),
            Error::ClassNotDefined {
                class,
                ..
            } => write!(
                f,
                "The class `{}` is not defined",
                class,
            ),
            Error::ClassAlreadyDefined {
                class,
                first_span,
                second_span,
            } => write!(
                f,
                "The class `{}` was defined more than once. First time at {}, second time at {}",
                class, first_span, second_span
            ),
            Error::MethodAlreadyDefined {
                class,
                method,
                first_span,
                second_span,
            } => write!(
                f,
                "The method `{class}#{method}` was defined more than once. First time at {first}, second time at {second}",
                class = class,
                method = method,
                first = first_span,
                second = second_span,
            ),
            Error::UndefinedLocal {
                name, span
            } => write!(
                f,
                "Undefined local variable `{}` at {}",
                name, span
            ),
            Error::MissingArgument {
                name, span
            } => write!(
                f,
                "Missing argument `{}:` at {}",
                name, span
            ),
            Error::UnexpectedArgument {
                name, span
            } => write!(
                f,
                "Unexpected argument `{}:` at {}",
                name, span
            ),
            Error::NoSelf(span) => write!(
                f,
                "`self` called outside method at {}",
                span,
            ),
        }
    }
}

impl std::error::Error for Error<'_> {}

macro_rules! assert_error {
    ($result:expr, $pat:pat) => {
        match $result {
            Err($pat) => {}
            other => panic!("\n\nExpected an error but got\n\n{:?}\n\n", other),
        }
    };
}
