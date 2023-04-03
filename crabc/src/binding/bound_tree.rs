use crate::{
    common::types::{Object, Type},
    syntax::token::TokenKind,
};

#[derive(Debug)]
pub(crate) enum BoundBinaryOperationKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,

    LogicalAnd,
    LogicalOr,

    NotEqual,
    Equal,
}

#[derive(Debug)]
pub(crate) enum BoundUnaryOperationKind {
    Identity,
    Negation,
    LogicalNegation,
}

#[derive(Debug)]
pub(crate) struct BoundUnaryOperator {
    pub(super) operator_kind: TokenKind,
    pub(crate) operation_kind: BoundUnaryOperationKind,
    pub(super) right_type: Type,
    pub(super) result_type: Type,
}

impl BoundUnaryOperator {
    pub(super) fn new(
        operator_kind: TokenKind,
        operation_kind: BoundUnaryOperationKind,
        right_type: Type,
        result_type: Type,
    ) -> Self {
        Self {
            operator_kind,
            operation_kind,
            right_type,
            result_type,
        }
    }

    pub(crate) fn bind(operator_kind: TokenKind, right_type: Type) -> Option<Self> {
        let operators = vec![
            BoundUnaryOperator::new(
                TokenKind::Plus,
                BoundUnaryOperationKind::Identity,
                Type::Number,
                Type::Number,
            ),
            BoundUnaryOperator::new(
                TokenKind::Minus,
                BoundUnaryOperationKind::Negation,
                Type::Number,
                Type::Number,
            ),
            BoundUnaryOperator::new(
                TokenKind::Bang,
                BoundUnaryOperationKind::LogicalNegation,
                Type::Boolean,
                Type::Boolean,
            ),
        ];
        for operator in operators {
            if operator.operator_kind == operator_kind && operator.right_type == right_type {
                return Some(operator);
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct BoundBinaryOperator {
    pub(super) operator_kind: TokenKind,
    pub(crate) operation_kind: BoundBinaryOperationKind,
    pub(super) left_type: Type,
    pub(super) right_type: Type,
    pub(super) result_type: Type,
}

impl BoundBinaryOperator {
    pub(super) fn new(
        operator_kind: TokenKind,
        operation_kind: BoundBinaryOperationKind,
        left_type: Type,
        right_type: Type,
        result_type: Type,
    ) -> Self {
        Self {
            operator_kind,
            operation_kind,
            left_type,
            right_type,
            result_type,
        }
    }

    pub(crate) fn bind(
        operator_kind: TokenKind,
        left_type: Type,
        right_type: Type,
    ) -> Option<Self> {
        let operators = vec![
            BoundBinaryOperator::new(
                TokenKind::Plus,
                BoundBinaryOperationKind::Addition,
                Type::Number,
                Type::Number,
                Type::Number,
            ),
            BoundBinaryOperator::new(
                TokenKind::Minus,
                BoundBinaryOperationKind::Subtraction,
                Type::Number,
                Type::Number,
                Type::Number,
            ),
            BoundBinaryOperator::new(
                TokenKind::Star,
                BoundBinaryOperationKind::Multiplication,
                Type::Number,
                Type::Number,
                Type::Number,
            ),
            BoundBinaryOperator::new(
                TokenKind::Slash,
                BoundBinaryOperationKind::Division,
                Type::Number,
                Type::Number,
                Type::Number,
            ),
            BoundBinaryOperator::new(
                TokenKind::AmpersandAmpersand,
                BoundBinaryOperationKind::LogicalAnd,
                Type::Boolean,
                Type::Boolean,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::PipePipe,
                BoundBinaryOperationKind::LogicalOr,
                Type::Boolean,
                Type::Boolean,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::BangEqual,
                BoundBinaryOperationKind::NotEqual,
                Type::Number,
                Type::Number,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::EqualEqual,
                BoundBinaryOperationKind::Equal,
                Type::Number,
                Type::Number,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::BangEqual,
                BoundBinaryOperationKind::NotEqual,
                Type::Boolean,
                Type::Boolean,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::EqualEqual,
                BoundBinaryOperationKind::Equal,
                Type::Boolean,
                Type::Boolean,
                Type::Boolean,
            ),
        ];
        for operator in operators {
            if operator.operator_kind == operator_kind
                && operator.left_type == left_type
                && operator.right_type == right_type
            {
                return Some(operator);
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) enum BoundExpression {
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
pub(crate) struct BoundLiteralExpression {
    pub(crate) value: Object,
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
pub(crate) struct BoundUnaryExpression {
    pub(crate) operator: BoundUnaryOperator,
    pub(crate) right: Box<BoundExpression>,
}

impl BoundUnaryExpression {
    pub(super) fn new(operator: BoundUnaryOperator, right: BoundExpression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    fn get_type(&self) -> Type {
        self.operator.result_type.clone()
    }
}

#[derive(Debug)]
pub(crate) struct BoundBinaryExpression {
    pub(crate) left: Box<BoundExpression>,
    pub(crate) operator: BoundBinaryOperator,
    pub(crate) right: Box<BoundExpression>,
}

impl BoundBinaryExpression {
    pub(super) fn new(
        left: BoundExpression,
        operator: BoundBinaryOperator,
        right: BoundExpression,
    ) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn get_type(&self) -> Type {
        self.operator.result_type.clone()
    }
}