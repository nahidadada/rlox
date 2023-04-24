use crate::{token::Token, token_type::{TokenType}};

#[derive(Debug)]
pub enum LoxError {
    ParseError,
    RuntimeError(Token, String),
}

pub struct Log {
    had_parse_error: bool,
    had_runtime_error: bool,
}

impl Log {
    pub fn new() -> Log {
        Log { had_parse_error: false, had_runtime_error: false }
    }
    
    pub fn had_parse_error(&self) -> bool {
        self.had_parse_error
    }

    pub fn had_runtime_error(&self) -> bool {
        self.had_runtime_error
    }

    pub fn reset_parse_error(&mut self) {
        self.had_parse_error = false;
    }

    pub fn reset_runtime_error(&mut self) {
        self.had_runtime_error = false;
    }

    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }
    
    pub fn report(&mut self, line: i32, place: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, place, message);
        self.had_parse_error = true;
    }

    pub fn token_error(&mut self, token: &Token, msg: &str) {
        if token.token_type == TokenType::Eofs {
            self.report(token.line, " at end", msg);
        } else {
            let mut place = " at '".to_string();
            place.push_str(&token.lexeme);
            place.push_str("'");
            self.report(token.line, &place, msg);
        }
    }

    pub fn runtime_error(&mut self, e: &LoxError) {
        if let LoxError::RuntimeError(token, msg) = e {
            println!("line {}, {} : {}", token.line, token.lexeme, msg);
        }
        self.had_runtime_error = true;
    }
}