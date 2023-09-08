mod error;
mod expr;
mod token_type;
mod tokens;

use error::*;
use expr::*;
use token_type::*;
use tokens::*;

struct AstPrinter;

impl AstPrinter {
    fn print(&self, expr: &Expr) -> Result<String, LoxError> {
        expr.eccept(self)
    }

    fn parenthesize(&self, lexeme: &String, exprs: &[&Box<Expr>]) -> Result<String, LoxError> {
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
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.left, &expr.right])
    }

    fn visitor_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize(&"group".to_string(), &vec![&expr.expression])
    }
    fn visitor_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        if let Some(value) = &expr.value {
            Ok(value.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
    fn visitor_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(&expr.operator.lexeme, &vec![&expr.right])
    }
}

fn main() {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token {
                ttype: TokenType::Minus,
                lexeme: "-".to_string(),
                literals: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Object::Num(123.0)),
            })),
        })),
        operator: Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            literals: None,
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
