use crate::token_type::*;
use crate::tokens::*;

#[derive(Debug, Clone)]
pub struct LoxError {
    token: Option<Token>,
    line: usize,
    msg: String,
}

impl LoxError {

    pub fn error(token: Option<Token>, msg: String) -> LoxError {
        let mut err: LoxError;
        let mut line = 0;
        if let Some(t) = token {
            line = t.line;
            let token = Some(t);
            err = LoxError { token, line, msg };
            err.report("".to_string());
            return err;
        }

        err = LoxError { token, line, msg };
        err.report("".to_string());
        err
    }

    pub fn report(&mut self, loc: String) {
        if let Some(token) = &self.token {
            if token.is(TokenType::Eof) {
                eprintln!("{} as end {}", self.line, self.msg)
            } else {
                eprintln!("{} at '{}' {}", self.line, token.lexeme, self.msg)
            }
        } else {
            eprintln!("[line {}] error{}: {}", self.line, loc, self.msg)
        }
    }
}
