use crate::error::*;
use crate::token_type::*;
use crate::tokens::*;
use crate::object::*;

#[derive(Debug)]
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            let (ttype, literals) = self.scan_token()?;
            match ttype {
                TokenType::Unknown => {}
                _ => self.add_token(ttype, literals),
            }
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let result = self.source[self.current];
        self.current += 1;
        result
    }

    fn add_token(&mut self, ttype: TokenType, literals: Option<Object>) {
        let lexeme = self.source[self.start..self.current].iter().collect();

        self.tokens
            .push(Token::new(ttype, lexeme, literals, self.line));
    }

    fn is_match(&mut self, expected: char) -> Option<()> {
        if self.source[self.current] != expected || self.is_at_end() {
            return None;
        }

        self.current += 1;
        Some(())
    }

    fn string(&mut self) -> Result<Object, LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::error(None, "Unterminated string".to_string()));
        }

        self.advance();
        let val: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        Ok(Object::Str(val))
    }

    fn number(&mut self) -> Result<Object, LoxError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let val: String = self.source[self.start..self.current].iter().collect();
        match val.parse::<f64>() {
            Ok(num) => Ok(Object::Num(num)),
            Err(_) => Err(LoxError::error(None, "Unable to parse number".to_string())),
        }
    }

    fn divider(&mut self, ch: char) -> Result<TokenType, LoxError> {
        if self.is_match(ch).is_some() {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            Ok(TokenType::Unknown)
        } else {
            Ok(TokenType::Slash)
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn identifier(&mut self, c: char) -> Result<TokenType, LoxError> {
        while self.peek().is_ascii_alphanumeric() || c == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        match self.keyword(text.as_str()) {
            Some(ttype) => Ok(ttype),
            None => Ok(TokenType::Identifier),
        }
    }

    //TODO: Use HashMap and the keywords Globaly Static
    fn keyword(&self, check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "let" => Some(TokenType::Let),
            "while" => Some(TokenType::While),
            "import" => Some(TokenType::Import),
            _ => None,
        }
    }

    fn scan_token(&mut self) -> Result<(TokenType, Option<Object>), LoxError> {
        let c = self.advance();
        match c {
            ' ' | '\t' | '\r' => Ok((TokenType::Unknown, None)),
            '\n' => {
                self.line += 1;
                Ok((TokenType::Unknown, None))
            }

            '.' => Ok((TokenType::Dot, None)),
            '(' => Ok((TokenType::LeftParen, None)),
            ')' => Ok((TokenType::RightParen, None)),
            '{' => Ok((TokenType::LeftBrace, None)),
            '}' => Ok((TokenType::RightBrace, None)),
            '[' => Ok((TokenType::LeftBracket, None)),
            ']' => Ok((TokenType::RightBracket, None)),
            ',' => Ok((TokenType::Comma, None)),
            '-' => Ok((TokenType::Minus, None)),
            '+' => Ok((TokenType::Plus, None)),
            ';' => Ok((TokenType::Semicolon, None)),
            '*' => Ok((TokenType::Star, None)),

            // Operators
            '!' => match self.is_match('=') {
                Some(_) => Ok((TokenType::BangEqual, None)),
                None => Ok((TokenType::Bang, None)),
            },
            '=' => match self.is_match('=') {
                Some(_) => Ok((TokenType::EqualEqual, None)),
                None => Ok((TokenType::Equal, None)),
            },
            '<' => match self.is_match('=') {
                Some(_) => Ok((TokenType::LessEqual, None)),
                None => Ok((TokenType::Less, None)),
            },
            '>' => match self.is_match('=') {
                Some(_) => Ok((TokenType::GreaterEqual, None)),
                None => Ok((TokenType::Greater, None)),
            },

            // TODO: handle excape sequence
            '"' => Ok((TokenType::String, Some(self.string()?))),
            '0'..='9' => Ok((TokenType::Number, Some(self.number()?))),
            '/' => Ok((self.divider('/')?, None)),

            _ if c.is_ascii_alphabetic() || c == '_' => Ok((self.identifier(c)?, None)),
            _ => Err(LoxError::error(None, "Unexpected character".to_string())),
        }
    }
}
