use crate::token::Token;

#[derive(Debug)]
pub enum LoxError {
    ParseError,
    RuntimeError(Token, String),
}

pub mod log {
    use crate::{token::Token, token_type::{TokenType, self}};

    use super::LoxError;

    pub fn error(line: i32, message: &str) {
        report(line, "", message);
    }
    
    pub fn report(line: i32, place: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, place, message);
        //had_error = true;
    }

    pub fn token_error(token: &Token, msg: &str) {
        if token.token_type == TokenType::Eofs {
            report(token.line, " at end", msg);
        } else {
            let mut place = " at '".to_string();
            place.push_str(&token.lexeme);
            place.push_str("'");
            report(token.line, &place, msg);
        }
    }

    pub fn runtime_error(e: LoxError) {
        if let LoxError::RuntimeError(token, msg) = e {
            println!("{:?} : {}", token.token_type, msg);
        }
        // had_runtime_error = true;
    }
}