use std::fmt;

use crate::{token_type::TokenType, loxfunction::{LoxFunction}, loxclass::{LoxClass, LoxInstance}};
use crate::rust_number::Number;

#[derive(Clone, Debug)]
pub enum Tokenliteral {
    Lstirng(String),
    Lnumber(Number),
    Lbool(bool),
    LCall(LoxFunction),
    LClass(LoxClass),
    LInst(LoxInstance),
    Nil,
}
impl fmt::Display for Tokenliteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Tokenliteral::Lstirng(v) => write!(f, "{}", v),
            Tokenliteral::Lnumber(v) => write!(f, "{}", v.to_value()),
            Tokenliteral::Lbool(v) => write!(f, "{}", v),
            Tokenliteral::Nil => write!(f, "Nil", ),
            Tokenliteral::LCall(v) => {
                let mut s = "<fn ".to_string();
                s.push_str(&v.name);
                s.push_str(">");
                write!(f, "{}", s)
            }
            Tokenliteral::LClass(v) => {
                write!(f, "{}", v.name)
            }
            Tokenliteral::LInst(v) => {
                write!(f, "{} instance", v.klass.name)
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Tokenliteral,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: &Tokenliteral, line: i32) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme.to_string(),
            literal: literal.clone(),
            line: line
        }
    }
}