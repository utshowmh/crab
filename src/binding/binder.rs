use crate::{
    common::types::Type,
    syntax::{
        syntax_tree::{
            BinaryExpression, Expression, LiteralExpression, ParenthesizedExpression,
            UnaryExpression,
        },
        token::TokenKind,
    },
};

use super::bound_tree::{
    BoundBinaryExpression, BoundBinaryOperatorKind, BoundExpression, BoundLiteralExpression,
    BoundUnaryExpression, BoundUnaryOperatorKind,
};

pub struct Binder {}

impl Binder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn bind(&self, root: Expression) -> BoundExpression {
        self.bind_expression(root)
    }

    fn bind_expression(&self, root: Expression) -> BoundExpression {
        match root {
            Expression::Literal(expression) => self.bind_literal_expression(expression),
            Expression::Parenthesized(expression) => self.bind_parenthesized_expression(expression),
            Expression::Unary(expression) => self.bind_unary_expression(expression),
            Expression::Binary(expression) => self.bind_binary_expression(expression),
        }
    }

    fn bind_literal_expression(&self, expression: LiteralExpression) -> BoundExpression {
        BoundExpression::Literal(BoundLiteralExpression::new(expression.value))
    }

    fn bind_parenthesized_expression(
        &self,
        expression: ParenthesizedExpression,
    ) -> BoundExpression {
        self.bind_expression(*expression.expression)
    }

    fn bind_unary_expression(&self, expression: UnaryExpression) -> BoundExpression {
        let right = self.bind_expression(*expression.right);
        if let Some(operator) = self.get_unary_operator_kind(expression.operator.kind, &right) {
            BoundExpression::Unary(BoundUnaryExpression::new(operator, right))
        } else {
            panic!(
                "Unary operator '{}' is not defined for '{}'",
                expression.operator.lexeme,
                right.get_type()
            )
        }
    }

    fn bind_binary_expression(&self, expression: BinaryExpression) -> BoundExpression {
        let left = self.bind_expression(*expression.left);
        let right = self.bind_expression(*expression.right);
        if let Some(operator) =
            self.get_binary_operator_kind(expression.operator.kind, &left, &right)
        {
            BoundExpression::Binary(BoundBinaryExpression::new(left, operator, right))
        } else {
            panic!(
                "Binary operator '{}' is not defined for '{}' and '{}'",
                expression.operator.lexeme,
                left.get_type(),
                right.get_type()
            )
        }
    }

    fn get_unary_operator_kind(
        &self,
        kind: TokenKind,
        right: &BoundExpression,
    ) -> Option<BoundUnaryOperatorKind> {
        match right.get_type() {
            Type::Number => match kind {
                TokenKind::Plus => Some(BoundUnaryOperatorKind::Identity),
                TokenKind::Minus => Some(BoundUnaryOperatorKind::Negation),
                _ => None,
            },
            Type::Boolean => None,
        }
    }

    fn get_binary_operator_kind(
        &self,
        kind: TokenKind,
        left: &BoundExpression,
        right: &BoundExpression,
    ) -> Option<BoundBinaryOperatorKind> {
        match (left.get_type(), right.get_type()) {
            (Type::Number, Type::Number) => match kind {
                TokenKind::Plus => Some(BoundBinaryOperatorKind::Addition),
                TokenKind::Minus => Some(BoundBinaryOperatorKind::Subtraction),
                TokenKind::Star => Some(BoundBinaryOperatorKind::Multiplication),
                TokenKind::Slash => Some(BoundBinaryOperatorKind::Division),
                _ => None,
            },
            (Type::Boolean, Type::Boolean) => match kind {
                _ => None,
            },
            _ => None,
        }
    }
}
