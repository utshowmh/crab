use crate::common::{diagnostic::DiagnosticBag, types::Object};

use super::{lexer::Lexer, parser::Parser, token::Token};

pub struct SyntaxTree {
    pub root: Expression,
    pub(crate) diagnostic_bag: DiagnosticBag,
}

impl SyntaxTree {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer.lex(), lexer.diagnostic_bag);
        Self {
            root: parser.parse(),
            diagnostic_bag: parser.diagnostic_bag,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Name(NameExpression),
    Parenthesized(ParenthesizedExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Assignment(AssignmentExpression),
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub(crate) value: Object,
}

impl LiteralExpression {
    pub(super) fn new(value: Object) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct NameExpression {
    pub(crate) identifier: Token,
}

impl NameExpression {
    pub(super) fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}

#[derive(Debug)]
pub struct ParenthesizedExpression {
    pub(crate) expression: Box<Expression>,
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
    pub(crate) operator: Token,
    pub(crate) right: Box<Expression>,
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
    pub(crate) left: Box<Expression>,
    pub(crate) operator: Token,
    pub(crate) right: Box<Expression>,
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

#[derive(Debug)]
pub struct AssignmentExpression {
    pub(crate) identifier: Token,
    pub(crate) equal: Token,
    pub(crate) expression: Box<Expression>,
}

impl AssignmentExpression {
    pub(super) fn new(identifier: Token, equal: Token, expression: Expression) -> Self {
        Self {
            identifier,
            equal,
            expression: Box::new(expression),
        }
    }
}
