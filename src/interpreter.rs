use crate::token_type::TokenType::*;
use crate::{
    expr::{Expr, Grouping, Visitor},
    token::Tokenliteral,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: &Expr) {
        let ret = self.evalute(expr);
        println!("{}", ret);
    }

    fn evalute(&self, expr: &Expr) -> Tokenliteral {
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

impl Visitor for Interpreter {
    fn visitAssignExpr(&self, expr: crate::expr::Assign) {
        todo!()
    }

    fn visitBinaryExpr(&self, expr: &crate::expr::Binary) -> Tokenliteral {
        let left = self.evalute(&*expr.left);
        let right = self.evalute(&*expr.right);

        let ret = match (&left, &right) {
            (Tokenliteral::Lnumber(vleft), Tokenliteral::Lnumber(vright)) => {
                match expr.operator.token_type {
                    Minus => {
                        Tokenliteral::Lnumber(vleft - vright)
                    }
                    Slash => {
                        Tokenliteral::Lnumber(vleft / vright)
                    }
                    Star => {
                        Tokenliteral::Lnumber(vleft * vright)
                    }
                    Plus => {
                        Tokenliteral::Lnumber(vleft + vright)
                    }
                    Greater => {
                        Tokenliteral::Lbool(vleft > vright)
                    }
                    GreaterEqual => {
                        Tokenliteral::Lbool(vleft >= vright)
                    }
                    Less => {
                        Tokenliteral::Lbool(vleft < vright)
                    }
                    LessEqual => {
                        Tokenliteral::Lbool(vleft <= vright)
                    }
                    BangEqual => {
                        Tokenliteral::Lbool(!self.is_equal(&left, &right))
                    }
                    EqualEqual => {
                        Tokenliteral::Lbool(self.is_equal(&left, &right))
                    }
                    _ => {
                        println!("match token type {:?} error", expr.operator);
                        Tokenliteral::Nil
                    }        
                }
            }
            (Tokenliteral::Lstirng(vleft), Tokenliteral::Lstirng(vright)) => {
                let mut s = vleft.clone();
                s.push_str(&vright);
                Tokenliteral::Lstirng(s)
            }
            _ => {
                println!("op + error, left={:?}, right={:?}", left, right);
                Tokenliteral::Nil
            }
        };
        return ret;
    }

    fn visitCallExpr(&self, expr: crate::expr::Call) {
        todo!()
    }

    fn visitGetExpr(&self, expr: crate::expr::Get) {
        todo!()
    }

    fn visitGroupingExpr(&self, expr: &crate::expr::Grouping) -> Tokenliteral {
        return self.evalute(&expr.expression);
    }

    fn visitLiteralExpr(&self, expr: &crate::expr::Literal) -> Tokenliteral {
        return expr.value.clone();
    }

    fn visitLogicalExpr(&self, expr: crate::expr::Logical) {
        todo!()
    }

    fn visitSetExpr(&self, expr: crate::expr::Set) {
        todo!()
    }

    fn visitSuperExpr(&self, expr: crate::expr::Super) {
        todo!()
    }

    fn visitThisExpr(&self, expr: crate::expr::This) {
        todo!()
    }

    fn visitUnaryExpr(&self, expr: &crate::expr::Unary) -> Tokenliteral {
        let right = self.evalute(&expr.right);
        let ret = match expr.operator.token_type {
            Minus => match right {
                Tokenliteral::Lnumber(v) => Tokenliteral::Lnumber(-v),
                _ => Tokenliteral::Nil,
            },
            Bang => Tokenliteral::Lbool(!self.is_truthy(&right)),
            _ => Tokenliteral::Nil,
        };
        return ret;
    }

    fn visitVariableExpr(&self, expr: crate::expr::Variable) {
        todo!()
    }
}
