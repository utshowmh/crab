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
            let right = evaluate_expression(*expression.right).as_number();
            match expression.operator {
                BoundUnaryOperatorKind::Identity => Object::Number(right),
                BoundUnaryOperatorKind::Negation => Object::Number(-right),
            }
        }

        BoundExpression::Binary(expression) => {
            let left = evaluate_expression(*expression.left).as_number();
            let right = evaluate_expression(*expression.right).as_number();
            match expression.operator {
                BoundBinaryOperatorKind::Addition => Object::Number(left + right),
                BoundBinaryOperatorKind::Subtraction => Object::Number(left - right),
                BoundBinaryOperatorKind::Multiplication => Object::Number(left * right),
                BoundBinaryOperatorKind::Division => Object::Number(left / right),
            }
        }
    }
}
