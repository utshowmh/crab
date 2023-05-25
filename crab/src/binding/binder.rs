use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{diagnostic::DiagnosticBag, types::Object},
    syntax::syntax_tree::{
        AssignmentExpression, BinaryExpression, Expression, LiteralExpression, NameExpression,
        ParenthesizedExpression, Statement, UnaryExpression,
    },
};

use super::{
    bindings::Bindings,
    bound_tree::{
        BoundAssignmentExpression, BoundBinaryExpression, BoundBinaryOperator, BoundExpression,
        BoundExpressionStatement, BoundLiteralExpression, BoundPrintStatement, BoundStatement,
        BoundUnaryExpression, BoundUnaryOperator, BoundVariableExpression,
    },
};

pub(crate) struct Binder {
    bindings: Rc<RefCell<Bindings>>,
    pub(crate) diagnostic_bag: Rc<RefCell<DiagnosticBag>>,
}

impl Binder {
    pub(crate) fn new(
        bindings: Rc<RefCell<Bindings>>,
        diagnostic_bag: Rc<RefCell<DiagnosticBag>>,
    ) -> Self {
        Self {
            bindings,
            diagnostic_bag,
        }
    }

    pub(crate) fn bind(&mut self, program: Vec<Statement>) -> Vec<BoundStatement> {
        let mut bound_statements = vec![];
        for statement in program {
            bound_statements.push(self.bind_statement(statement));
        }
        bound_statements
    }

    fn bind_statement(&mut self, statement: Statement) -> BoundStatement {
        match statement {
            Statement::Expression(statement) => BoundStatement::Expression(
                BoundExpressionStatement::new(self.bind_expression(statement.expression)),
            ),
            Statement::Print(statement) => BoundStatement::Print(BoundPrintStatement::new(
                self.bind_expression(statement.expression),
            )),
        }
    }

    fn bind_expression(&mut self, expression: Expression) -> BoundExpression {
        match expression {
            Expression::Literal(expression) => self.bind_literal_expression(expression),
            Expression::Name(expression) => self.bind_name_expression(expression),
            Expression::Parenthesized(expression) => self.bind_parenthesized_expression(expression),
            Expression::Unary(expression) => self.bind_unary_expression(expression),
            Expression::Binary(expression) => self.bind_binary_expression(expression),
            Expression::Assignment(expression) => self.bind_assignment_expression(expression),
        }
    }

    fn bind_literal_expression(&self, expression: LiteralExpression) -> BoundExpression {
        BoundExpression::Literal(BoundLiteralExpression::new(expression.value))
    }

    fn bind_name_expression(&mut self, expression: NameExpression) -> BoundExpression {
        if let Some(value) = self.bindings.borrow().get(&expression.identifier.lexeme) {
            BoundExpression::Variable(BoundVariableExpression::new(
                expression.identifier.lexeme.clone(),
                value.get_type(),
            ))
        } else {
            self.diagnostic_bag
                .borrow_mut()
                .undefined_name(expression.identifier.position, expression.identifier.lexeme);
            BoundExpression::Literal(BoundLiteralExpression::new(Object::Unit))
        }
    }

    fn bind_parenthesized_expression(
        &mut self,
        expression: ParenthesizedExpression,
    ) -> BoundExpression {
        self.bind_expression(*expression.expression)
    }

    fn bind_unary_expression(&mut self, expression: UnaryExpression) -> BoundExpression {
        let right = self.bind_expression(*expression.right);
        if let Some(operator) =
            BoundUnaryOperator::bind(expression.operator.kind.clone(), right.get_type())
        {
            BoundExpression::Unary(BoundUnaryExpression::new(operator, right))
        } else {
            self.diagnostic_bag.borrow_mut().invalid_unary_operator(
                expression.operator.position,
                expression.operator.kind,
                right.get_type(),
            );
            BoundExpression::Literal(BoundLiteralExpression::new(Object::Unit))
        }
    }

    fn bind_binary_expression(&mut self, expression: BinaryExpression) -> BoundExpression {
        let left = self.bind_expression(*expression.left);
        let right = self.bind_expression(*expression.right);
        if let Some(operator) = BoundBinaryOperator::bind(
            expression.operator.kind.clone(),
            left.get_type(),
            right.get_type(),
        ) {
            BoundExpression::Binary(BoundBinaryExpression::new(left, operator, right))
        } else {
            self.diagnostic_bag.borrow_mut().invalid_binary_operator(
                expression.operator.position,
                expression.operator.kind,
                left.get_type(),
                right.get_type(),
            );
            BoundExpression::Literal(BoundLiteralExpression::new(Object::Unit))
        }
    }

    fn bind_assignment_expression(&mut self, expression: AssignmentExpression) -> BoundExpression {
        let bound_expression = self.bind_expression(*expression.expression);
        self.bindings.borrow_mut().set(
            expression.identifier.lexeme.clone(),
            bound_expression.get_type().default(),
        );
        BoundExpression::Assignment(BoundAssignmentExpression::new(
            expression.identifier.lexeme,
            bound_expression,
        ))
    }
}
