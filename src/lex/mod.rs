use crate::Pos;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{self, Write};

pub fn lex<'a>(program: &'a str) -> Vec<Token<'a>> {
    Lexer::lex(program)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Token<'a> {
    Let(Let),
    Name(Name<'a>),
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
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Let(inner) => write!(f, "{}", inner),
            Token::Name(inner) => write!(f, "{}", inner),
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
        }
    }
}

pub trait ParseToken<'a>: Sized {
    fn debug_name() -> &'static str;

    fn from_token<'b>(token: &'b Token<'a>) -> Option<&'b Self>;
}

macro_rules! token_with_pos {
    ( $name:ident, $re_name:ident, $pattern:expr ) => {
        #[derive(Eq, PartialEq, Debug)]
        pub struct $name {
            pub pos: Pos,
        }

        impl $name {
            fn new(pos: Pos) -> Self {
                Self { pos }
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

        impl<'a> ParseToken<'a> for $name {
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

token_with_pos!(Let, LET, "let");
token_with_pos!(Eq, EQ, "=");
token_with_pos!(Semicolon, SEMICOLON, ";");
token_with_pos!(OBracket, OBRACKET, r#"\["#);
token_with_pos!(CBracket, CBRACKET, r#"\]"#);
token_with_pos!(OBrace, OBRACE, r#"\{"#);
token_with_pos!(CBrace, CBRACE, r#"\}"#);
token_with_pos!(OParen, OPAREN, r#"\("#);
token_with_pos!(CParen, CPAREN, r#"\)"#);
token_with_pos!(Colon, COLON, ":");
token_with_pos!(At, AT, "@");
token_with_pos!(Hash, HASH, "#");
token_with_pos!(Comma, COMMA, ",");
token_with_pos!(Pipe, PIPE, r#"\|"#);
token_with_pos!(True, TRUE, "true");
token_with_pos!(False, FALSE, "false");

lazy_static! {
    static ref NAME: Regex = Regex::new(r#"\A([a-zA-Z_]+)"#).unwrap();
    static ref DIGIT: Regex = Regex::new(r#"\A([0-9]+)"#).unwrap();
}

#[derive(Eq, PartialEq, Debug)]
pub struct Name<'a> {
    pub name: &'a str,
    pub pos: Pos,
}

impl<'a> Name<'a> {
    fn new(name: &'a str, pos: Pos) -> Self {
        Self { name, pos }
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

impl<'a> ParseToken<'a> for Name<'a> {
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
pub struct Digit {
    pub digit: i32,
    pub pos: Pos,
}

impl Digit {
    fn new(digit: i32, pos: Pos) -> Self {
        Self { digit, pos }
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

impl<'a> ParseToken<'a> for Digit {
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
    pos: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lexer<'a> {
    fn lex(program: &'a str) -> Vec<Token<'a>> {
        let mut lexer = Self {
            program,
            pos: 0,
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
        self.pos >= self.program.len()
    }

    fn step(&mut self) {
        macro_rules! scan_for {
            ( $ty:ty ) => {
                if let Some(capture) = self.scan(<$ty>::regex()) {
                    let token = <$ty>::new(self.new_pos(capture.len()));
                    let token = Token::from(token);
                    self.tokens.push(token);
                    self.pos += capture.len();
                    return;
                }
            };

            ( $ty:ty, $make_token:expr ) => {
                if let Some(capture) = self.scan(<$ty>::regex()) {
                    let token = ($make_token)(capture);
                    let token = Token::from(token);
                    self.tokens.push(token);
                    self.pos += capture.len();
                    return;
                }
            };
        }

        self.ignore_whitespace();

        scan_for!(Let);
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
        scan_for!(Name, |capture: &'a str| Name::new(
            capture,
            self.new_pos(capture.len())
        ));
        scan_for!(Digit, |capture: &'a str| {
            let digit = capture
                .parse::<i32>()
                .expect("tokenized a digit, but parsing to i32 didn't work");
            Digit::new(digit, self.new_pos(capture.len()))
        });

        if self.at_end() {
            return;
        }

        // TODO: Turn this error into a Result
        let mut f = String::new();
        writeln!(f, "Unexpected token!").unwrap();
        writeln!(f).unwrap();
        writeln!(f, "{}", &self.program[self.pos..=self.pos]).unwrap();
        writeln!(f).unwrap();
        writeln!(f, "Tokens:").unwrap();
        writeln!(f, "{:#?}", self.tokens).unwrap();
        panic!("{}", f);
    }

    fn scan(&self, re: &Regex) -> Option<&'a str> {
        let program = &self.program[self.pos..];

        re.captures(program).map(|captures| {
            let match_ = &captures[0];
            &program[0..match_.len()]
        })
    }

    fn new_pos(&self, len: usize) -> Pos {
        Pos::new(self.pos, self.pos + len)
    }

    fn ignore_whitespace(&mut self) {
        loop {
            if self.at_end() {
                break;
            }

            let rest = &self.program[self.pos..=self.pos];

            if rest.chars().all(|c| c.is_whitespace()) {
                self.pos += rest.chars().count();
            } else {
                break;
            }
        }
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
    }

    #[test]
    fn let_number() {
        let program = "let number = 1;";
        assert_eq!(
            lex(program),
            vec![
                Token::Let(Let::new(Pos::new(0, 3))),
                Token::Name(Name::new("number", Pos::from_with(4, "number"))),
                Token::Eq(Eq::new(Pos::from_with(11, "="))),
                Token::Digit(Digit::new(1, Pos::from_with(13, "1"))),
                Token::Semicolon(Semicolon::new(Pos::from_with(14, ";"))),
            ]
        );
    }

    #[test]
    fn bool() {
        let program = "true";
        assert_eq!(lex(program), vec![Token::True(True::new(Pos::new(0, 4)))]);

        let program = "false";
        assert_eq!(lex(program), vec![Token::False(False::new(Pos::new(0, 5)))]);
    }

    #[test]
    fn let_call() {
        let program = "[user set id: 123]\n";
        assert_eq!(
            lex(program),
            vec![
                Token::OBracket(OBracket::new(Pos::from_with(0, "["))),
                Token::Name(Name::new("user", Pos::from_with(1, "user"))),
                Token::Name(Name::new("set", Pos::from_with(6, "set"))),
                Token::Name(Name::new("id", Pos::from_with(10, "id"))),
                Token::Colon(Colon::new(Pos::from_with(12, ":"))),
                Token::Digit(Digit::new(123, Pos::from_with(14, "123"))),
                Token::CBracket(CBracket::new(Pos::from_with(17, "]"))),
            ]
        );
    }
}
