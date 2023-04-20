use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub enum Tokenliteral {
    Lstirng(String),
    Lnumber(f64),
    Lbool(bool),
    Nil,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: String,
    pub literal: Tokenliteral,
    line: i32,
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

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        let s = format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal);
        s
    }
}