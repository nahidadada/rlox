use crate::{token::Token, token_type::TokenType};
use crate::token_type::TokenType::*;
use crate::token::Tokenliteral;
use crate::expr::*;
use crate::errors::LoxError;
use crate::errors::log;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        return self.expression();
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        return self.equality();
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

        if self.is_match(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(&RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::GroupingExpr(Grouping::new(&expr)));
        }

        return self.error(&self.peek(), "Expect expression.")
            .map(|_| { Expr::NoSense });
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

    fn error(&self, token: &Token, msg: &str) -> Result<Token, LoxError> {
        log::token_error(&token, msg);
        return Err(LoxError::ParseError);
    }

    #[allow(unreachable_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => {
                    return;
                }
                _ => {
                    println!("!!! NO IMPL HERE !!!");
                    unimplemented!();
                }
            }
            self.advance();
        }
    }
}