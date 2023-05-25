use std::{cell::RefCell, rc::Rc};

use crate::{
    binding::{
        bindings::Bindings,
        bound_tree::{
            BoundBinaryOperationKind, BoundExpression, BoundStatement, BoundUnaryOperationKind,
        },
    },
    common::types::Object,
};

pub(crate) struct Evaluator {
    bound_statements: Vec<BoundStatement>,
    pub(super) bindings: Rc<RefCell<Bindings>>,
}

impl Evaluator {
    pub(crate) fn new(
        bound_statements: Vec<BoundStatement>,
        bindings: Rc<RefCell<Bindings>>,
    ) -> Self {
        Self {
            bound_statements,
            bindings,
        }
    }

    pub(crate) fn evaluate(&mut self) -> Object {
        let mut object = Object::Unit;
        for statement in self.bound_statements.clone() {
            object = self.evaluate_statement(statement);
        }
        object
    }

    fn evaluate_statement(&mut self, statement: BoundStatement) -> Object {
        match statement {
            BoundStatement::Expression(statement) => {
                self.evaluate_expression(&statement.expression)
            }
            BoundStatement::Print(statement) => {
                println!("{}", self.evaluate_expression(&statement.expression));
                Object::Unit
            }
        }
    }

    fn evaluate_expression(&mut self, bound_expression: &BoundExpression) -> Object {
        match bound_expression {
            BoundExpression::Literal(expression) => expression.value.clone(),

            BoundExpression::Variable(expression) => {
                self.bindings.borrow().get(&expression.name).unwrap()
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
                let object = self.evaluate_expression(&expression.expression);
                self.bindings
                    .borrow_mut()
                    .set(expression.name.clone(), object.clone());
                object
            }
        }
    }
}
