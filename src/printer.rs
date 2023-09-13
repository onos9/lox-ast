
use crate::error::*;
use crate::expr::*;
use crate::token_type::*;
use crate::tokens::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.eccept(self)
    }

    pub fn parenthesize(&self, lexeme: &String, exprs: &[&Expr]) -> Result<String, LoxError> {
        let mut builder = format!("({lexeme}");
        for expr in exprs {
            builder = format!("{builder} {}", expr.eccept(self)?);
        }
        builder = format!("{builder})");

        Ok(builder)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visitor_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visitor_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize(&"group".to_string(), &[&expr.expression])
    }
    fn visitor_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
    fn visitor_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
}

pub fn _print() {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token {
                ttype: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        })),
        operator: Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(45.67)),
            })),
        })),
    });

    let printer = AstPrinter {};
    println!("{}", printer.print(&expression).unwrap());
}
