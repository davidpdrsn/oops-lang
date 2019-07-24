use crate::Span;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self, Write};

pub fn lex<'a>(program: &'a str) -> Vec<Token<'a>> {
    Lexer::lex(program)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Token<'a> {
    Let(Let),
    Self_(Self_),
    Name(Name<'a>),
    ClassName(ClassName<'a>),
    Eq(Eq),
    Digit(Digit),
    Semicolon(Semicolon),
    OBracket(OBracket),
    CBracket(CBracket),
    OBrace(OBrace),
    CBrace(CBrace),
    OParen(OParen),
    CParen(CParen),
    Colon(Colon),
    At(At),
    Hash(Hash),
    Comma(Comma),
    Pipe(Pipe),
    True(True),
    False(False),
    Return(Return),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Let(inner) => write!(f, "{}", inner),
            Token::Name(inner) => write!(f, "{}", inner),
            Token::ClassName(inner) => write!(f, "{}", inner),
            Token::Self_(inner) => write!(f, "{}", inner),
            Token::Eq(inner) => write!(f, "{}", inner),
            Token::Digit(inner) => write!(f, "{}", inner),
            Token::Semicolon(inner) => write!(f, "{}", inner),
            Token::OBracket(inner) => write!(f, "{}", inner),
            Token::CBracket(inner) => write!(f, "{}", inner),
            Token::OBrace(inner) => write!(f, "{}", inner),
            Token::CBrace(inner) => write!(f, "{}", inner),
            Token::OParen(inner) => write!(f, "{}", inner),
            Token::CParen(inner) => write!(f, "{}", inner),
            Token::Colon(inner) => write!(f, "{}", inner),
            Token::At(inner) => write!(f, "{}", inner),
            Token::Hash(inner) => write!(f, "{}", inner),
            Token::Comma(inner) => write!(f, "{}", inner),
            Token::Pipe(inner) => write!(f, "{}", inner),
            Token::True(inner) => write!(f, "{}", inner),
            Token::False(inner) => write!(f, "{}", inner),
            Token::Return(inner) => write!(f, "{}", inner),
        }
    }
}

pub trait Parse<'a>: Sized {
    fn debug_name() -> &'static str;

    fn from_token<'b>(token: &'b Token<'a>) -> Option<&'b Self>;
}

macro_rules! token_with_span {
    ( $name:ident, $re_name:ident, $pattern:expr ) => {
        #[derive(Eq, PartialEq, Debug)]
        pub struct $name {
            pub span: Span,
        }

        impl $name {
            fn new(span: Span) -> Self {
                Self { span }
            }

            #[inline]
            fn regex() -> &'static Regex {
                &$re_name
            }
        }

        impl<'a> From<$name> for Token<'a> {
            fn from(val: $name) -> Token<'a> {
                Token::$name(val)
            }
        }

        lazy_static! {
            static ref $re_name: Regex = {
                let re = format!(r#"\A({})"#, $pattern);
                Regex::new(&re).unwrap()
            };
        }

        impl<'a> Parse<'a> for $name {
            fn debug_name() -> &'static str {
                $pattern
            }

            fn from_token<'b>(token: &'b Token<'a>) -> Option<&'b Self> {
                if let Token::$name(inner) = token {
                    Some(inner)
                } else {
                    None
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", $pattern)
            }
        }
    };
}

token_with_span!(Let, LET, "let");
token_with_span!(Self_, SELF, "self");
token_with_span!(Eq, EQ, "=");
token_with_span!(Semicolon, SEMICOLON, ";");
token_with_span!(OBracket, OBRACKET, r#"\["#);
token_with_span!(CBracket, CBRACKET, r#"\]"#);
token_with_span!(OBrace, OBRACE, r#"\{"#);
token_with_span!(CBrace, CBRACE, r#"\}"#);
token_with_span!(OParen, OPAREN, r#"\("#);
token_with_span!(CParen, CPAREN, r#"\)"#);
token_with_span!(Colon, COLON, ":");
token_with_span!(At, AT, "@");
token_with_span!(Hash, HASH, "#");
token_with_span!(Comma, COMMA, ",");
token_with_span!(Pipe, PIPE, r#"\|"#);
token_with_span!(True, TRUE, "true");
token_with_span!(False, FALSE, "false");
token_with_span!(Return, RETURN, "return");

lazy_static! {
    static ref CLASS_NAME: Regex = Regex::new(r#"\A([A-Z][a-zA-Z_]*)"#).unwrap();
    static ref NAME: Regex = Regex::new(r#"\A([a-z][a-zA-Z_]*)"#).unwrap();
    static ref DIGIT: Regex = Regex::new(r#"\A([0-9]+)"#).unwrap();
    static ref WHITE_SPACE: Regex = Regex::new(r#"^( +|\n+|\t+)"#).unwrap();
    static ref COMMENT: Regex = Regex::new(r#"^(//[^\n]*)"#).unwrap();
}

#[derive(Eq, PartialEq, Debug)]
pub struct Name<'a> {
    pub name: &'a str,
    pub span: Span,
}

impl<'a> Name<'a> {
    fn new(name: &'a str, span: Span) -> Self {
        Self { name, span }
    }

    #[inline]
    fn regex() -> &'static Regex {
        &NAME
    }
}

impl<'a> From<Name<'a>> for Token<'a> {
    fn from(val: Name<'a>) -> Token<'a> {
        Token::Name(val)
    }
}

impl<'a> Parse<'a> for Name<'a> {
    fn debug_name() -> &'static str {
        "name"
    }

    fn from_token<'b>(token: &'b Token<'a>) -> Option<&'b Self> {
        if let Token::Name(inner) = token {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for Name<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct ClassName<'a> {
    pub name: &'a str,
    pub span: Span,
}

impl<'a> ClassName<'a> {
    fn new(name: &'a str, span: Span) -> Self {
        Self { name, span }
    }

    #[inline]
    fn regex() -> &'static Regex {
        &CLASS_NAME
    }
}

impl<'a> From<ClassName<'a>> for Token<'a> {
    fn from(val: ClassName<'a>) -> Token<'a> {
        Token::ClassName(val)
    }
}

impl<'a> Parse<'a> for ClassName<'a> {
    fn debug_name() -> &'static str {
        "class name"
    }

    fn from_token<'b>(token: &'b Token<'a>) -> Option<&'b Self> {
        if let Token::ClassName(inner) = token {
            Some(inner)
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for ClassName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Digit {
    pub digit: i32,
    pub span: Span,
}

impl Digit {
    fn new(digit: i32, span: Span) -> Self {
        Self { digit, span }
    }

    #[inline]
    fn regex() -> &'static Regex {
        &DIGIT
    }
}

impl<'a> From<Digit> for Token<'a> {
    fn from(val: Digit) -> Token<'a> {
        Token::Digit(val)
    }
}

impl<'a> Parse<'a> for Digit {
    fn debug_name() -> &'static str {
        "digit"
    }

    fn from_token<'b>(token: &'b Token<'a>) -> Option<&'b Self> {
        if let Token::Digit(inner) = token {
            Some(inner)
        } else {
            None
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.digit)
    }
}

struct Lexer<'a> {
    program: &'a str,
    span: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    fn lex(program: &'a str) -> Vec<Token<'a>> {
        let mut lexer = Self {
            program,
            span: 0,
            tokens: vec![],
        };

        loop {
            if lexer.at_end() {
                break;
            } else {
                lexer.step();
            }
        }

        lexer.tokens
    }

    fn at_end(&self) -> bool {
        self.span >= self.program.len()
    }

    fn step(&mut self) {
        macro_rules! scan_for {
            ( $ty:ty ) => {
                if let Some(capture) = self.scan(<$ty>::regex()) {
                    let token = <$ty>::new(self.new_span_with_length(capture.len()));
                    let token = Token::from(token);
                    self.tokens.push(token);
                    self.span += capture.len();
                    return;
                }
            };

            ( $ty:ty, $make_token:expr ) => {
                if let Some(capture) = self.scan(<$ty>::regex()) {
                    let token = ($make_token)(capture);
                    let token = Token::from(token);
                    self.tokens.push(token);
                    self.span += capture.len();
                    return;
                }
            };
        }

        while self.skip(&COMMENT) || self.skip(&WHITE_SPACE) {}

        scan_for!(Let);
        scan_for!(Self_);
        scan_for!(Eq);
        scan_for!(OBracket);
        scan_for!(CBracket);
        scan_for!(OParen);
        scan_for!(CParen);
        scan_for!(OBrace);
        scan_for!(CBrace);
        scan_for!(Semicolon);
        scan_for!(Colon);
        scan_for!(At);
        scan_for!(Hash);
        scan_for!(Comma);
        scan_for!(Pipe);
        scan_for!(True);
        scan_for!(False);
        scan_for!(Return);

        scan_for!(ClassName, |capture: &'a str| ClassName::new(
            capture,
            self.new_span_with_length(capture.len())
        ));

        scan_for!(Name, |capture: &'a str| Name::new(
            capture,
            self.new_span_with_length(capture.len())
        ));

        scan_for!(Digit, |capture: &'a str| {
            let digit = capture
                .parse::<i32>()
                .expect("tokenized a digit, but parsing to i32 didn't work");
            Digit::new(digit, self.new_span_with_length(capture.len()))
        });

        if self.at_end() {
            return;
        }

        // TODO: Turn this error into a Result
        let mut f = String::new();
        writeln!(f, "Unexpected token!").unwrap();
        writeln!(f).unwrap();
        writeln!(f, "{:?}", &self.program[self.span..]).unwrap();
        writeln!(f).unwrap();
        writeln!(f, "Tokens:").unwrap();
        writeln!(f, "{:#?}", self.tokens).unwrap();
        panic!("{}", f);
    }

    fn scan(&self, re: &Regex) -> Option<&'a str> {
        let program = &self.program[self.span..];

        re.captures(program).map(|captures| {
            let match_ = &captures[0];
            &program[0..match_.len()]
        })
    }

    fn skip(&mut self, re: &Regex) -> bool {
        let program = &self.program[self.span..];

        if let Some(captures) = re.captures(program) {
            let match_ = &captures[0];
            self.span += match_.len();
            true
        } else {
            false
        }
    }

    fn new_span_with_length(&self, len: usize) -> Span {
        Span::new(self.span, self.span + len)
    }
}

pub trait Indent {
    fn indent(&self, level: usize) -> String;
}

impl<T> Indent for T
where
    T: ToString,
{
    fn indent(&self, level: usize) -> String {
        let mut indent = String::new();
        for _ in 0..level {
            indent.push_str(" ");
        }

        self.to_string()
            .lines()
            .map(|line| format!("{}{}", indent, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn empty() {
        let program = "";
        assert_eq!(lex(program), vec![]);

        let program = " ";
        assert_eq!(lex(program), vec![]);
    }

    #[test]
    fn let_number() {
        let program = "let number = 1; ";
        assert_eq!(
            lex(program),
            vec![
                Token::Let(Let::new(Span::new(0, 3))),
                Token::Name(Name::new("number", Span::from_with(4, "number"))),
                Token::Eq(Eq::new(Span::from_with(11, "="))),
                Token::Digit(Digit::new(1, Span::from_with(13, "1"))),
                Token::Semicolon(Semicolon::new(Span::from_with(14, ";"))),
            ]
        );
    }

    #[test]
    fn bool() {
        let program = "true";
        assert_eq!(lex(program), vec![Token::True(True::new(Span::new(0, 4)))]);

        let program = "false";
        assert_eq!(lex(program), vec![Token::False(False::new(Span::new(0, 5)))]);
    }

    #[test]
    fn let_call() {
        let program = "[user set id: 123]\n";
        assert_eq!(
            lex(program),
            vec![
                Token::OBracket(OBracket::new(Span::from_with(0, "["))),
                Token::Name(Name::new("user", Span::from_with(1, "user"))),
                Token::Name(Name::new("set", Span::from_with(6, "set"))),
                Token::Name(Name::new("id", Span::from_with(10, "id"))),
                Token::Colon(Colon::new(Span::from_with(12, ":"))),
                Token::Digit(Digit::new(123, Span::from_with(14, "123"))),
                Token::CBracket(CBracket::new(Span::from_with(17, "]"))),
            ]
        );
    }

    #[test]
    fn ignores_comments_with_newline() {
        lex("// Just a comment\n");
        lex("// one\n// two\n");
        lex("let n = 1; // a comment");
        lex("let n = 1; // a comment\n");
        lex("// Just a comment");
        lex("// one\n// two");

        let program = vec![
            "// a comment\n",
            "let n = 1;\n",
            "// a comment\n",
            "// a comment\n\n",
            "let n = 1;\n",
            "// a comment\n",
            "// a comment\n",
        ]
        .join("");
        lex(&program);
    }
}
