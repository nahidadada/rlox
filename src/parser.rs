use crate::{token::Token, token_type::TokenType};
use crate::token_type::TokenType::*;
use crate::token::Tokenliteral;
use crate::expr::*;

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

    pub fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.is_match(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::BinaryExpr(Binary::new(&expr, &operator, &right));
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.is_match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::BinaryExpr(Binary::new(&expr, &op, &right));
        }

        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.is_match(&[Minus, Plus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::BinaryExpr(Binary::new(&expr, &op, &right));
        }

        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.is_match(&[Slash, Star]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::BinaryExpr(Binary::new(&expr, &op, &right));
        }

        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.is_match(&[Bang, Minus]) {
            let op = self.previous();
            let right = self.unary();
            return Expr::UnaryExpr(Unary::new(&op, &right));
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.is_match(&[False]) {
            return Expr::LiteralExpr(Literal::new(&Tokenliteral::Lbool(false)));
        }
        if self.is_match(&[True]) {
            return Expr::LiteralExpr(Literal::new(&Tokenliteral::Lbool(true)));
        }
        if self.is_match(&[Nils]) {
            return Expr::LiteralExpr(Literal::new(&Tokenliteral::Nil));
        }

        if self.is_match(&[Number, Strings]) {
            return Expr::LiteralExpr(Literal::new(&self.previous().literal));
        }

        if self.is_match(&[LeftParen]) {
            let expr = self.expression();
            self.consume(&RightParen, "Expect ')' after expression.");
            return Expr::GroupingExpr(Grouping::new(&expr));
        }

        //////////////////////////////
        panic!();
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

    fn consume(&mut self, tp: &TokenType, msg: &str) -> Token {
        if self.check(tp) {
            return self.advance();
        }

        panic!("{}", msg);
    }
}