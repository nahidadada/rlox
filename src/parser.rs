use crate::{token::Token, token_type::TokenType};
use crate::token_type::TokenType::*;
use crate::token::Tokenliteral;
use crate::expr::*;
use crate::stmt::{self, *};
use crate::errors::LoxError;
use crate::errors::Log;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    errors: &'a mut Log,
}

impl Parser<'_> {
    pub fn new<'a>(tokens: &'a Vec<Token>, log: &'a mut Log) -> Parser<'a> {
        Parser {
            tokens: tokens.clone(),
            current: 0,
            errors: log,
        }
    }

    pub fn parse(&mut self) -> Vec<Result<Stmt, LoxError>> {
        let mut result = Vec::new();
        while !self.is_at_end() {
            let ret = self.declaration();
            result.push(ret);
            
        }
        return result;
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(&[Var]) {
            let ret = self.var_declaration();
            if ret.is_err() {
                self.synchronize();
            } 
            return ret;
        }

        let ret = self.statement();
        if ret.is_err() {
            self.synchronize();
        }
        return ret;
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(&Identifier, "Expect variable name")?;
        
        let mut ret = Ok(Expr::Nil);
        if self.is_match(&[Equal]) {
            ret = self.expression();
        }

        let initializer = ret?;

        self.consume(&Semicolon, "Expect ';' after variable declaration.")?;
        return Ok(Stmt::VarStmt(stmt::Var::new(&name, &initializer)));
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(&[Print]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after value")?;
        Ok(Stmt::PrintStmt(stmt::Print::new(&value)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(&Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::ExpressionStmt(Expression::new(&expr)))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.is_match(&[Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match &expr {
                Expr::VariableExpr(vars) => {
                    let name = &vars.name;
                    return Ok(Expr::AssignExpr(Assign::new(name, &value)));
                }
                _ => {
                    return self.error(&equals, "Invalid assignment target").map(|_| Expr::Nil);
                }
            }
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::BinaryExpr(Binary::new(&expr, &operator, &right));
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::BinaryExpr(Binary::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[Minus, Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::BinaryExpr(Binary::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[Slash, Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::BinaryExpr(Binary::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[Bang, Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::UnaryExpr(Unary::new(&op, &right)));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[False]) {
            return Ok(Expr::LiteralExpr(Literal::new(&Tokenliteral::Lbool(false))));
        }
        if self.is_match(&[True]) {
            return Ok(Expr::LiteralExpr(Literal::new(&Tokenliteral::Lbool(true))));
        }
        if self.is_match(&[Nils]) {
            return Ok(Expr::LiteralExpr(Literal::new(&Tokenliteral::Nil)));
        }

        if self.is_match(&[Number, Strings]) {
            return Ok(Expr::LiteralExpr(Literal::new(&self.previous().literal)));
        }

        if self.is_match(&[Identifier]) {
            return Ok(Expr::VariableExpr(Variable::new(&self.previous())));
        }

        if self.is_match(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::GroupingExpr(Grouping::new(&expr)));
        }

        return self.error(&self.peek(), "Expect expression.")
            .map(|_| { Expr::Nil });
    }

    fn is_match(&mut self, types: &[TokenType]) -> bool {
        for tp in types {
            if self.check(tp) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, tp: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == tp.clone();
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == Eofs;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, tp: &TokenType, msg: &str) -> Result<Token, LoxError> {
        if self.check(tp) {
            return Ok(self.advance());
        }

        return self.error(&self.peek(), msg);
    }

    fn error(&mut self, token: &Token, msg: &str) -> Result<Token, LoxError> {
        self.errors.token_error(&token, msg);
        return Err(LoxError::ParseError);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}