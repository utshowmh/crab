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
                BoundUnaryOperatorKind::Identity => right,
                BoundUnaryOperatorKind::Negation => match right {
                    Object::Number(n) => Object::Number(-n),
                },
            }
        }

        BoundExpression::Binary(expression) => {
            let left = evaluate_expression(*expression.left);
            let right = evaluate_expression(*expression.right);
            match expression.operator {
                BoundBinaryOperatorKind::Addition => match (left, right) {
                    (Object::Number(x), Object::Number(y)) => Object::Number(x + y),
                },
                BoundBinaryOperatorKind::Subtraction => match (left, right) {
                    (Object::Number(x), Object::Number(y)) => Object::Number(x - y),
                },
                BoundBinaryOperatorKind::Multiplication => match (left, right) {
                    (Object::Number(x), Object::Number(y)) => Object::Number(x * y),
                },
                BoundBinaryOperatorKind::Division => match (left, right) {
                    (Object::Number(x), Object::Number(y)) => Object::Number(x / y),
                },
            }
        }
    }
}
