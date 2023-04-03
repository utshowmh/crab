use crate::syntax::syntax_tree::{
    BinaryExpression, Expression, LiteralExpression, ParenthesizedExpression, UnaryExpression,
};

use super::bound_tree::{
    BoundBinaryExpression, BoundBinaryOperator, BoundExpression, BoundLiteralExpression,
    BoundUnaryExpression, BoundUnaryOperator,
};

pub(crate) struct Binder {
    pub(crate) diagnostics: Vec<String>,
}

impl Binder {
    pub(crate) fn new(diagnostics: Vec<String>) -> Self {
        Self { diagnostics }
    }

    pub(crate) fn bind(&mut self, root: Expression) -> BoundExpression {
        self.bind_expression(root)
    }

    fn bind_expression(&mut self, root: Expression) -> BoundExpression {
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
        &mut self,
        expression: ParenthesizedExpression,
    ) -> BoundExpression {
        self.bind_expression(*expression.expression)
    }

    fn bind_unary_expression(&mut self, expression: UnaryExpression) -> BoundExpression {
        let right = self.bind_expression(*expression.right);
        if let Some(operator) = BoundUnaryOperator::bind(expression.operator.kind, right.get_type())
        {
            BoundExpression::Unary(BoundUnaryExpression::new(operator, right))
        } else {
            self.diagnostics.push(format!(
                "Unary operator '{}' is not defined for '{}'",
                expression.operator.lexeme,
                right.get_type()
            ));
            right
        }
    }

    fn bind_binary_expression(&mut self, expression: BinaryExpression) -> BoundExpression {
        let left = self.bind_expression(*expression.left);
        let right = self.bind_expression(*expression.right);
        if let Some(operator) =
            BoundBinaryOperator::bind(expression.operator.kind, left.get_type(), right.get_type())
        {
            BoundExpression::Binary(BoundBinaryExpression::new(left, operator, right))
        } else {
            self.diagnostics.push(format!(
                "Binary operator '{}' is not defined for '{}' and '{}'",
                expression.operator.lexeme,
                left.get_type(),
                right.get_type()
            ));
            left
        }
    }
}
