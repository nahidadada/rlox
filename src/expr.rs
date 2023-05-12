use crate::errors::LoxError;
use crate::token::Token;
use crate::token::Tokenliteral;

pub trait ExprVisitor {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<Tokenliteral, LoxError>;
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<Tokenliteral, LoxError>;
    fn visit_call_expr(&mut self, expr: &Call) -> Result<Tokenliteral, LoxError>;
    fn visit_get_expr(&mut self, expr: &Get);
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<Tokenliteral, LoxError>;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<Tokenliteral, LoxError>;
    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<Tokenliteral, LoxError>;
    fn visit_set_expr(&mut self, expr: &Set);
    fn visit_super_expr(&mut self, expr: &Super);
    fn visit_this_expr(&mut self, expr: &This);
    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<Tokenliteral, LoxError>;
    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<Tokenliteral, LoxError>;
}

//////////////////////
#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
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
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Box<Expr>>,
}
impl Call {
    pub fn new(callee: &Expr, paren: &Token, args: &Vec<Box<Expr>>) -> Call {
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
        Literal { 
            value: s.clone() 
        }
    }
}

//////////////////////////////////
#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
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
    pub name: Token,
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
    Nil,
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Expr {
    pub fn accept(&self, inter: &mut dyn ExprVisitor) -> Result<Tokenliteral, LoxError> {
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
            Expr::AssignExpr(assign) => {
                return inter.visit_assign_expr(assign);
            }
            Expr::CallExpr(call) => {
                return inter.visit_call_expr(call);
            }
            Expr::GetExpr(_) => {
                todo!();
            },
            Expr::LogicalExpr(expr) => {
                return inter.visit_logical_expr(expr);
            },
            Expr::SetExpr(_) => {
                todo!();
            },
            Expr::SuperExpr(_) => {
                todo!();
            },
            Expr::ThisExpr(_) => {
                todo!();
            },
            Expr::VariableExpr(expr) => {
                return inter.visit_variable_expr(expr);
            },
            Expr::Nil => {
                return Ok(Tokenliteral::Nil);
            },
        }
    }
}