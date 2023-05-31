use crate::common::{diagnostic::Position, types::Object};

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

impl Expression {
    pub(crate) fn get_position(&self) -> Position {
        match self {
            Expression::Literal(expression) => expression.get_position(),
            Expression::Name(expression) => expression.get_position(),
            Expression::Parenthesized(expression) => expression.get_position(),
            Expression::Unary(expression) => expression.get_position(),
            Expression::Binary(expression) => expression.get_position(),
            Expression::Assignment(expression) => expression.get_position(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub(crate) value: Object,
    position: Position,
}

impl LiteralExpression {
    pub(super) fn new(value: Object, position: Position) -> Self {
        Self { value, position }
    }

    pub(crate) fn get_position(&self) -> Position {
        self.position.clone()
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

    pub(crate) fn get_position(&self) -> Position {
        self.identifier.position.clone()
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

    pub(crate) fn get_position(&self) -> Position {
        self.expression.get_position()
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

    pub(crate) fn get_position(&self) -> Position {
        self.operator.position.clone()
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

    pub(crate) fn get_position(&self) -> Position {
        self.operator.position.clone()
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

    pub(crate) fn get_position(&self) -> Position {
        self.identifier.position.clone()
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Var(VarStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
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

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub(crate) condition: Expression,
    pub(crate) consequence: Box<Statement>,
    pub(crate) else_clause: Box<Option<Statement>>,
}

impl IfStatement {
    pub(super) fn new(
        condition: Expression,
        consequence: Statement,
        else_clause: Option<Statement>,
    ) -> Self {
        Self {
            condition,
            consequence: Box::new(consequence),
            else_clause: Box::new(else_clause),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub(crate) condition: Expression,
    pub(crate) body: Box<Statement>,
}

impl WhileStatement {
    pub(super) fn new(condition: Expression, body: Statement) -> Self {
        Self {
            condition,
            body: Box::new(body),
        }
    }
}
