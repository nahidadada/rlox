use std::collections::HashMap;
use std::hash;

use crate::loxfunction::{LoxCallable, LoxFunction};
use crate::token::Tokenliteral;
use crate::errors::LoxError;
use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: &str, methods: &HashMap<String, LoxFunction>) -> LoxClass {
        LoxClass { 
            name: name.to_string(),
            methods: methods.clone(),
        }
    }

    pub fn find_method(&self, name: &str) -> Option<Tokenliteral> {
        let ret = self.methods.get(name);
        if let Some(v) = ret {
            return Some(Tokenliteral::LCall(v.clone()));
        }
        return None;
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        let ret = self.find_method("init");
        match ret {
            Some(v) => {
                match v {
                    Tokenliteral::LCall(f) => {
                        return f.arity();
                    }
                    _ => {
                        return 0;
                    }
                }
            }
            None => {
                return 0;
            }
        }
    }

    fn call(&mut self, 
        inter: &mut crate::interpreter::Interpreter, 
        params: &Vec<crate::token::Tokenliteral>) -> Result<Tokenliteral, LoxError> {
        let inst = LoxInstance::new(self);
        
        let ret = self.find_method("init");
        if let Some(initializer) = ret {
            match initializer {
                Tokenliteral::LCall(mut init) => {
                    let ret = init.bind(&inst);
                    if let Ok(v) = ret {
                        match v {
                            Tokenliteral::LCall(mut call) => {
                                let ret = call.call(inter, params);
                                if ret.is_err() {
                                    println!("construct obj failed");
                                }
                            }
                            _ => {
                            }
                        }
                    }

                }
                _ => {
                    println!("should be Tokenliteral::LCall");
                    panic!();
                }
            }
        }

        return Ok(Tokenliteral::LInst(inst));
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub klass: LoxClass,
    pub fields: HashMap<String, Tokenliteral>,
}

impl LoxInstance {
    pub fn new(klass: &LoxClass) -> LoxInstance {
        LoxInstance { 
            klass: klass.clone(),
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Tokenliteral, LoxError> {
        let ret = self.fields.get(&name.lexeme);
        if let Some(v) = ret {
            return Ok(v.clone());
        }

        let method = self.klass.find_method(&name.lexeme);
        if let Some(m) = method {
            match m {
                Tokenliteral::LCall(mut v) => {
                    return v.bind(self);
                }
                _ => {
                    println!("should be lox function");
                    return Err(LoxError::RuntimeError(name.clone(), "should be lox function".to_string()));
                }
            }
        }

        let mut msg = "Undefined property '".to_string();
        msg.push_str(&name.lexeme);
        msg.push_str("'.");
        return Err(LoxError::RuntimeError(name.clone(), msg));
    }

    pub fn set(&mut self, name: &Token, value: &Tokenliteral) {
        self.fields.insert(name.lexeme.clone(), value.clone());
    }
}