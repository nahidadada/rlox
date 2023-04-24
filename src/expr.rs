use crate::errors::LoxError;
use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::token::Tokenliteral;

pub trait ExprVisitor {
    fn visit_assign_expr(&self, expr: Assign);
    fn visit_binary_expr(&self, expr: &Binary) -> Result<Tokenliteral, LoxError>;
    fn visit_call_expr(&self, expr: Call);
    fn visit_get_expr(&self, expr: Get);
    fn visit_grouping_expr(&self, expr: &Grouping) -> Result<Tokenliteral, LoxError>;
    fn visit_literal_expr(&self, expr: &Literal) -> Result<Tokenliteral, LoxError>;
    fn visit_logical_expr(&self, expr: Logical);
    fn visit_set_expr(&self, expr: Set);
    fn visit_super_expr(&self, expr: Super);
    fn visit_this_expr(&self, expr: This);
    fn visit_unary_expr(&self, expr: &Unary) -> Result<Tokenliteral, LoxError>;
    fn visit_variable_expr(&self, expr: Variable);
}

//////////////////////
#[derive(Debug, Clone)]
pub struct Assign {
    name: Token,
    value: Box<Expr>,
}
impl Assign {
    pub fn new(name: &Token, value: &Expr) -> Assign {
        Assign {
            name: name.clone(),
            value: Box::new(value.clone()),
        }
    }
}

////////////////////////
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Binary {
    pub fn new(left: &Expr, op: &Token, right: &Expr) -> Binary {
        Binary {
            left: Box::new(left.clone()),
            operator: op.clone(),
            right: Box::new(right.clone()),
        }
    }
}

/////////////////////////
#[derive(Debug, Clone)]
pub struct Call {
    callee: Box<Expr>,
    paren: Token,
    arguments: Vec<Expr>,
}
impl Call {
    pub fn new(callee: &Expr, paren: &Token, args: &Vec<Expr>) -> Call {
        Call {
            callee: Box::new(callee.clone()),
            paren: paren.clone(),
            arguments: args.clone(),
        }
    }
}

///////////////////////////
#[derive(Debug, Clone)]
pub struct Get {
    object: Box<Expr>,
    name: Token,
}
impl Get {
    pub fn new(obj: &Expr, name: &Token) -> Get {
        Get {
            object: Box::new(obj.clone()),
            name: name.clone(),
        }
    }
}

/////////////////////////////
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}
impl Grouping {
    pub fn new(expr: &Expr) -> Grouping {
        Grouping {
            expression: Box::new(expr.clone()),
        }
    }
}

////////////////////////////////
#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Tokenliteral,
}
impl Literal {
    pub fn new(s: &Tokenliteral) -> Literal {
        Literal { value: s.clone() }
    }
}

//////////////////////////////////
#[derive(Debug, Clone)]
pub struct Logical {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}
impl Logical {
    pub fn new(left: &Expr, op: &Token, right: &Expr) -> Logical {
        Logical {
            left: Box::new(left.clone()),
            operator: op.clone(),
            right: Box::new(right.clone()),
        }
    }
}

/////////////////////////////////
#[derive(Debug, Clone)]
pub struct Set {
    object: Box<Expr>,
    name: Token,
    value: Box<Expr>,
}
impl Set {
    pub fn new(obj: &Expr, name: &Token, value: &Expr) -> Set {
        Self {
            object: Box::new(obj.clone()),
            name: name.clone(),
            value: Box::new(value.clone()),
        }
    }
}

/////////////////////////////////
#[derive(Debug, Clone)]
pub struct Super {
    keyword: Token,
    method: Token,
}
impl Super {
    pub fn new(keywork: &Token, method: &Token) -> Super {
        Super {
            keyword: keywork.clone(),
            method: method.clone(),
        }
    }
}

/////////////////////////////////
#[derive(Debug, Clone)]
pub struct This {
    keyword: Token,
}
impl This {
    pub fn new(keyword: &Token) -> This {
        This {
            keyword: keyword.clone(),
        }
    }
}

//////////////////////////////////
#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Unary {
    pub fn new(op: &Token, right: &Expr) -> Unary {
        Unary {
            operator: op.clone(),
            right: Box::new(right.clone()),
        }
    }
}

///////////////////////////////////
#[derive(Debug, Clone)]
pub struct Variable {
    name: Token,
}
impl Variable {
    pub fn new(name: &Token) -> Variable {
        Variable { name: name.clone() }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    AssignExpr(Assign),
    BinaryExpr(Binary),
    CallExpr(Call),
    GetExpr(Get),
    GroupingExpr(Grouping),
    LiteralExpr(Literal),
    LogicalExpr(Logical),
    SetExpr(Set),
    SuperExpr(Super),
    ThisExpr(This),
    UnaryExpr(Unary),
    VariableExpr(Variable),
    NoSense,
}

impl Expr {
    pub fn accept(&self, inter: &Interpreter) -> Result<Tokenliteral, LoxError> {
        match self {
            Expr::BinaryExpr(binary) => {
                return inter.visit_binary_expr(binary);
            }
            Expr::GroupingExpr(group) => {
                return inter.visit_grouping_expr(group);
            }
            Expr::LiteralExpr(literal) => {
                return inter.visit_literal_expr(literal);
            }
            Expr::UnaryExpr(unary) => {
                return inter.visit_unary_expr(unary);
            }
            _ => {
                return Ok(Tokenliteral::Nil);
            }
        }
    }
}