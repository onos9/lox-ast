use std::fmt::{self};

use crate::token_type::*;

#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Num(n) => write!(f, "{n}"),
            Object::Str(s) => write!(f, "\"{s}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }

    pub fn is(&self, ttype: TokenType) -> bool {
        self.ttype == ttype
    }
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{:?} {} {}",
            self.ttype,
            self.lexeme,
            match &self.literal {
                Some(literal) => literal.to_string(),
                None => "None".to_string(),
            },
        )
    }
}
