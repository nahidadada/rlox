use std::collections::HashMap;

use crate::{errors::LoxError, token::{Token, Tokenliteral}};

pub struct Environment {
    values: HashMap<String, Tokenliteral>,
}
impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: &Tokenliteral) {
        self.values.insert(name.to_string(), value.clone());
    }

    pub fn get(&self, name: &str) -> Result<Tokenliteral, LoxError> {
        let mut msg = "Undefined variable '".to_string();
        msg.push_str(name);
        msg.push_str("'.'");

        let ret = self.values.get(name);
        if let Some(v) = ret {
            return Ok(v.clone());
        }
        return Err(LoxError::ValueError(msg));
    }
}