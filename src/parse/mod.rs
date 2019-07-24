mod ast;
mod parse_stream;

use self::parse_stream::{ParseError, ParseStream};
use crate::lex::Token;

pub use ast::*;

pub fn parse<'a>(tokens: &'a Vec<Token<'a>>) -> Result<Vec<Stmt<'a>>, ParseError> {
    let mut stream = ParseStream::new(tokens);
    let acc = stream.parse_many::<Stmt>();

    if !stream.at_eof() {
        Err(ParseError::Error("Expected EOF, but wasn't".to_string()))
    } else {
        Ok(acc)
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::{lex::lex, Span};

    #[test]
    fn let_digit() {
        let program = "let number = 1;";
        let tokens = lex(&program);
        let ast = ok_or_panic(parse(&tokens));

        assert_eq!(
            ast,
            vec![Stmt::LetLocal(LetLocal {
                ident: Ident {
                    name: "number",
                    span: Span::new(4, 10)
                },
                body: Expr::Digit(Digit {
                    digit: 1,
                    span: Span::new(13, 14)
                }),
                span: Span::new(0, 15),
            })]
        );
    }

    #[test]
    fn let_name() {
        let program = "let a = b;";
        let tokens = lex(&program);
        let ast = ok_or_panic(parse(&tokens));

        assert_eq!(
            ast,
            vec![Stmt::LetLocal(LetLocal {
                ident: Ident {
                    name: "a",
                    span: Span::new(4, 5)
                },
                body: Expr::Local(Local(Ident {
                    name: "b",
                    span: Span::new(8, 9),
                })),
                span: Span::new(0, 10),
            })]
        );
    }

    fn ok_or_panic<T, E: std::error::Error>(value: Result<T, E>) -> T {
        match value {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}\n", e);
                panic!("error!")
            }
        }
    }
}
