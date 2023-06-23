use std::{cell::RefCell, rc::Rc};

use crate::{
    binding::bound_tree::{
        BoundBinaryOperationKind, BoundExpression, BoundStatement, BoundUnaryOperationKind,
    },
    common::types::Object,
};

use super::environment::Environment;

pub(crate) struct Evaluator {
    bound_statements: Vec<BoundStatement>,
    pub(super) bindings: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub(crate) fn new(
        bound_statements: Vec<BoundStatement>,
        bindings: Rc<RefCell<Environment>>,
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
            BoundStatement::Var(statement) => {
                let object = self.evaluate_expression(&statement.expression);
                self.bindings
                    .borrow_mut()
                    .set(statement.name, object.clone());
                object
            }
            BoundStatement::Block(statement) => {
                self.bindings =
                    Rc::new(RefCell::new(Environment::extend(Rc::clone(&self.bindings))));
                for statement in statement.statements {
                    self.evaluate_statement(statement);
                }
                let old_bindings = self
                    .bindings
                    .borrow()
                    .outer
                    .clone()
                    .unwrap_or(Rc::new(RefCell::new(Environment::default())));
                self.bindings = old_bindings;
                Object::Unit
            }
            BoundStatement::If(statement) => {
                if self.evaluate_expression(&statement.condition).as_boolean() {
                    self.evaluate_statement(*statement.consequence)
                } else {
                    match *statement.else_clause {
                        Some(statement) => self.evaluate_statement(statement),
                        None => Object::Unit,
                    }
                }
            }
            BoundStatement::While(statement) => {
                loop {
                    let value = self.evaluate_expression(&statement.condition).as_boolean();
                    if !value {
                        break;
                    }
                    self.evaluate_statement(*statement.body.clone());
                }
                Object::Unit
            }
            BoundStatement::For(statement) => {
                let mut lower_bound = self.evaluate_expression(&statement.lower_bound).as_number();
                let upper_bound = self.evaluate_expression(&statement.upper_bound).as_number();
                self.bindings
                    .borrow_mut()
                    .set(statement.identifier.clone(), Object::Number(lower_bound));
                while lower_bound < upper_bound {
                    self.evaluate_statement(*statement.body.clone());
                    lower_bound = self
                        .bindings
                        .borrow()
                        .get(&statement.identifier)
                        .unwrap()
                        .as_number();
                    self.bindings.borrow_mut().reset(
                        statement.identifier.clone(),
                        Object::Number(lower_bound + 1),
                    );
                }
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

                    BoundBinaryOperationKind::Greater => {
                        Object::Boolean(left.as_number() > right.as_number())
                    }
                    BoundBinaryOperationKind::Lesser => {
                        Object::Boolean(left.as_number() < right.as_number())
                    }
                    BoundBinaryOperationKind::GreaterEqual => {
                        Object::Boolean(left.as_number() >= right.as_number())
                    }
                    BoundBinaryOperationKind::LesserEqual => {
                        Object::Boolean(left.as_number() <= right.as_number())
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
                    .reset(expression.name.clone(), object.clone());
                object
            }
        }
    }
}
