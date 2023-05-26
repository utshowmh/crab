use crate::common::types::Object;

use super::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(LiteralExpression),
    Name(NameExpression),
    Parenthesized(ParenthesizedExpression),
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Assignment(AssignmentExpression),
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub(crate) value: Object,
}

impl LiteralExpression {
    pub(super) fn new(value: Object) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone)]
pub struct NameExpression {
    pub(crate) identifier: Token,
}

impl NameExpression {
    pub(super) fn new(identifier: Token) -> Self {
        Self { identifier }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub(crate) identifier: Token,
    pub(crate) expression: Box<Expression>,
}

impl AssignmentExpression {
    pub(super) fn new(identifier: Token, expression: Expression) -> Self {
        Self {
            identifier,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Var(VarStatement),
    Block(BlockStatement),
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub(crate) expression: Expression,
}

impl ExpressionStatement {
    pub(super) fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct PrintStatement {
    pub(crate) expression: Expression,
}

impl PrintStatement {
    pub(super) fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct VarStatement {
    pub(crate) identifier: Token,
    pub(crate) expression: Expression,
}

impl VarStatement {
    pub(super) fn new(identifier: Token, expression: Expression) -> Self {
        Self {
            identifier,
            expression,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub(crate) statements: Vec<Statement>,
}

impl BlockStatement {
    pub(super) fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}
