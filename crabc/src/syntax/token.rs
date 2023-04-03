use std::fmt::Display;

use crate::common::diagnostic::Position;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Ampersand,
    AmpersandAmpersand,
    Pipe,
    PipePipe,

    OpenParen,
    CloseParen,

    Number,

    True,
    False,
    Identifier,

    Whitespace,

    Invalid,
    Eof,
}

impl TokenKind {
    pub(super) fn get_lexeme_type(lexeme: &str) -> Self {
        match lexeme {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier,
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

            TokenKind::Bang => write!(f, "!"),
            TokenKind::BangEqual => write!(f, "!="),
            TokenKind::Equal => write!(f, "="),
            TokenKind::EqualEqual => write!(f, "=="),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::AmpersandAmpersand => write!(f, "&&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::PipePipe => write!(f, "||"),

            TokenKind::OpenParen => write!(f, "("),
            TokenKind::CloseParen => write!(f, ")"),

            TokenKind::Number => write!(f, "{self:?}"),

            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Identifier => write!(f, "{self:?}"),

            TokenKind::Whitespace => write!(f, "{self:?}"),

            TokenKind::Invalid => write!(f, "{self:?}"),
            TokenKind::Eof => write!(f, "{self:?}"),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) lexeme: String,
    pub(crate) position: Position,
}

impl Token {
    pub(super) fn new(kind: TokenKind, lexeme: String, position: Position) -> Self {
        Self {
            kind,
            lexeme,
            position,
        }
    }
}
