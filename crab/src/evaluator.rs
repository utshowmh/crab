use std::collections::HashMap;

use crate::{
    binding::bound_tree::{
        BoundBinaryOperationKind, BoundExpression, BoundStatement, BoundUnaryOperationKind,
    },
    common::types::Object,
};

pub(crate) struct Evaluator {
    bound_statements: Vec<BoundStatement>,
    pub(super) variables: HashMap<String, Object>,
}

impl Evaluator {
    pub(crate) fn new(
        bound_statements: Vec<BoundStatement>,
        variables: HashMap<String, Object>,
    ) -> Self {
        Self {
            bound_statements,
            variables,
        }
    }

    pub(crate) fn evaluate(&mut self) {
        for statement in self.bound_statements.clone() {
            self.evaluate_statement(statement)
        }
    }

    fn evaluate_statement(&mut self, statement: BoundStatement) {
        match statement {
            BoundStatement::Expression(statement) => {
                self.evaluate_expression(&statement.expression);
            }
        }
    }

    fn evaluate_expression(&mut self, bound_expression: &BoundExpression) -> Object {
        match bound_expression {
            BoundExpression::Literal(expression) => expression.value.clone(),

            BoundExpression::Variable(expression) => {
                self.variables.get(&expression.name).unwrap().clone()
            }

            BoundExpression::Unary(expression) => {
                let right = self.evaluate_expression(&expression.right);
                match expression.operator.operation_kind {
                    BoundUnaryOperationKind::Identity => Object::Number(right.as_number()),
                    BoundUnaryOperationKind::Negation => Object::Number(-right.as_number()),
                    BoundUnaryOperationKind::LogicalNegation => {
                        Object::Boolean(!right.as_boolean())
                    }
                }
            }

            BoundExpression::Binary(expression) => {
                let left = self.evaluate_expression(&expression.left);
                let right = self.evaluate_expression(&expression.right);
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

            BoundExpression::Assignment(expression) => {
                let value = self.evaluate_expression(&expression.expression);
                self.variables
                    .insert(expression.name.clone(), value.clone());
                value
            }
        }
    }
}
