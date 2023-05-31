use std::fmt::Display;

use crate::common::diagnostic::Position;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenKind {
    Plus,
    Minus,
    Star,
    Slash,

    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,

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
    OpenBrace,
    CloseBrace,

    Number,

    True,
    False,
    Identifier,

    Print,
    Var,
    If,
    Else,
    While,

    Whitespace,

    Invalid,
    Eof,
}

impl TokenKind {
    pub(super) fn get_lexeme_type(lexeme: &str) -> Self {
        match lexeme {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "print" => TokenKind::Print,
            "var" => TokenKind::Var,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
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

            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),
            TokenKind::Lesser => write!(f, "<"),
            TokenKind::LesserEqual => write!(f, "<="),

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
            TokenKind::OpenBrace => write!(f, "{{"),
            TokenKind::CloseBrace => write!(f, "}}"),

            TokenKind::Number => write!(f, "NUMBER"),

            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Identifier => write!(f, "IDENTIFIER"),

            TokenKind::Print => write!(f, "print"),
            TokenKind::Var => write!(f, "var"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),

            TokenKind::Whitespace => write!(f, "WHITESPACE"),

            TokenKind::Invalid => write!(f, "INVALID"),
            TokenKind::Eof => write!(f, "EOF"),
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
