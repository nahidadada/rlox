use crate::{token::Token, expr::Expr, interpreter::Interpreter};

pub trait StmtVisitor {
    fn visit_block_stmt(&self, stmt: &Block);
    fn visit_class_stmt(&self, stmt: &Class);
    fn visit_expression_stmt(&self, stmt: &Expression);
    fn visit_function_stmt(&self, stmt: &Function);
    fn visit_if_stmt(&self, stmt: &If);
    fn visit_print_stmt(&mut self, stmt: &Print);
    fn visit_return_stmt(&self, stmt: &Return);
    fn visit_var_stmt(&mut self, stmt: &Var);
    fn visit_while_stmt(&self, stmt: &While);

}
///////////////////////
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Box<Stmt>>,
}
impl Block {
}

///////////////////////
#[derive(Debug, Clone)]
pub struct Class {
    name: Token,
    superclass: Box<Expr>,
    methods: Function,
}
impl Class {

}

/////////////////////////
#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Box<Expr>,
}
impl Expression {
    pub fn new(expr: &Expr) -> Expression {
        Expression {
            expression: Box::new(expr.clone()),
        }
    }
}

////////////////////////////
#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Box<Stmt>>,
}
impl Function {

}

///////////////////////////////
#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Box<Stmt>,
}
impl If {

}

/////////////////////////////////
#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Box<Expr>,
}
impl Print {
    pub fn new(expr: &Expr) -> Print {
        Print {
            expression: Box::new(expr.clone()),
        }
    }
}

/////////////////////////////////
#[derive(Debug, Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Box<Expr>,
}
impl Return {

}

//////////////////////////////////
#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Box<Expr>,
}
impl Var {
    pub fn new(name: &Token, initializer: &Expr) -> Var {
        Var {
            name: name.clone(),
            initializer: Box::new(initializer.clone()),
        }
    }
}

///////////////////////////////////
#[derive(Debug, Clone)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}
impl While {

}

///////////////////////////////////
#[derive(Debug, Clone)]
pub enum Stmt {
    BlockStmt(Block),
    ClassStmt(Class),
    ExpressionStmt(Expression),
    FunctionStmt(Function),
    IfStmt(If),
    PrintStmt(Print),
    ReturnStmt(Return),
    VarStmt(Var),
    WhileStmt(While),
}
impl Stmt {
    pub fn accept(&self, intr: &mut Interpreter) {
        match self {
            Stmt::BlockStmt(_) => todo!(),
            Stmt::ClassStmt(_) => todo!(),
            Stmt::ExpressionStmt(stmt) => {
                intr.visit_expression_stmt(stmt);
            },
            Stmt::FunctionStmt(_) => todo!(),
            Stmt::IfStmt(_) => todo!(),
            Stmt::PrintStmt(stmt) => {
                intr.visit_print_stmt(stmt);
            },
            Stmt::ReturnStmt(_) => todo!(),
            Stmt::VarStmt(stmt) => {
                intr.visit_var_stmt(stmt);
            },
            Stmt::WhileStmt(_) => todo!(),
        }
    }
}