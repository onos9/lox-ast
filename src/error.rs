#[derive(Debug)]
pub struct LoxError {
    line: usize,
    msg: String,
}

impl LoxError {
    pub fn error(line: usize, msg: String) -> LoxError {
        LoxError { line, msg }
    }

    pub fn report(&mut self, loc: String) {
        eprint!("[line {}] error{}: {}", self.line, loc, self.msg)
    }
}
