use self::Token::*;
use crate::Pos;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Write;

pub fn lex<'a>(program: &'a str) -> Vec<Token<'a>> {
    Lexer::lex(program)
}

#[derive(Eq, PartialEq, Debug)]
pub enum Token<'a> {
    Let(Pos),
    Name((&'a str, Pos)),
    Eq(Pos),
    Digit((i32, Pos)),
    Semicolon(Pos),
    OBracket(Pos),
    CBracket(Pos),
    OBrace(Pos),
    CBrace(Pos),
    OParen(Pos),
    CParen(Pos),
    Colon(Pos),
    At(Pos),
    Hash(Pos),
    Comma(Pos),
    Pipe(Pos),
    True(Pos),
    False(Pos),
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
        // TODO: Remove the need for this macro
        macro_rules! scan_for {
            ( $re:expr, $make_token:expr ) => {
                if let Some(capture) = self.scan(&$re) {
                    let token = ($make_token)(capture);
                    self.tokens.push(token);
                    self.pos += capture.len();
                    return;
                }
            };
        }

        lazy_static! {
            static ref RE_NAME: Regex = Regex::new(r#"\A([a-zA-Z_]+)"#).unwrap();
            static ref RE_LET: Regex = Regex::new(r#"\A(let)"#).unwrap();
            static ref RE_OBRACKET: Regex = Regex::new(r#"\A(\[)"#).unwrap();
            static ref RE_CBRACKET: Regex = Regex::new(r#"\A(\])"#).unwrap();
            static ref RE_OPAREN: Regex = Regex::new(r#"\A(\()"#).unwrap();
            static ref RE_CPAREN: Regex = Regex::new(r#"\A(\))"#).unwrap();
            static ref RE_COLON: Regex = Regex::new(r#"\A(:)"#).unwrap();
            static ref RE_EQ: Regex = Regex::new(r#"\A(=)"#).unwrap();
            static ref RE_SEMICOLON: Regex = Regex::new(r#"\A(;)"#).unwrap();
            static ref RE_DIGIT: Regex = Regex::new(r#"\A([0-9]+)"#).unwrap();
            static ref RE_AT: Regex = Regex::new(r#"\A(@)"#).unwrap();
            static ref RE_HASH: Regex = Regex::new(r#"\A(#)"#).unwrap();
            static ref RE_COMMA: Regex = Regex::new(r#"\A(,)"#).unwrap();
            static ref RE_PIPE: Regex = Regex::new(r#"\A(\|)"#).unwrap();
            static ref RE_OBRACE: Regex = Regex::new(r#"\A(\{)"#).unwrap();
            static ref RE_CBRACE: Regex = Regex::new(r#"\A(\})"#).unwrap();
            static ref RE_TRUE: Regex = Regex::new(r#"\A(true)"#).unwrap();
            static ref RE_FALSE: Regex = Regex::new(r#"\A(false)"#).unwrap();
        }

        self.ignore_whitespace();

        scan_for!(RE_LET, |capture: &'a str| Let(self.new_pos(capture.len())));
        scan_for!(RE_EQ, |capture: &'a str| Eq(self.new_pos(capture.len())));
        scan_for!(RE_OBRACKET, |capture: &'a str| OBracket(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_CBRACKET, |capture: &'a str| CBracket(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_OPAREN, |capture: &'a str| OParen(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_CPAREN, |capture: &'a str| CParen(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_OBRACE, |capture: &'a str| OBrace(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_CBRACE, |capture: &'a str| CBrace(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_SEMICOLON, |capture: &'a str| Semicolon(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_COLON, |capture: &'a str| Colon(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_AT, |capture: &'a str| At(self.new_pos(capture.len())));
        scan_for!(RE_HASH, |capture: &'a str| Hash(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_COMMA, |capture: &'a str| Comma(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_PIPE, |capture: &'a str| Pipe(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_TRUE, |capture: &'a str| True(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_FALSE, |capture: &'a str| False(
            self.new_pos(capture.len())
        ));
        scan_for!(RE_NAME, |capture: &'a str| Name((
            capture,
            self.new_pos(capture.len())
        )));
        scan_for!(RE_DIGIT, |capture: &'a str| {
            let digit = capture
                .parse::<i32>()
                .expect("tokenized a digit, but parsing to i32 didn't work");
            Digit((digit, self.new_pos(capture.len())))
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
                Let(Pos::new(0, 3)),
                Name(("number", Pos::from_with(4, "number"))),
                Eq(Pos::from_with(11, "=")),
                Digit((1, Pos::from_with(13, "1"))),
                Semicolon(Pos::from_with(14, ";")),
            ]
        );
    }

    #[test]
    fn bool() {
        let program = "true";
        assert_eq!(lex(program), vec![True(Pos::new(0, 4)),]);

        let program = "false";
        assert_eq!(lex(program), vec![False(Pos::new(0, 5)),]);
    }

    #[test]
    fn let_call() {
        let program = "[user set id: 123]\n";
        assert_eq!(
            lex(program),
            vec![
                OBracket(Pos::from_with(0, "[")),
                Name(("user", Pos::from_with(1, "user"))),
                Name(("set", Pos::from_with(6, "set"))),
                Name(("id", Pos::from_with(10, "id"))),
                Colon(Pos::from_with(12, ":")),
                Digit((123, Pos::from_with(14, "123"))),
                CBracket(Pos::from_with(17, "]")),
            ]
        );
    }
}
