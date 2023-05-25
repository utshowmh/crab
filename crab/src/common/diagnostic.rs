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

    pub fn get_line_and_column(&self, source: &str) -> (usize, usize) {
        let mut line = 1;
        let mut column = 0;
        for char in source.chars() {
            if char == '\n' {
                line += 1;
                column = 0;
            } else {
                column += 1;
            }
        }
        (line, column)
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

    pub(crate) fn unexpected_character(&mut self, position: Position, char: char) {
        self.diagnostics.push(Diagnostic::new(
            position,
            format!("Unexpected character '{char}'"),
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
            format!("Unary operator '{operator}' is not defined for '{right_type}'"),
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
                "Binary operator '{operator}' is not defined for '{left_type}' and '{right_type}'",
            ),
        ))
    }

    pub(crate) fn undefined_name(&mut self, position: Position, name: String) {
        self.diagnostics.push(Diagnostic::new(
            position,
            format!("Name '{name}' is not defined"),
        ))
    }

    pub(crate) fn invalid_assignment(
        &mut self,
        position: Position,
        name: String,
        expected_type: Type,
        given_type: Type,
    ) {
        self.diagnostics.push(Diagnostic::new(
            position,
            format!("Name '{name}' expects '{expected_type}', got '{given_type}'"),
        ))
    }
}
