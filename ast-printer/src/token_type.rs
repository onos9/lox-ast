use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    Let,
    While,
    Import,

    // Other.
    Eof,
    // UnterminatedString,
    Unknown,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::LeftParen => "')'",
                TokenType::RightParen => "')'",
                TokenType::LeftBrace => "'{'",
                TokenType::RightBrace => "'}'",
                TokenType::LeftBracket => "'['",
                TokenType::RightBracket => "']'",
                TokenType::Comma => "','",
                TokenType::Dot => "'.'",
                TokenType::Minus => "'-'",
                TokenType::Plus => "'+'",
                TokenType::Semicolon => "';'",
                TokenType::Slash => "'/'",
                TokenType::Star => "'*'",
                TokenType::Bang => "'!'",
                TokenType::BangEqual => "'!='",
                TokenType::Equal => "'='",
                TokenType::EqualEqual => "'=='",
                TokenType::Greater => "'>'",
                TokenType::GreaterEqual => "'>='",
                TokenType::Less => "'<'",
                TokenType::LessEqual => "'<='",
                TokenType::Identifier => "identifier",
                TokenType::String => "string",
                TokenType::Number => "number",
                TokenType::And => "'and'",
                TokenType::Class => "'class'",
                TokenType::Else => "'else'",
                TokenType::False => "'false'",
                TokenType::Fun => "'fun'",
                TokenType::For => "'for'",
                TokenType::If => "'if'",
                TokenType::Nil => "nil",
                TokenType::Or => "'or'",
                TokenType::Print => "'print'",
                TokenType::Return => "'return'",
                TokenType::Super => "'super'",
                TokenType::This => "'this'",
                TokenType::True => "'true'",
                TokenType::Var => "'var'",
                TokenType::Let => "'Let'",
                TokenType::While => "'while'",
                TokenType::Import => "'import'",
                TokenType::Eof => "<EOF>",
                TokenType::Unknown => "<Unknown>",
            }
        )
    }
}
