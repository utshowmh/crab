use crate::syntax::token::TokenKind;

use super::types::Type;

#[derive(Debug, Clone)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub(crate) fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub struct Diagnostic {
    pub position: Position,
    pub message: String,
}

impl Diagnostic {
    fn new(position: Position, message: String) -> Self {
        Self { position, message }
    }
}

pub struct DiagnosticBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub(crate) fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn unexpected_character(&mut self, start: usize, char: char) {
        self.diagnostics.push(Diagnostic::new(
            Position::new(start, 1),
            format!("Unexpected character '{char}"),
        ));
    }

    pub(crate) fn unexpected_token(
        &mut self,
        position: Position,
        expected_token: TokenKind,
        current_token: TokenKind,
    ) {
        self.diagnostics.push(Diagnostic::new(
            position,
            format!("Unexpected token '{current_token}', expected '{expected_token}'"),
        ))
    }

    pub(crate) fn invalid_unary_operator(
        &mut self,
        position: Position,
        operator: TokenKind,
        right_type: Type,
    ) {
        self.diagnostics.push(Diagnostic::new(
            position,
            format!(
                "Unary operator '{}' is not defined for '{}'",
                operator, right_type
            ),
        ))
    }

    pub(crate) fn invalid_binary_operator(
        &mut self,
        position: Position,
        operator: TokenKind,
        left_type: Type,
        right_type: Type,
    ) {
        self.diagnostics.push(Diagnostic::new(
            position,
            format!(
                "Binary operator '{}' is not defined for '{}' and '{}'",
                operator, left_type, right_type
            ),
        ))
    }
}
