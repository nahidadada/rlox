use std::{collections::HashMap, rc::Rc, cell::RefCell};
use crate::{errors::LoxError, token::{Token, Tokenliteral}};

pub struct Environment {
    values: HashMap<String, Tokenliteral>,
    env_visitor: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment { 
            values: HashMap::new(), 
            env_visitor: None,
        }
    }
    
    pub fn new_with_visitor(env: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            env_visitor: Some(Rc::clone(env)),
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

        if let Some(enclosing) = &self.env_visitor {
            return enclosing.borrow().get(name);
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

        if let Some(enclosing) = &mut self.env_visitor {
            return enclosing.borrow_mut().assign(name, value);
        }

        let mut msg = String::from("Undefined variable '");
        msg.push_str(&name.lexeme);
        msg.push_str("'.");
        Err(LoxError::RuntimeError(name.clone(), msg))
    }
}