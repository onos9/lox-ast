use crate::error::*;
use crate::expr::*;
use crate::token_type::*;
use crate::tokens::*;
use crate::object::*;

pub struct Perser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Perser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Perser {
        Perser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;
        let ttypes = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while self.is_match(&ttypes) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;
        let ttypes = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];

        while self.is_match(&ttypes) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;
        let ttypes = vec![TokenType::Minus, TokenType::Plus];

        while self.is_match(&ttypes) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;
        let ttypes = vec![TokenType::Slash, TokenType::Star];

        while self.is_match(&ttypes) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        let ttypes = vec![TokenType::Bang, TokenType::Minus];
        let mut expr = self.primary()?;

        while self.is_match(&ttypes) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }

        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }

        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            if self.consume(TokenType::RightParen).is_some() {
                let expr = Expr::Grouping(GroupingExpr {
                    expression: Box::new(self.expression()?),
                });

                return Ok(expr);
            }

            return Err(Perser::error(
                None,
                "Expect ')' after expression.".to_string(),
            ));
        }

        Err(Perser::error(None, "Expect an expression".to_string()))
    }

    fn consume(&mut self, ttype: TokenType) -> Option<Token> {
        if self.check(&ttype) {
            return Some(self.advance());
        }

        None
    }

    fn error(token: Option<Token>, msg: String) -> LoxError {
        LoxError::error(token, msg)
    }

    // fn synchronize(&mut self) {
    //     if self.previous().is(TokenType::Semicolon) {
    //         return;
    //     }

    //     while !self.is_at_end() {
    //         if matches!(
    //             self.peek().ttype,
    //             TokenType::Class
    //                 | TokenType::Fun
    //                 | TokenType::Var
    //                 | TokenType::For
    //                 | TokenType::If
    //                 | TokenType::While
    //                 | TokenType::Print
    //                 | TokenType::Return
    //         ) {
    //             return;
    //         }

    //         self.advance();
    //     }
    // }

    fn is_match(&mut self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().is(ttype)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is(&TokenType::Eof)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}
