use std::collections::HashMap;

use crate::{errors::LoxError, token::{Token, Tokenliteral, self}};

pub struct Environment {
    values: HashMap<String, Tokenliteral>,
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &Token, value: &Tokenliteral) {
        self.values.insert(name.lexeme.to_string(), value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Tokenliteral, LoxError> {
        let ret = self.values.get(&name.lexeme.to_string());
        if let Some(v) = ret {
            return Ok(v.clone());
        }

        let mut msg = "Undefined variable '".to_string();
        msg.push_str(&name.lexeme);
        msg.push_str("'.'");
        return Err(LoxError::RuntimeError(name.clone(), msg));
    }

    pub fn assign(&mut self, name: &Token, value: &Tokenliteral) -> Result<(), LoxError> {
        let ret = self.values.get(&name.lexeme);
        if let Some(_v) = ret {
            let _old = self.values.insert(name.lexeme.to_string(), value.clone());
            return Ok(());
        }

        let mut msg = String::from("Undefined variable '");
        msg.push_str(&name.lexeme);
        msg.push_str("'.");
        Err(LoxError::RuntimeError(name.clone(), msg))
    }
}