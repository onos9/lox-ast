use crate::error::*;
use crate::token_type::TokenType;
use crate::tokens::*;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            let c = self.advance();
            if let Some(ttype) = self.match_token(c) {
                self.add_token(ttype, Some(Object::Nil));
                self.line += 1;
            }

            self.start = self.current;
            let _ = self.scan_tokens();
        }

        self.tokens.push(Token::eof(self.line));

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        let t: Vec<char> = self.source.chars().collect();
        t[self.current + 1]
    }

    fn add_token(&mut self, ttype: TokenType, literals: Option<Object>) {
        let lexeme = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect();

        self.tokens
            .push(Token::new(ttype, lexeme, literals, self.line));
    }

    fn check(&mut self, expected: char) -> bool {
        let t: Vec<char> = self.source.chars().collect();
        if self.is_at_end() {
            return false;
        }
        if t[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while (self.peek() != '"' && !self.is_at_end()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            LoxError::error(self.line, "Unterminated string.".to_string());
            return;
        }
        // The closing ".
        self.advance();

        let value = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 1)
            .collect();

        self.add_token(, value);
    }

    fn peek(&self) -> char {
        let t: Vec<char> = self.source.chars().collect();
        if self.is_at_end() {
            return '\0';
        }
        t[self.current]
    }

    fn match_token(&mut self, token: char) -> Option<TokenType> {
        match token {
            ' ' => None,
            '\n' => None,
            '\t' => None,
            '\r' => None,

            '.' => Some(TokenType::Dot),
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            '[' => Some(TokenType::LeftBracket),
            ']' => Some(TokenType::RightBracket),
            ',' => Some(TokenType::Comma),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),

            // Operators
            '!' => match self.check('=') {
                true => Some(TokenType::BangEqual),
                false => Some(TokenType::Bang),
            },
            '=' => match self.check('=') {
                true => Some(TokenType::EqualEqual),
                false => Some(TokenType::Equal),
            },
            '<' => match self.check('=') {
                true => Some(TokenType::LessEqual),
                false => Some(TokenType::Less),
            },
            '>' => match self.check('=') {
                true => Some(TokenType::GreaterEqual),
                false => Some(TokenType::Greater),
            },

            '/' => match self.check('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                }
                false => Some(TokenType::Slash),
            },

            c => Some(TokenType::Unknown(c)),
        }
    }
}
