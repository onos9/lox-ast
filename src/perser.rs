use crate::error::*;
use crate::expr::*;
use crate::token_type::*;
use crate::tokens::*;

pub struct Perser {
    tokens: Vec<Token>,
    current: usize,
}

impl Perser {
    fn new(tokens: Vec<Token>) -> Perser {
        Perser { tokens, current: 0 }
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

        while self.is_match(&ttypes) {
            let operator = self.previous();
            let right = self.term()?;
            let expr = Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            });

            return Ok(expr);
        }

        let expr = self.primary()?;
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&vec![TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }

        if self.is_match(&vec![TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.is_match(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.is_match(&vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literals,
            }));
        }

        if self.is_match(&vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            )?;
            return Ok(expr);
        }

        Err(LoxError::error(0, "Unknown token type".to_string()))
    }

    fn consume(&mut self, ttype: TokenType, msg: String) -> Result<Token, LoxError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }

        Err(LoxError::error(self.peek().line, msg))
    }

    fn is_match(&mut self, ttypes: &Vec<TokenType>) -> bool {
        for ttype in ttypes {
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, ttype: TokenType) -> bool {
        if !self.is_at_end() {
            return false;
        }

        self.peek().ttype == ttype
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().ttype == TokenType::Eof
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
