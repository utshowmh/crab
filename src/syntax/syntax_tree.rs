use crate::common::types::Object;

use super::{lexer::Lexer, parser::Parser, token::Token};

pub struct SyntaxTree {
    pub root: Expression,
    pub diagnostics: Vec<String>,
}

impl SyntaxTree {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer.lex(), lexer.diagnostics);
        Self {
            root: parser.parse(),
            diagnostics: parser.diagnostics,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Parenthesized(ParenthesizedExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub token: Token,
    pub value: Object,
}

impl LiteralExpression {
    pub(super) fn new(token: Token, value: Object) -> Self {
        Self { token, value }
    }
}

#[derive(Debug)]
pub struct ParenthesizedExpression {
    pub expression: Box<Expression>,
}

impl ParenthesizedExpression {
    pub(super) fn new(expression: Expression) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl UnaryExpression {
    pub(super) fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub(super) fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}
