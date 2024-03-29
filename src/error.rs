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
    MessageSentToNonInstance(Span),
    IVarAccessedWithoutSelf(Span),
    IVarAccessedOnNonInstanceValue(Span),
    UndefinedMethod {
        class: &'a str,
        method: &'a str,
        span: Span,
    },
    IVarAccessedOutsideMethod {
        name: &'a str,
        span: Span,
    },
    UndefinedIVar {
        name: &'a str,
        span: Span,
    },
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
            Error::MessageSentToNonInstance(span) => write!(
                f,
                "Message sent to non instance value at {}",
                span,
            ),
            Error::UndefinedMethod {
                class, method, span
            } => write!(
                f,
                "Undefined method `{}#{}` at {}",
                class, method, span
            ),
            Error::IVarAccessedOutsideMethod {
                name, span
            } => write!(
                f,
                "Instance variable `{}` accessed outside method at {}",
                name, span
            ),
            Error::UndefinedIVar {
                name, span
            } => write!(
                f,
                "Instance variable `{}` is not defined. Accessed at {}",
                name, span
            ),
            Error::IVarAccessedWithoutSelf(span) => write!(
                f,
                "Instance variabled access without a `self` at {}",
                span
            ),
            Error::IVarAccessedOnNonInstanceValue(span) => write!(
                f,
                "Instance variabled access on `self` that isn't an instance at {}",
                span
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
