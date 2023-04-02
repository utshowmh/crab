use crate::{
    binding::bound_tree::{BoundBinaryOperationKind, BoundExpression, BoundUnaryOperationKind},
    common::types::Object,
};

pub fn evaluate(root: BoundExpression) -> Object {
    evaluate_expression(root)
}

fn evaluate_expression(root: BoundExpression) -> Object {
    match root {
        BoundExpression::Literal(expression) => expression.value,

        BoundExpression::Unary(expression) => {
            let right = evaluate_expression(*expression.right);
            match expression.operator.operation_kind {
                BoundUnaryOperationKind::Identity => Object::Number(right.as_number()),
                BoundUnaryOperationKind::Negation => Object::Number(-right.as_number()),
                BoundUnaryOperationKind::LogicalNegation => Object::Boolean(!right.as_boolean()),
            }
        }

        BoundExpression::Binary(expression) => {
            let left = evaluate_expression(*expression.left);
            let right = evaluate_expression(*expression.right);
            match expression.operator.operation_kind {
                BoundBinaryOperationKind::Addition => {
                    Object::Number(left.as_number() + right.as_number())
                }
                BoundBinaryOperationKind::Subtraction => {
                    Object::Number(left.as_number() - right.as_number())
                }
                BoundBinaryOperationKind::Multiplication => {
                    Object::Number(left.as_number() * right.as_number())
                }
                BoundBinaryOperationKind::Division => {
                    Object::Number(left.as_number() / right.as_number())
                }

                BoundBinaryOperationKind::LogicalAnd => {
                    Object::Boolean(left.as_boolean() && right.as_boolean())
                }
                BoundBinaryOperationKind::LogicalOr => {
                    Object::Boolean(left.as_boolean() || right.as_boolean())
                }

                BoundBinaryOperationKind::NotEqual => Object::Boolean(left != right),
                BoundBinaryOperationKind::Equal => Object::Boolean(left == right),
            }
        }
    }
}
