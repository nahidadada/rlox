use std::{rc::Rc, cell::RefCell};

use crate::{token::{Tokenliteral, Token}, interpreter::Interpreter, stmt::Stmt, environment::Environment, errors::LoxError, loxclass::LoxInstance};
use crate::token_type::TokenType;

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(&mut self, inter: &mut Interpreter, params: &Vec<Tokenliteral>) -> Result<Tokenliteral, LoxError>;
}

#[derive(Clone, Debug)]
pub struct LoxFunction {
    pub name: String,
    pub declaration: Box<Stmt>,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(name: &str, 
        declaration: &Box<Stmt>, 
        closure: &Rc<RefCell<Environment>>,
        is_init: bool) -> LoxFunction {
        LoxFunction {
            name: name.to_string(),
            declaration: declaration.clone(),
            closure: Rc::clone(closure),
            is_initializer: is_init,
        }
    }

    pub fn bind(&mut self, instance: &LoxInstance) -> Result<Tokenliteral, LoxError> {
        let env = Rc::new(RefCell::new(Environment::new_with_visitor(&Rc::clone(&self.closure))));
        env.borrow_mut().define(
            &Token::new(TokenType::Nils, "this", &Tokenliteral::Nil, -1),
            &Tokenliteral::LInst(instance.clone()));
        let f = LoxFunction::new(
                &instance.klass.name, 
                &self.declaration, 
                &env,
                self.is_initializer);
        return Ok(Tokenliteral::LCall(f));
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
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
                        if self.is_initializer {
                            let ret = 
                                self.closure.borrow().get_at(0, &Token::new(TokenType::Nils, "this", &Tokenliteral::Nil, -1));
                            return ret;
                        }

                        return Ok(v);
                    }
                    Err(LoxError::Return(v)) => {
                        if self.is_initializer {
                            let ret = self.closure.borrow().get_at(0, &Token::new(TokenType::Nils, "this", &Tokenliteral::Nil, -1));
                            return ret;
                        }
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

