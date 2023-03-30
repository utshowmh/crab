use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
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

impl TokenKind {
    pub(super) fn get_unary_precedence(&self) -> usize {
        match self {
            TokenKind::Plus | TokenKind::Minus => 3,
            _ => 0,
        }
    }

    pub(super) fn get_binary_precedence(&self) -> usize {
        match self {
            TokenKind::Star | TokenKind::Slash => 2,
            TokenKind::Plus | TokenKind::Minus => 1,
            _ => 0,
        }
    }
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

            TokenKind::Number => write!(f, "{self:?}"),

            TokenKind::Whitespace => write!(f, "{self:?}"),

            TokenKind::Invalid => write!(f, "{self:?}"),
            TokenKind::Eof => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String) -> Self {
        Self { kind, lexeme }
    }
}
