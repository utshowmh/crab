use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,

    Number,

    Whitespace,

    Invalid,
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),

            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),

            TokenKind::Number => write!(f, "{:?}", self),

            TokenKind::Whitespace => write!(f, "{:?}", self),

            TokenKind::Invalid => write!(f, "{:?}", self),
            TokenKind::Eof => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String) -> Self {
        Self { kind, lexeme }
    }
}
