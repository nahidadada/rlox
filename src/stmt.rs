use crate::{token::{Token, Tokenliteral}, expr::Expr, errors::LoxError};

pub trait StmtVisitor {
    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<Tokenliteral, LoxError>;
    fn visit_class_stmt(&mut self, stmt: &Class) -> Result<Tokenliteral, LoxError>;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<Tokenliteral, LoxError>;
    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<Tokenliteral, LoxError>;
    fn visit_if_stmt(&mut self, stmt: &If) -> Result<Tokenliteral, LoxError>;
    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<Tokenliteral, LoxError>;
    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<Tokenliteral, LoxError>;
    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<Tokenliteral, LoxError>;
    fn visit_while_stmt(&mut self, stmt: &While) -> Result<Tokenliteral, LoxError>;

}
///////////////////////
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Box<Stmt>>,
}
impl Block {
    pub fn new(statements: &Vec<Box<Stmt>>) -> Block {
        Block {
            statements: statements.clone(),
        }
    }
}

///////////////////////
#[derive(Debug, Clone)]
pub struct Class {
    pub name: Token,
    //pub superclass: Box<Expr>,
    pub methods: Vec<Function>,
}
impl Class {
    pub fn new(name: &Token, methods: &Vec<Function>) -> Class {
        Class {
            name: name.clone(),
            methods: methods.clone(),
        }
    }
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
    pub fn new(name: &Token, parameters: &Vec<Token>, body: &Vec<Box<Stmt>>) -> Function {
        Function { 
            name: name.clone(), 
            params: parameters.clone(), 
            body: body.clone() 
        }
    }
}

///////////////////////////////
#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
impl If {
    pub fn new(condition: &Expr, then_br: &Stmt, else_br: &Stmt) -> If {
        If {
            condition: Box::new(condition.clone()),
            then_branch: Box::new(then_br.clone()),
            else_branch: match else_br {
                Stmt::Nil => {
                    None
                }
                _ => {
                    Some(Box::new(else_br.clone()))
                }
            } ,
        }
    }
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
    pub value: Option<Box<Expr>>,
}
impl Return {
    pub fn new(keyword: &Token, value: &Expr) -> Return {
        Return {
            keyword: keyword.clone(),
            value: match value {
                Expr::Nil => {
                    None
                }
                _ => {
                    Some(Box::new(value.clone()))
                }
            }
        }
    }
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
    pub fn new(condition: &Expr, body: &Stmt) -> While {
        While { 
            condition: Box::new(condition.clone()), 
            body: Box::new(body.clone()) 
        }
    }
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
    Nil,
}
impl Stmt {
    pub fn accept(&self, intr: &mut dyn StmtVisitor) -> Result<Tokenliteral, LoxError> {
        let ret = match self {
            Stmt::BlockStmt(stmt) => {
                intr.visit_block_stmt(stmt)
            },
            Stmt::ClassStmt(stmt) => {
                intr.visit_class_stmt(stmt)
            },
            Stmt::ExpressionStmt(stmt) => {
                intr.visit_expression_stmt(stmt)
            },
            Stmt::FunctionStmt(stmt) => {
                intr.visit_function_stmt(stmt)
            },
            Stmt::IfStmt(stmt) => {
                intr.visit_if_stmt(stmt)
            },
            Stmt::PrintStmt(stmt) => {
                intr.visit_print_stmt(stmt)
            },
            Stmt::ReturnStmt(stmt) => {
                intr.visit_return_stmt(stmt)
            }
            Stmt::VarStmt(stmt) => {
                intr.visit_var_stmt(stmt)
            },
            Stmt::WhileStmt(stmt) => {
                intr.visit_while_stmt(stmt)
            },
            Stmt::Nil => {unimplemented!()}
        };
        return ret;
    }
}