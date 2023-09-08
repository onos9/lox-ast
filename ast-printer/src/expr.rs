use crate::error::*;
use crate::tokens::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn eccept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Binary(be) => be.eccept(visitor),
            Expr::Grouping(g) => g.eccept(visitor),
            Expr::Unary(un) => un.eccept(visitor),
            Expr::Literal(le) => le.eccept(visitor),
        }
    }
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visitor_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visitor_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visitor_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visitor_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

impl BinaryExpr {
    pub fn eccept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visitor_binary_expr(self)
    }
}

impl GroupingExpr {
    fn eccept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visitor_grouping_expr(self)
    }
}

impl LiteralExpr {
    fn eccept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visitor_literal_expr(self)
    }
}

impl UnaryExpr {
    fn eccept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, LoxError> {
        visitor.visitor_unary_expr(self)
    }
}
