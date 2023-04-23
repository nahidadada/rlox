use crate::interpreter::Interpreter;
use crate::token::Token;
use crate::token::Tokenliteral;
//use crate::::interpreter::Interpreter;

pub trait Visitor {
    fn visitAssignExpr(&self, expr: Assign);
    fn visitBinaryExpr(&self, expr: &Binary) -> Tokenliteral;
    fn visitCallExpr(&self, expr: Call);
    fn visitGetExpr(&self, expr: Get);
    fn visitGroupingExpr(&self, expr: &Grouping) -> Tokenliteral;
    fn visitLiteralExpr(&self, expr: &Literal) -> Tokenliteral;
    fn visitLogicalExpr(&self, expr: Logical);
    fn visitSetExpr(&self, expr: Set);
    fn visitSuperExpr(&self, expr: Super);
    fn visitThisExpr(&self, expr: This);
    fn visitUnaryExpr(&self, expr: &Unary) -> Tokenliteral;
    fn visitVariableExpr(&self, expr: Variable);
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
    pub fn accept(&self, inter: &Interpreter) -> Tokenliteral {
        match self {
            Expr::BinaryExpr(binary) => {
                return inter.visitBinaryExpr(binary);
            }
            Expr::GroupingExpr(group) => {
                return inter.visitGroupingExpr(group);
            }
            Expr::LiteralExpr(literal) => {
                return inter.visitLiteralExpr(literal);
            }
            Expr::UnaryExpr(unary) => {
                return inter.visitUnaryExpr(unary);
            }
            _ => {
                return Tokenliteral::Nil;
            }
        }
    }
}