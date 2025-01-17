#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

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
    While,

    // Literals.
    Identifier(String),
    String(String),
    Number(f32),

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Token {
        return Self {
            token_type,
            lexeme,
            line,
        };
    }
    pub fn get_token_type(&self) -> &TokenType {
        return &self.token_type;
    }
    pub fn get_lexeme(&self) -> String {
        return self.lexeme.clone();
    }
    pub fn to_string(&self) -> String {
        return format!("{:?} {}", self.token_type, self.lexeme);
    }
}
