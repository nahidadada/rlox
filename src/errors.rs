pub enum LoxError {
    ParseError,
}

pub mod log {
    use crate::{token::Token, token_type::TokenType};

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
}