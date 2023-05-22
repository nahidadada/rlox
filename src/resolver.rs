use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::Log;
use crate::expr::Expr;
use crate::expr::ExprVisitor;
use crate::interpreter::Interpreter;
use crate::stmt::StmtVisitor;
use crate::stmt;
use crate::expr;
use crate::token::Token;
use crate::token::Tokenliteral;

#[derive(Clone, PartialEq)]
enum FuncType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone, PartialEq)]
enum ClassType {
    None, 
    Class,
}
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    log: Rc<RefCell<Log>>,
    current_function: FuncType,
    current_class: ClassType,
}

impl Resolver {
    pub fn new(inter: &Rc<RefCell<Interpreter>>, log: &Rc<RefCell<Log>>) -> Resolver {
        Resolver {
            interpreter: Rc::clone(inter),
            scopes: Vec::new(),
            log : Rc::clone(log),
            current_function: FuncType::None,
            current_class: ClassType::None,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    pub fn resolve_statement(&mut self, statements: &Vec<Box<stmt::Stmt>>) {
        for stmt in statements.iter() {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Box<stmt::Stmt>) {
        let ret = stmt.accept(self);
        match ret {
            Err(e) => {
                println!("resolve stmt : {:?}", e);
            }
            Ok(_v) => {}
        }
    }

    fn resolve_expr(&mut self, expr: &expr::Expr) {
        let ret = expr.accept(self);
        match ret {
            Err(e) => {
                println!("resolve expr : {:?}", e);
            }
            Ok(_v) => {}
        }
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let ret = self.scopes.last_mut();
        if let Some(scope) = ret {
            if scope.contains_key(&name.lexeme) {
                self.log.borrow_mut().error(name.line, "Already variable with this name in this scope.");
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    pub fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let ret = self.scopes.last_mut();
        if let Some(scope) = ret {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    pub fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        if self.scopes.is_empty() {
            println!("resolve_local: self.scopes is empty");
            return;
        }

        let arr_idx: Vec<usize> = (0..=self.scopes.len() - 1).into_iter().rev().collect();

        for &i in arr_idx.iter() {
            let ret = self.scopes.get(i);
            if let Some(v) = ret {
                if v.contains_key(&name.lexeme) {
                    self.interpreter.borrow_mut().resolve(expr, (self.scopes.len() - 1 - i) as i32);
                    return;
                }
            }
        }
    }

    fn resolve_func_stmt(&mut self, function: &stmt::Function, tp: FuncType) {
        let enclosing_function = self.current_function.clone();
        self.current_function = tp;

        self.begin_scope();

        for param in function.params.iter() {
            self.declare(param);
            self.define(param);
        }

        self.resolve_statement(&function.body);
        self.end_scope();

        self.current_function = enclosing_function;
    }

}

impl StmtVisitor for Resolver {
    fn visit_block_stmt(&mut self, stmt: &crate::stmt::Block) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.begin_scope();
        self.resolve_statement(&stmt.statements);
        self.end_scope();
        return Ok(Tokenliteral::Nil);
    }

    fn visit_class_stmt(&mut self, stmt: &crate::stmt::Class) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.begin_scope();
        let idx = self.scopes.len() - 1;
        let ret = self.scopes.get_mut(idx);
        if let Some(v) = ret {
            v.insert("this".to_string(), true);
        } else {
            println!("self.scope peek failed");
            return Err(crate::errors::LoxError::RuntimeError(stmt.name.clone(), "self.scope peek failed".to_string()));
        }
        
        for method in stmt.methods.iter() {
            let mut declaration = FuncType::Method;
            if method.name.lexeme.eq("init") {
                declaration = FuncType::Initializer;
            }
            self.resolve_func_stmt(method, declaration);
        }

        self.end_scope();
        self.current_class = enclosing_class;
        return Ok(Tokenliteral::Nil);
    }

    fn visit_expression_stmt(&mut self, stmt: &crate::stmt::Expression) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&stmt.expression);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_function_stmt(&mut self, stmt: &crate::stmt::Function) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_func_stmt(stmt, FuncType::Function);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_if_stmt(&mut self, stmt: &crate::stmt::If) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_stmt) = &stmt.else_branch {
            self.resolve_stmt(else_stmt);
        }
        return Ok(Tokenliteral::Nil);
    }

    fn visit_print_stmt(&mut self, stmt: &crate::stmt::Print) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&stmt.expression);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_return_stmt(&mut self, stmt: &crate::stmt::Return) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        if self.current_function == FuncType::None {
            self.log.borrow_mut().error(stmt.keyword.line, "Can't return from top-level code.");
        }        

        if let Some(v) = &stmt.value {
            if self.current_function == FuncType::Initializer {
                self.log.borrow_mut().error(stmt.keyword.line, "Can't return a value from an initializer.");
            }
            self.resolve_expr(v);
        }
        return Ok(Tokenliteral::Nil);
    }

    fn visit_var_stmt(&mut self, stmt: &crate::stmt::Var) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.declare(&stmt.name);
        match *stmt.initializer {
            expr::Expr::Nil => {
            }
            _ => {
                self.resolve_expr(&stmt.initializer);
            }
        }
        self.define(&stmt.name);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_while_stmt(&mut self, stmt: &crate::stmt::While) -> Result<crate::token::Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        return Ok(Tokenliteral::Nil);
    }
}

impl ExprVisitor for Resolver {
    fn visit_assign_expr(&mut self, expr: &expr::Assign) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.value); 
        self.resolve_local(&Expr::AssignExpr(expr.clone()) , &expr.name);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_binary_expr(&mut self, expr: &expr::Binary) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_call_expr(&mut self, expr: &expr::Call) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.callee);

        for arg in expr.arguments.iter() {
            self.resolve_expr(arg);
        }
        return Ok(Tokenliteral::Nil);
    }

    fn visit_get_expr(&mut self, expr: &expr::Get) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.object);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_grouping_expr(&mut self, expr: &expr::Grouping) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.expression);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_literal_expr(&mut self, _expr: &expr::Literal) -> Result<Tokenliteral, crate::errors::LoxError> {
        return Ok(Tokenliteral::Nil);
    }

    fn visit_logical_expr(&mut self, expr: &expr::Logical) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_set_expr(&mut self, expr: &expr::Set) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.value);
        self.resolve_expr(&expr.object);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_super_expr(&mut self, _expr: &expr::Super) {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &expr::This) -> Result<Tokenliteral, crate::errors::LoxError> {
        if self.current_class == ClassType::None {
            println!("Can't use 'this' outside of a class.");
            return Err(crate::errors::LoxError::RuntimeError(expr.keyword.clone(), "Can't use 'this' outside of a class.".to_string()));
        }

        self.resolve_local(&Expr::ThisExpr(expr.clone()), &expr.keyword);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_unary_expr(&mut self, expr: &expr::Unary) -> Result<Tokenliteral, crate::errors::LoxError> {
        self.resolve_expr(&expr.right);
        return Ok(Tokenliteral::Nil);
    }

    fn visit_variable_expr(&mut self, expr: &expr::Variable) -> Result<Tokenliteral, crate::errors::LoxError> {
        if !self.scopes.is_empty() {
            let scope = self.scopes.get(self.scopes.len() - 1).unwrap();
            let ret = scope.get(&expr.name.lexeme);
            if let Some(v) = ret {
                if !v {
                self.log.borrow_mut().error(expr.name.line,
                    "Can't read local variable in its own initializer.");
                }
            }
        }

        let var_expr = Expr::VariableExpr(expr.clone());
        self.resolve_local(&var_expr , &expr.name);
        return Ok(Tokenliteral::Nil);
    }
}