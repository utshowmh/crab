use crate::{
    binding::bound_tree::{BoundBinaryOperatorKind, BoundExpression, BoundUnaryOperatorKind},
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
            match expression.operator {
                BoundUnaryOperatorKind::Identity => Object::Number(right.as_number()),
                BoundUnaryOperatorKind::Negation => Object::Number(-right.as_number()),
                BoundUnaryOperatorKind::LogicalNegation => Object::Boolean(!right.as_boolean()),
            }
        }

        BoundExpression::Binary(expression) => {
            let left = evaluate_expression(*expression.left);
            let right = evaluate_expression(*expression.right);
            match expression.operator {
                BoundBinaryOperatorKind::Addition => {
                    Object::Number(left.as_number() + right.as_number())
                }
                BoundBinaryOperatorKind::Subtraction => {
                    Object::Number(left.as_number() - right.as_number())
                }
                BoundBinaryOperatorKind::Multiplication => {
                    Object::Number(left.as_number() * right.as_number())
                }
                BoundBinaryOperatorKind::Division => {
                    Object::Number(left.as_number() / right.as_number())
                }
                BoundBinaryOperatorKind::LogicalAnd => {
                    Object::Boolean(left.as_boolean() && right.as_boolean())
                }
                BoundBinaryOperatorKind::LogicalOr => {
                    Object::Boolean(left.as_boolean() || right.as_boolean())
                }
            }
        }
    }
}
