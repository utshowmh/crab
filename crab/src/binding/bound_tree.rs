use std::fmt::Display;

use crate::{
    common::{
        diagnostic::Position,
        types::{Object, Type},
    },
    syntax::token::TokenKind,
};

#[derive(Debug, Clone)]
pub enum BoundBinaryOperationKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,

    Greater,
    Lesser,
    GreaterEqual,
    LesserEqual,

    LogicalAnd,
    LogicalOr,

    NotEqual,
    Equal,
}

impl Display for BoundBinaryOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundBinaryOperationKind::Addition => write!(f, "+"),
            BoundBinaryOperationKind::Subtraction => write!(f, "-"),
            BoundBinaryOperationKind::Multiplication => write!(f, "*"),
            BoundBinaryOperationKind::Division => write!(f, "/"),
            BoundBinaryOperationKind::Greater => write!(f, ">"),
            BoundBinaryOperationKind::Lesser => write!(f, "<"),
            BoundBinaryOperationKind::GreaterEqual => write!(f, ">="),
            BoundBinaryOperationKind::LesserEqual => write!(f, "<="),
            BoundBinaryOperationKind::LogicalAnd => write!(f, "&&"),
            BoundBinaryOperationKind::LogicalOr => write!(f, "||"),
            BoundBinaryOperationKind::NotEqual => write!(f, "!="),
            BoundBinaryOperationKind::Equal => write!(f, "=="),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BoundUnaryOperationKind {
    Identity,
    Negation,
    LogicalNegation,
}

impl Display for BoundUnaryOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundUnaryOperationKind::Identity => write!(f, "+"),
            BoundUnaryOperationKind::Negation => write!(f, "-"),
            BoundUnaryOperationKind::LogicalNegation => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundUnaryOperator {
    pub(super) operator_kind: TokenKind,
    pub operation_kind: BoundUnaryOperationKind,
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

    pub fn bind(operator_kind: TokenKind, right_type: Type) -> Option<Self> {
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
        operators.into_iter().find(|operator| {
            operator.operator_kind == operator_kind && operator.right_type == right_type
        })
    }
}

#[derive(Debug, Clone)]
pub struct BoundBinaryOperator {
    pub(super) operator_kind: TokenKind,
    pub operation_kind: BoundBinaryOperationKind,
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

    pub fn bind(operator_kind: TokenKind, left_type: Type, right_type: Type) -> Option<Self> {
        let operators = vec![
            BoundBinaryOperator::new(
                TokenKind::Plus,
                BoundBinaryOperationKind::Addition,
                Type::Number,
                Type::Number,
                Type::Number,
            ),
            BoundBinaryOperator::new(
                TokenKind::Plus,
                BoundBinaryOperationKind::Addition,
                Type::String,
                Type::String,
                Type::String,
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
                TokenKind::Greater,
                BoundBinaryOperationKind::Greater,
                Type::Number,
                Type::Number,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::Lesser,
                BoundBinaryOperationKind::Lesser,
                Type::Number,
                Type::Number,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::GreaterEqual,
                BoundBinaryOperationKind::GreaterEqual,
                Type::Number,
                Type::Number,
                Type::Boolean,
            ),
            BoundBinaryOperator::new(
                TokenKind::LesserEqual,
                BoundBinaryOperationKind::LesserEqual,
                Type::Number,
                Type::Number,
                Type::Boolean,
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
        operators.into_iter().find(|operator| {
            operator.operator_kind == operator_kind
                && operator.left_type == left_type
                && operator.right_type == right_type
        })
    }
}

#[derive(Debug, Clone)]
pub enum BoundExpression {
    Literal(BoundLiteralExpression),
    Variable(BoundVariableExpression),
    Unary(BoundUnaryExpression),
    Binary(BoundBinaryExpression),
    Assignment(BoundAssignmentExpression),
}

impl BoundExpression {
    pub fn get_type(&self) -> Type {
        match self {
            BoundExpression::Literal(expression) => expression.get_type(),
            BoundExpression::Variable(expression) => expression.get_type(),
            BoundExpression::Unary(expression) => expression.get_type(),
            BoundExpression::Binary(expression) => expression.get_type(),
            BoundExpression::Assignment(expression) => expression.get_type(),
        }
    }

    pub fn get_position(&self) -> Position {
        match self {
            BoundExpression::Literal(expression) => expression.get_position(),
            BoundExpression::Variable(expression) => expression.get_position(),
            BoundExpression::Unary(expression) => expression.get_position(),
            BoundExpression::Binary(expression) => expression.get_position(),
            BoundExpression::Assignment(expression) => expression.get_position(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundLiteralExpression {
    pub value: Object,
    position: Position,
}

impl BoundLiteralExpression {
    pub(super) fn new(value: Object, position: Position) -> Self {
        Self { value, position }
    }

    fn get_type(&self) -> Type {
        self.value.get_type()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundVariableExpression {
    pub name: String,
    pub value_type: Type,
    position: Position,
}

impl BoundVariableExpression {
    pub(super) fn new(name: String, value_type: Type, position: Position) -> Self {
        Self {
            name,
            value_type,
            position,
        }
    }

    fn get_type(&self) -> Type {
        self.value_type.clone()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundUnaryExpression {
    pub operator: BoundUnaryOperator,
    pub right: Box<BoundExpression>,
    position: Position,
}

impl BoundUnaryExpression {
    pub(super) fn new(
        operator: BoundUnaryOperator,
        right: BoundExpression,
        position: Position,
    ) -> Self {
        Self {
            operator,
            right: Box::new(right),
            position,
        }
    }

    fn get_type(&self) -> Type {
        self.operator.result_type.clone()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundBinaryExpression {
    pub left: Box<BoundExpression>,
    pub operator: BoundBinaryOperator,
    pub right: Box<BoundExpression>,
    position: Position,
}

impl BoundBinaryExpression {
    pub(super) fn new(
        left: BoundExpression,
        operator: BoundBinaryOperator,
        right: BoundExpression,
        position: Position,
    ) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            position,
        }
    }

    fn get_type(&self) -> Type {
        self.operator.result_type.clone()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BoundAssignmentExpression {
    pub name: String,
    pub expression: Box<BoundExpression>,
    position: Position,
}

impl BoundAssignmentExpression {
    pub fn new(name: String, expression: BoundExpression, position: Position) -> Self {
        Self {
            name,
            expression: Box::new(expression),
            position,
        }
    }

    pub fn get_type(&self) -> Type {
        self.expression.get_type()
    }

    fn get_position(&self) -> Position {
        self.position.clone()
    }
}

#[derive(Debug, Clone)]
pub enum BoundStatement {
    Expression(BoundExpressionStatement),
    Print(BoundPrintStatement),
    Var(BoundVarStatement),
    Block(BoundBlockStatement),
    If(BoundIfStatement),
    While(BoundWhileStatement),
    For(BoundForStatement),
}

#[derive(Debug, Clone)]
pub struct BoundExpressionStatement {
    pub expression: BoundExpression,
}

impl BoundExpressionStatement {
    pub fn new(expression: BoundExpression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct BoundPrintStatement {
    pub expression: BoundExpression,
}

impl BoundPrintStatement {
    pub fn new(expression: BoundExpression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct BoundVarStatement {
    pub name: String,
    pub expression: Box<BoundExpression>,
}

impl BoundVarStatement {
    pub fn new(name: String, expression: BoundExpression) -> Self {
        Self {
            name,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundBlockStatement {
    pub statements: Vec<BoundStatement>,
}

impl BoundBlockStatement {
    pub(super) fn new(statements: Vec<BoundStatement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub struct BoundIfStatement {
    pub condition: BoundExpression,
    pub consequence: Box<BoundStatement>,
    pub else_clause: Box<Option<BoundStatement>>,
}

impl BoundIfStatement {
    pub(super) fn new(
        condition: BoundExpression,
        consequence: BoundStatement,
        else_clause: Option<BoundStatement>,
    ) -> Self {
        Self {
            condition,
            consequence: Box::new(consequence),
            else_clause: Box::new(else_clause),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundWhileStatement {
    pub condition: BoundExpression,
    pub body: Box<BoundStatement>,
}

impl BoundWhileStatement {
    pub(super) fn new(condition: BoundExpression, body: BoundStatement) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundForStatement {
    pub identifier: String,
    pub lower_bound: BoundExpression,
    pub upper_bound: BoundExpression,
    pub body: Box<BoundStatement>,
}

impl BoundForStatement {
    pub(super) fn new(
        identifier: String,
        lower_bound: BoundExpression,
        upper_bound: BoundExpression,
        body: BoundStatement,
    ) -> Self {
        Self {
            identifier,
            lower_bound,
            upper_bound,
            body: Box::new(body),
        }
    }
}
