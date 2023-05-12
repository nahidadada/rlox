use std::{rc::Rc, cell::RefCell};

use crate::{token::{Tokenliteral}, interpreter::Interpreter, stmt::Stmt, environment::Environment, errors::LoxError};

pub trait LoxCallable {
    fn arity(&mut self) -> usize;
    fn call(&mut self, inter: &mut Interpreter, params: &Vec<Tokenliteral>) -> Result<Tokenliteral, LoxError>;
}

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub name: String,
    pub declaration: Box<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(name: &str, declaration: &Box<Stmt>, closure: &Rc<RefCell<Environment>>) -> LoxFunction {
        LoxFunction {
            name: name.to_string(),
            declaration: declaration.clone(),
            closure: Rc::clone(closure),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&mut self) -> usize {
        match *self.declaration.clone(){
            Stmt::FunctionStmt(stmt) => {
                return stmt.params.len();
            }
            _ => {
                panic!("must be func stmt");
            }
        }
    }

    fn call(&mut self, inter: &mut Interpreter, params: &Vec<Tokenliteral>) -> Result<Tokenliteral, LoxError> {
        let env = Rc::new(RefCell::new(Environment::new_with_visitor(&self.closure)));

        match *self.declaration.clone(){
            Stmt::FunctionStmt(stmt) => {
                for (idx, tok) in stmt.params.iter().enumerate() {
                    let v = params.get(idx).unwrap();
                    env.borrow_mut().define(tok, v);
                }
            }
            _ => {
                unreachable!("must be func stmt");
            }
        }

        match *self.declaration.clone() {
            Stmt::FunctionStmt(stmt) => {
                let ret = inter.execute_block(&stmt.body, &env);
                match ret {
                    Ok(v) => {
                        return Ok(v);
                    }
                    Err(LoxError::Return(v)) => {
                        return Ok(v)
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
                
            }
            _ => {
                unreachable!("must be func stmt");
            }
        }    
    }
}

