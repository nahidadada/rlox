use crate::{token::{Tokenliteral}, interpreter::Interpreter, stmt::Stmt, environment::Environment, errors::LoxError};

pub trait LoxCallable {
    fn arity(&mut self) -> usize;
    fn call(&mut self, inter: &mut Interpreter, params: &Vec<Tokenliteral>) -> Result<Tokenliteral, LoxError>;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub name: String,
    pub declaration: Box<Stmt>,
}

impl LoxFunction {
    pub fn new(name: &str, declaration: &Box<Stmt>) -> LoxFunction {
        LoxFunction {
            name: name.to_string(),
            declaration: declaration.clone(),
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
        let mut env = Environment::new_with_visitor(&inter.globals);

        match *self.declaration.clone(){
            Stmt::FunctionStmt(stmt) => {
                for (idx, tok) in stmt.params.iter().enumerate() {
                    let v = params.get(idx).unwrap();
                    env.define(tok, &v);
                }
            }
            _ => {
                unreachable!("must be func stmt");
            }
        }

        match *self.declaration.clone() {
            Stmt::FunctionStmt(stmt) => {
                inter.execute_block(&stmt.body, &mut env)
            }
            _ => {
                unreachable!("must be func stmt");
            }
        }    
        return Ok(Tokenliteral::Nil);
    }
}

