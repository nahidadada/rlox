use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::{Log, LoxError};
use crate::loxfunction::{LoxCallable, LoxFunction};
use crate::stmt::{Stmt, StmtVisitor};
use crate::token_type::TokenType;
use crate::{
    expr::{Expr, ExprVisitor},
    token::Tokenliteral,
};

pub struct Interpreter<'a> {
    errors: &'a mut Log,
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter<'_> {
    pub fn new(log: &mut Log) -> Interpreter {
        Interpreter {
            errors: log,
            globals: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Result<Stmt, LoxError>>) {
        for stmt in statements.iter() {
            match stmt {
                Ok(s) => {
                    let _ret = self.execute(s);
                    if self.errors.had_runtime_error() {
                        break;
                    }
                }
                Err(_e) => {
                    break;
                }
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Tokenliteral, LoxError> {
        return stmt.accept(self);
    }

    pub fn execute_block(&mut self, stmt: &Vec<Box<Stmt>>, env: & Rc<RefCell<Environment>>) -> Result<Tokenliteral, LoxError> {
        let previous = Rc::clone(&self.environment);

        self.environment = Rc::clone(env);

        let mut ret = Tokenliteral::Nil;
        for elem in stmt.iter() {
            ret = self.execute(elem)?;
        }

        self.environment = Rc::clone(&previous);

        return Ok(ret);
    }

    fn evalute(&mut self, expr: &Expr) -> Result<Tokenliteral, LoxError> {
        return expr.accept(self);
    }
    fn is_truthy(&self, literal: &Tokenliteral) -> bool {
        match literal {
            Tokenliteral::Lstirng(_) => return true,
            Tokenliteral::Lnumber(_) => return true,
            Tokenliteral::Lbool(v) => return *v,
            Tokenliteral::Nil => return false,
            Tokenliteral::LCall(_) => return true,
        }
    }
    fn is_equal(&self, left: &Tokenliteral, right: &Tokenliteral) -> bool {
        let ret = self.is_number_equal(left, right);
        if ret.is_some() {
            return ret.unwrap();
        }
        let ret = self.is_string_equal(left, right);
        if ret.is_some() {
            return ret.unwrap();
        }
        return false;
    }
    fn is_number_equal(&self, left: &Tokenliteral, right: &Tokenliteral) -> Option<bool> {
        if let Tokenliteral::Lnumber(vleft) = left {
            if let Tokenliteral::Lnumber(vright) = right {
                return Some(vleft.eq(vright));
            }
        }
        return None;
    }
    fn is_string_equal(&self, left: &Tokenliteral, right: &Tokenliteral) -> Option<bool> {
        if let Tokenliteral::Lstirng(vleft) = left {
            if let Tokenliteral::Lstirng(vright) = right {
                return Some(vleft.eq(vright));
            }
        }
        return None;
    }
}

impl ExprVisitor for Interpreter<'_> {
    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> Result<Tokenliteral, LoxError> {
        let value = self.evalute(&expr.value)?;
        self.environment.borrow_mut().assign(&expr.name, &value)?;
        return Ok(value);
    }

    fn visit_binary_expr(&mut self, expr: &crate::expr::Binary) -> Result<Tokenliteral, LoxError> {
        let left = self.evalute(&*expr.left)?;
        let right = self.evalute(&*expr.right)?;

        let ret = match (&left, &right) {
            (Tokenliteral::Lnumber(vleft), Tokenliteral::Lnumber(vright)) => {
                match expr.operator.token_type {
                    TokenType::Minus => Ok(Tokenliteral::Lnumber(vleft - vright)),
                    TokenType::Slash => Ok(Tokenliteral::Lnumber(vleft / vright)),
                    TokenType::Star => Ok(Tokenliteral::Lnumber(vleft * vright)),
                    TokenType::Plus => Ok(Tokenliteral::Lnumber(vleft + vright)),
                    TokenType::Greater => Ok(Tokenliteral::Lbool(vleft > vright)),
                    TokenType::GreaterEqual => Ok(Tokenliteral::Lbool(vleft >= vright)),
                    TokenType::Less => Ok(Tokenliteral::Lbool(vleft < vright)),
                    TokenType::LessEqual => Ok(Tokenliteral::Lbool(vleft <= vright)),
                    TokenType::BangEqual => Ok(Tokenliteral::Lbool(!self.is_equal(&left, &right))),
                    TokenType::EqualEqual => Ok(Tokenliteral::Lbool(self.is_equal(&left, &right))),
                    _ => Err(LoxError::RuntimeError(
                        expr.operator.clone(),
                        "operator error".to_string(),
                    )),
                }
            }
            (Tokenliteral::Lstirng(vleft), Tokenliteral::Lstirng(vright)) => {
                let mut s = vleft.clone();
                s.push_str(&vright);
                Ok(Tokenliteral::Lstirng(s))
            }
            _ => Err(LoxError::RuntimeError(
                expr.operator.clone(),
                "operands must be two numbers or two strings".to_string(),
            )),
        };
        return ret;
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> Result<Tokenliteral, LoxError> {
        let callee = self.evalute(&expr.callee)?;

        let mut arguments = Vec::new();
        for arg in expr.arguments.iter() {
            let literal = self.evalute(arg)?;
            arguments.push(literal);
        }
    
        match callee {
            Tokenliteral::LCall(mut call) => {
                if arguments.len() != call.arity() {
                    return Err(LoxError::RuntimeError(expr.paren.clone(), "arg size must match".to_string()));
                }
                return call.call(self, &arguments);
            }
            _ => {
                return Err(LoxError::RuntimeError(expr.paren.clone(), "must be func".to_string()));
            }
        }
    }

    fn visit_get_expr(&mut self, _expr: &crate::expr::Get) {
        todo!()
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &crate::expr::Grouping,
    ) -> Result<Tokenliteral, LoxError> {
        return self.evalute(&expr.expression);
    }

    fn visit_literal_expr(
        &mut self,
        expr: &crate::expr::Literal,
    ) -> Result<Tokenliteral, LoxError> {
        return Ok(expr.value.clone());
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::Logical) -> Result<Tokenliteral, LoxError> {
        let left = self.evalute(&expr.left)?;

        match &expr.operator.token_type {
            TokenType::Or => {
                if self.is_truthy(&left) {
                    return Ok(left);
                }
            }
            TokenType::And => {
                if !self.is_truthy(&left) {
                    return Ok(left);
                }
            }
            _ => {
                return Err(LoxError::RuntimeError(expr.operator.clone(), "logical operator error".to_string()));
            }
        }
        return self.evalute(&expr.right);
    }

    fn visit_set_expr(&mut self, _expr: &crate::expr::Set) {
        todo!()
    }

    fn visit_super_expr(&mut self, _expr: &crate::expr::Super) {
        todo!()
    }

    fn visit_this_expr(&mut self, _expr: &crate::expr::This) {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> Result<Tokenliteral, LoxError> {
        let right = self.evalute(&expr.right)?;
        let ret = match expr.operator.token_type {
            TokenType::Minus => match right {
                Tokenliteral::Lnumber(v) => Ok(Tokenliteral::Lnumber(-v)),
                _ => Err(LoxError::RuntimeError(
                    expr.operator.clone(),
                    "Operand must be a number".to_string(),
                )),
            },
            TokenType::Bang => Ok(Tokenliteral::Lbool(!self.is_truthy(&right))),
            _ => Err(LoxError::RuntimeError(
                expr.operator.clone(),
                "wrong operator".to_string(),
            )),
        };
        return ret;
    }

    fn visit_variable_expr(
        &mut self,
        expr: &crate::expr::Variable,
    ) -> Result<Tokenliteral, LoxError> {
        return self.environment.borrow().get(&expr.name);
    }
}

impl StmtVisitor for Interpreter<'_> {
    fn visit_block_stmt(&mut self, stmt: &crate::stmt::Block) -> Result<Tokenliteral, LoxError> {
         let mut env = Rc::new(RefCell::new(Environment::new_with_visitor(&Rc::clone(&self.environment))));
         return self.execute_block(&stmt.statements, &env);
    }

    fn visit_class_stmt(&mut self, stmt: &crate::stmt::Class) -> Result<Tokenliteral, LoxError> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::Expression) -> Result<Tokenliteral, LoxError> {
        return self.evalute(&stmt.expression);
    }

    fn visit_function_stmt(&mut self, stmt: &crate::stmt::Function) -> Result<Tokenliteral, LoxError> {
        let function = LoxFunction::new(&stmt.name.lexeme, &Box::new(Stmt::FunctionStmt(stmt.clone())));
        self.environment.borrow_mut().define(&stmt.name, &Tokenliteral::LCall(function.clone()));
        return Ok(Tokenliteral::Nil);
    }

    fn visit_if_stmt(&mut self, stmt: &crate::stmt::If) -> Result<Tokenliteral, LoxError> {
        let result = self.evalute(&stmt.condition);
        let mut ret = Tokenliteral::Nil;

        if result.is_ok() {
            if self.is_truthy(&result.unwrap()) {
                ret = self.execute(&stmt.then_branch)?;
            } else if let Some(ref else_br) = stmt.else_branch {
                ret = self.execute(else_br)?;
            }
        }
        return Ok(ret);
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::Print) -> Result<Tokenliteral, LoxError> {
        let value = self.evalute(&stmt.expression);
        match value {
            Ok(token) => {
                println!("{}", token);
            }
            Err(e) => {
                self.errors.runtime_error(&e);
            }
        }
        Ok(Tokenliteral::Nil)
    }

    fn visit_return_stmt(&mut self, stmt: &crate::stmt::Return) -> Result<Tokenliteral, LoxError> {
        let mut value = Tokenliteral::Nil;
        if let Some(expr) = &stmt.value {
            value = self.evalute(&expr)?;
        }
        return Err(LoxError::Return(value));
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::Var) -> Result<Tokenliteral, LoxError> {
        let mut value = Ok(Tokenliteral::Nil);
        match *stmt.initializer {
            Expr::Nil => {}
            _ => {
                value = self.evalute(&stmt.initializer);
            }
        }

        if let Ok(v) = value {
            self.environment.borrow_mut().define(&stmt.name, &v);
        } else {
            println!("visit var stmt error");
        }
        Ok(Tokenliteral::Nil)
    }

    fn visit_while_stmt(&mut self, stmt: &crate::stmt::While) -> Result<Tokenliteral, LoxError> {
        let mut ret  = Tokenliteral::Nil;
        loop {
            let result = self.evalute(&stmt.condition);
            match result {
                Ok(literal) => {
                    if self.is_truthy(&literal) {
                        ret = self.execute(&stmt.body)?;
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
        return Ok(ret);
    }
}
