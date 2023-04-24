use crate::errors::{LoxError, Log};
use crate::token_type::TokenType::*;
use crate::{
    expr::{Expr, Visitor},
    token::Tokenliteral,
};

pub struct Interpreter<'a> {
    errors: &'a mut Log,
}

impl Interpreter<'_> {
    pub fn new(log: &mut Log) -> Interpreter {
        Interpreter { errors: log }
    }

    pub fn interpret(&mut self, expr: &Expr) {
        let ret = self.evalute(expr);
        match ret {
            Ok(v) => {
                println!("{}", v);
            }
            Err(e) => {
                self.errors.runtime_error(e);
            }
        }
    }

    fn evalute(&self, expr: &Expr) -> Result<Tokenliteral, LoxError> {
        return expr.accept(self);
    }
    fn is_truthy(&self, literal: &Tokenliteral) -> bool {
        match literal {
            Tokenliteral::Lstirng(_) => return true,
            Tokenliteral::Lnumber(_) => return true,
            Tokenliteral::Lbool(v) => return *v,
            Tokenliteral::Nil => return false,
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

impl Visitor for Interpreter<'_> {
    fn visit_assign_expr(&self, _expr: crate::expr::Assign) {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &crate::expr::Binary) -> Result<Tokenliteral, LoxError> {
        let left = self.evalute(&*expr.left)?;
        let right = self.evalute(&*expr.right)?;

        let ret = match (&left, &right) {
            (Tokenliteral::Lnumber(vleft), Tokenliteral::Lnumber(vright)) => {
                match expr.operator.token_type {
                    Minus => {
                        Ok(Tokenliteral::Lnumber(vleft - vright))
                    }
                    Slash => {
                        Ok(Tokenliteral::Lnumber(vleft / vright))
                    }
                    Star => {
                        Ok(Tokenliteral::Lnumber(vleft * vright))
                    }
                    Plus => {
                        Ok(Tokenliteral::Lnumber(vleft + vright))
                    }
                    Greater => {
                        Ok(Tokenliteral::Lbool(vleft > vright))
                    }
                    GreaterEqual => {
                        Ok(Tokenliteral::Lbool(vleft >= vright))
                    }
                    Less => {
                        Ok(Tokenliteral::Lbool(vleft < vright))
                    }
                    LessEqual => {
                        Ok(Tokenliteral::Lbool(vleft <= vright))
                    }
                    BangEqual => {
                        Ok(Tokenliteral::Lbool(!self.is_equal(&left, &right)))
                    }
                    EqualEqual => {
                        Ok(Tokenliteral::Lbool(self.is_equal(&left, &right)))
                    }
                    _ => {
                        Err(LoxError::RuntimeError(expr.operator.clone(), "operator error".to_string()))
                    }        
                }
            }
            (Tokenliteral::Lstirng(vleft), Tokenliteral::Lstirng(vright)) => {
                let mut s = vleft.clone();
                s.push_str(&vright);
                Ok(Tokenliteral::Lstirng(s))
            }
            _ => {
                Err(LoxError::RuntimeError(expr.operator.clone(), "operator error".to_string()))
            }
        };
        return ret;
    }

    fn visit_call_expr(&self, _expr: crate::expr::Call) {
        todo!()
    }

    fn visit_get_expr(&self, _expr: crate::expr::Get) {
        todo!()
    }

    fn visit_grouping_expr(&self, expr: &crate::expr::Grouping) -> Result<Tokenliteral, LoxError> {
        return self.evalute(&expr.expression);
    }

    fn visit_literal_expr(&self, expr: &crate::expr::Literal) -> Result<Tokenliteral, LoxError> {
        return Ok(expr.value.clone());
    }

    fn visit_logical_expr(&self, _expr: crate::expr::Logical) {
        todo!()
    }

    fn visit_set_expr(&self, _expr: crate::expr::Set) {
        todo!()
    }

    fn visit_super_expr(&self, _expr: crate::expr::Super) {
        todo!()
    }

    fn visit_this_expr(&self, _expr: crate::expr::This) {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &crate::expr::Unary) -> Result<Tokenliteral, LoxError> {
        let right = self.evalute(&expr.right)?;
        let ret = match expr.operator.token_type {
            Minus => match right {
                Tokenliteral::Lnumber(v) => Ok(Tokenliteral::Lnumber(-v)),
                _ => {
                    Err(LoxError::RuntimeError(expr.operator.clone(), "Operand must be a number".to_string()))
                }
            },
            Bang => Ok(Tokenliteral::Lbool(!self.is_truthy(&right))),
            _ => {
                Err(LoxError::RuntimeError(expr.operator.clone(), "wrong operator".to_string()))
            }
        };
        return ret;
    }

    fn visit_variable_expr(&self, _expr: crate::expr::Variable) {
        todo!()
    }
}
