use std::cell::RefCell;
use std::rc::Rc;

use crate::{token::Token, token_type::TokenType};
use crate::token::Tokenliteral;
use crate::expr::{Expr, self};
use crate::stmt::{self, Stmt};
use crate::errors::LoxError;
use crate::errors::Log;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Rc<RefCell<Log>>,
}

impl Parser {
    pub fn new(tokens: &Vec<Token>, log: &Rc<RefCell<Log>>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            current: 0,
            errors: Rc::clone(log),
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
        if self.is_match(&[TokenType::Class]) {
            return self.class_declaration();
        }

        if self.is_match(&[TokenType::Fun]) {
            let ret = self.function("function");
            if ret.is_err() {
                self.synchronize();
            }
            return ret;
        }

        if self.is_match(&[TokenType::Var]) {
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

    fn class_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(&TokenType::Identifier, "Expect class name.")?;
        self.consume(&TokenType::LeftBrace, "Expect '{' before class body.")?;

        let mut methods = Vec::new();
        while !self.check(&TokenType::RightBrace) && ! self.is_at_end() {
            let f = self.function("method")?;
            match f {
                Stmt::FunctionStmt(v) => {
                    methods.push(v);
                }
                _ => {
                    println!("should be func stmt");
                }
            }
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after class body.")?;
        return Ok(Stmt::ClassStmt(stmt::Class::new(&name, &methods)));
    }

    fn function(&mut self, msg: &str) -> Result<Stmt, LoxError> {
        let mut err_msg = "Expect ".to_string();
        err_msg.push_str(msg);
        err_msg.push_str(" name.");

        let name = self.consume(&TokenType::Identifier, &err_msg)?;

        err_msg.clear();
        err_msg = "Expect '(' after ".to_string();
        err_msg.push_str(msg);
        err_msg.push_str(" name.");
        self.consume(&TokenType::LeftParen, &err_msg)?;

        let mut parameters = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let _ret = self.error(&self.peek(), "Can't have more than 255 parameters.");
                }
                parameters.push(self.consume(&TokenType::Identifier, "Expect parameter name.")?);
                if !self.is_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightParen, "Expect ')' after parameters.")?;

        err_msg.clear();
        err_msg = "Expect '{' before ".to_string();
        err_msg.push_str(msg);
        err_msg.push_str(" body.");
        self.consume(&TokenType::LeftBrace, &err_msg)?;

        let body = self.block()?;
        return Ok(Stmt::FunctionStmt(stmt::Function::new(&name, &parameters, &body)));

    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name")?;
        
        let mut ret = Ok(Expr::Nil);
        if self.is_match(&[TokenType::Equal]) {
            ret = self.expression();
        }

        let initializer = ret?;

        self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        return Ok(Stmt::VarStmt(stmt::Var::new(&name, &initializer)));
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.is_match(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.is_match(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.is_match(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.is_match(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.is_match(&[TokenType::LeftBrace]) {
            let ret = self.block()?;
            return Ok(Stmt::BlockStmt(stmt::Block::new(&ret)));
        }
        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let mut initializer: Option<Stmt> = None;
        if self.is_match(&[TokenType::Semicolon]) {
        } else if self.is_match(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self.check(&TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(&TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Expr> = None;
        if !self.check(&TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(&TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if let Some(inc_expr) = increment {
            let arr = vec![Box::new(body.clone()), Box::new(Stmt::ExpressionStmt(stmt::Expression::new(&inc_expr)))];
            body = Stmt::BlockStmt(stmt::Block::new(&arr));
        }

        if condition.is_none() {
            condition = Some(Expr::LiteralExpr(expr::Literal::new(&Tokenliteral::Lbool(true))));
        }
        body = Stmt::WhileStmt(stmt::While::new(&condition.unwrap(), &body));

        if let Some(init_expr) = initializer {
            let arr = vec![Box::new(init_expr.clone()), Box::new(body.clone())];
            body = Stmt::BlockStmt(stmt::Block::new(&arr));
        }

        return Ok(body);
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'if'.").map(|_| Stmt::Nil)?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after if condition.").map(|_| Stmt::Nil)?;

        let then_branch = self.statement()?;
        let mut else_branch = Stmt::Nil;
        if self.is_match(&[TokenType::Else]) {
            else_branch = self.statement()?;
        }

        return Ok(Stmt::IfStmt(stmt::If::new(&condition, &then_branch, &else_branch)));
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value")?;
        Ok(Stmt::PrintStmt(stmt::Print::new(&value)))
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxError> {
        let keyword = self.previous();
        
        let mut value = Expr::Nil;
        if !self.check(&TokenType::Semicolon) {
            value = self.expression()?;
        }

        self.consume(&TokenType::Semicolon, "Expect ';' after return value.")?;
        return Ok(Stmt::ReturnStmt(stmt::Return::new(&keyword, &value)));        
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;
        return Ok(Stmt::WhileStmt(stmt::While::new(&condition, &body)));
    }

    fn block(&mut self) -> Result<Vec<Box<Stmt>>, LoxError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(Box::new(stmt));
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.")?;
        return Ok(statements);
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::ExpressionStmt(stmt::Expression::new(&expr)))
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            match &expr {
                Expr::VariableExpr(vars) => {
                    let name = &vars.name;
                    return Ok(Expr::AssignExpr(expr::Assign::new(name, &value)));
                }
                Expr::GetExpr(v) => {
                    return Ok(Expr::SetExpr(expr::Set::new(&v.object, &v.name, &value)));
                }
                _ => {
                    return self.error(&equals, "Invalid assignment target").map(|_| Expr::Nil);
                }
            }
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::Or]) {
            let op = self.previous();
            let right = self.and()?;
            expr = Expr::LogicalExpr(expr::Logical::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::And]) {
            let op = self.previous();
            let right = self.equality()?;
            expr = Expr::LogicalExpr(expr::Logical::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::BinaryExpr(expr::Binary::new(&expr, &operator, &right));
        }
        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.is_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let op = self.previous();
            let right = self.term()?;
            expr = Expr::BinaryExpr(expr::Binary::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let op = self.previous();
            let right = self.factor()?;
            expr = Expr::BinaryExpr(expr::Binary::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let op = self.previous();
            let right = self.unary()?;
            expr = Expr::BinaryExpr(expr::Binary::new(&expr, &op, &right));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::UnaryExpr(expr::Unary::new(&op, &right)));
        }
        return self.call();
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(&[TokenType::LeftParen]) {
                expr = self.finish_call(&expr)?;
            } else if self.is_match(&[TokenType::Dot]) {
                let name = self.consume(&TokenType::Identifier, 
                    "Expect property name after '.'.")?;
                expr = Expr::GetExpr(expr::Get::new(&expr, &name));
            } else {
                break;
            }
        }
        return Ok(expr);
    }

    fn finish_call(&mut self, callee: &Expr) -> Result<Expr, LoxError> {
        let mut arguments = Vec::new();
        
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let _e = self.error(&self.peek(), "Can't have more than 255 arguments.").map(|_| Expr::Nil);
                }
                arguments.push(Box::new(self.expression()?));
                if !self.is_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expect ')' after arguments.")?;
        return Ok(Expr::CallExpr(expr::Call::new(&callee, &paren, &arguments)));
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::LiteralExpr(expr::Literal::new(&Tokenliteral::Lbool(false))));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::LiteralExpr(expr::Literal::new(&Tokenliteral::Lbool(true))));
        }
        if self.is_match(&[TokenType::Nils]) {
            return Ok(Expr::LiteralExpr(expr::Literal::new(&Tokenliteral::Nil)));
        }

        if self.is_match(&[TokenType::Number, TokenType::Strings]) {
            let tok = self.previous();
            return Ok(Expr::LiteralExpr(expr::Literal::new(&tok.literal)));
        }

        if self.is_match(&[TokenType::This]) {
            let token = self.previous();
            return Ok(Expr::ThisExpr(expr::This::new(&token)));
        }

        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::VariableExpr(expr::Variable::new(&self.previous())));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::GroupingExpr(expr::Grouping::new(&expr)));
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
        return self.peek().token_type == TokenType::Eofs;
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
        self.errors.borrow_mut().token_error(&token, msg);
        return Err(LoxError::ParseError);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }
}