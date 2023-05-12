use std::{collections::HashMap, rc::Rc, cell::RefCell};
use crate::{errors::LoxError, token::{Token, Tokenliteral}};

#[derive(Debug)]
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

    pub fn get_at(&self, distance: i32, name: &Token) -> Result<Tokenliteral, LoxError> {
        if distance == 0 {
            let ret = self.values.get(&name.lexeme);
            if let Some(v) = ret {
                return Ok(v.clone());
            }
            return Err(LoxError::RuntimeError(name.clone(), "env get error".to_string()));
        } else {
            if let Some(enclosing) = &self.env_visitor {
                return self.get_at_idx(&name, enclosing, 1, distance);
            } else {
                return Err(LoxError::RuntimeError(name.clone(), "env get error".to_string()));
            }
        }
    }

    fn get_at_idx(&self, 
        name: &Token,
        enclosing: &Rc<RefCell<Environment>>, 
        current: i32, 
        distance: i32) -> Result<Tokenliteral, LoxError> {
        if current == distance {
            return enclosing.borrow().get(&name);
        } else {
            if let Some(env) = &enclosing.borrow().env_visitor {
                return self.get_at_idx(name, env, current + 1, distance);
            }
        }
        return Err(LoxError::RuntimeError(name.clone(), "get_at_idx".to_string()));
    }

    pub fn assign_at(&mut self, name: &Token, value: &Tokenliteral, distance: i32) {
        if distance == 0 {
            self.values.insert(name.lexeme.to_string(), value.clone());
        } else {
            if let Some(enclosing) = &self.env_visitor {
                return self.assign_at_idx(name, value, enclosing, 1, distance);
            }
        }
    }

    fn assign_at_idx(
        &self,
        name: &Token,
        value: &Tokenliteral,
        enclosing: &Rc<RefCell<Environment>>,
        current: i32,
        distance: i32
        ) {
            if current == distance {
                enclosing.borrow_mut().values.insert(name.lexeme.to_string(), value.clone());
            } else {
                if let Some(env) = &enclosing.borrow().env_visitor {
                    self.assign_at_idx(name, value, env, current + 1, distance)
                }
            }
    }

}