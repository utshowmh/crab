use crate::common::types::{Object, Type};

#[derive(Debug)]
pub enum BoundBinaryOperatorKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LogicalAnd,
    LogicalOr,
}

#[derive(Debug)]
pub enum BoundUnaryOperatorKind {
    Identity,
    Negation,
    LogicalNegation,
}

#[derive(Debug)]
pub enum BoundExpression {
    Literal(BoundLiteralExpression),
    Unary(BoundUnaryExpression),
    Binary(BoundBinaryExpression),
}

impl BoundExpression {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            BoundExpression::Literal(expression) => expression.get_type(),
            BoundExpression::Unary(expression) => expression.get_type(),
            BoundExpression::Binary(expression) => expression.get_type(),
        }
    }
}

#[derive(Debug)]
pub struct BoundLiteralExpression {
    pub value: Object,
}

impl BoundLiteralExpression {
    pub(super) fn new(value: Object) -> Self {
        Self { value }
    }

    fn get_type(&self) -> Type {
        self.value.get_type()
    }
}

#[derive(Debug)]
pub struct BoundUnaryExpression {
    pub operator: BoundUnaryOperatorKind,
    pub right: Box<BoundExpression>,
}

impl BoundUnaryExpression {
    pub(super) fn new(operator: BoundUnaryOperatorKind, right: BoundExpression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    fn get_type(&self) -> Type {
        self.right.get_type()
    }
}

#[derive(Debug)]
pub struct BoundBinaryExpression {
    pub left: Box<BoundExpression>,
    pub operator: BoundBinaryOperatorKind,
    pub right: Box<BoundExpression>,
}

impl BoundBinaryExpression {
    pub(super) fn new(
        left: BoundExpression,
        operator: BoundBinaryOperatorKind,
        right: BoundExpression,
    ) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn get_type(&self) -> Type {
        self.left.get_type()
    }
}
