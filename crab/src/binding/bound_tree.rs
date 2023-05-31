use crate::{
    common::{
        diagnostic::Position,
        types::{Object, Type},
    },
    syntax::token::TokenKind,
};

#[derive(Debug, Clone)]
pub(crate) enum BoundBinaryOperationKind {
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

#[derive(Debug, Clone)]
pub(crate) enum BoundUnaryOperationKind {
    Identity,
    Negation,
    LogicalNegation,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub(crate) enum BoundExpression {
    Literal(BoundLiteralExpression),
    Variable(BoundVariableExpression),
    Unary(BoundUnaryExpression),
    Binary(BoundBinaryExpression),
    Assignment(BoundAssignmentExpression),
}

impl BoundExpression {
    pub(crate) fn get_type(&self) -> Type {
        match self {
            BoundExpression::Literal(expression) => expression.get_type(),
            BoundExpression::Variable(expression) => expression.get_type(),
            BoundExpression::Unary(expression) => expression.get_type(),
            BoundExpression::Binary(expression) => expression.get_type(),
            BoundExpression::Assignment(expression) => expression.get_type(),
        }
    }

    pub(crate) fn get_position(&self) -> Position {
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
pub(crate) struct BoundLiteralExpression {
    pub(crate) value: Object,
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
pub(crate) struct BoundVariableExpression {
    pub(crate) name: String,
    pub(crate) value_type: Type,
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
pub(crate) struct BoundUnaryExpression {
    pub(crate) operator: BoundUnaryOperator,
    pub(crate) right: Box<BoundExpression>,
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
pub(crate) struct BoundBinaryExpression {
    pub(crate) left: Box<BoundExpression>,
    pub(crate) operator: BoundBinaryOperator,
    pub(crate) right: Box<BoundExpression>,
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
pub(crate) struct BoundAssignmentExpression {
    pub(crate) name: String,
    pub(crate) expression: Box<BoundExpression>,
    position: Position,
}

impl BoundAssignmentExpression {
    pub(crate) fn new(name: String, expression: BoundExpression, position: Position) -> Self {
        Self {
            name,
            expression: Box::new(expression),
            position,
        }
    }

    pub(crate) fn get_type(&self) -> Type {
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
}

#[derive(Debug, Clone)]
pub struct BoundExpressionStatement {
    pub(crate) expression: BoundExpression,
}

impl BoundExpressionStatement {
    pub(crate) fn new(expression: BoundExpression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct BoundPrintStatement {
    pub(crate) expression: BoundExpression,
}

impl BoundPrintStatement {
    pub(crate) fn new(expression: BoundExpression) -> Self {
        Self { expression }
    }
}

#[derive(Debug, Clone)]
pub struct BoundVarStatement {
    pub(crate) name: String,
    pub(crate) expression: Box<BoundExpression>,
}

impl BoundVarStatement {
    pub(crate) fn new(name: String, expression: BoundExpression) -> Self {
        Self {
            name,
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BoundBlockStatement {
    pub(crate) statements: Vec<BoundStatement>,
}

impl BoundBlockStatement {
    pub(super) fn new(statements: Vec<BoundStatement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub struct BoundIfStatement {
    pub(crate) condition: BoundExpression,
    pub(crate) consequence: Box<BoundStatement>,
    pub(crate) else_clause: Box<Option<BoundStatement>>,
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
    pub(crate) condition: BoundExpression,
    pub(crate) body: Box<BoundStatement>,
}

impl BoundWhileStatement {
    pub(super) fn new(condition: BoundExpression, body: BoundStatement) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}
