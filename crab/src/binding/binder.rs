use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{
        diagnostic::DiagnosticBag,
        types::{Object, Type},
    },
    syntax::syntax_tree::{
        AssignmentExpression, BinaryExpression, BlockStatement, Expression, ForStatement,
        IfStatement, LiteralExpression, NameExpression, ParenthesizedExpression, Statement,
        UnaryExpression, VarStatement, WhileStatement,
    },
};

use super::{
    bindings::Bindings,
    bound_tree::{
        BoundAssignmentExpression, BoundBinaryExpression, BoundBinaryOperator, BoundBlockStatement,
        BoundExpression, BoundExpressionStatement, BoundForStatement, BoundIfStatement,
        BoundLiteralExpression, BoundPrintStatement, BoundStatement, BoundUnaryExpression,
        BoundUnaryOperator, BoundVarStatement, BoundVariableExpression, BoundWhileStatement,
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
            Statement::Var(statement) => self.bind_var_statement(statement),
            Statement::Block(statement) => self.bind_block_statement(statement),
            Statement::If(statement) => self.bind_if_statement(statement),
            Statement::While(statement) => self.bind_while_statement(statement),
            Statement::For(statement) => self.bind_for_statement(statement),
        }
    }

    fn bind_var_statement(&mut self, statement: VarStatement) -> BoundStatement {
        let bound_expression = self.bind_expression(statement.expression);
        self.bindings.borrow_mut().set(
            statement.identifier.lexeme.clone(),
            bound_expression.get_type(),
        );
        BoundStatement::Var(BoundVarStatement::new(
            statement.identifier.lexeme,
            bound_expression,
        ))
    }

    fn bind_block_statement(&mut self, statement: BlockStatement) -> BoundStatement {
        let mut statements = vec![];
        self.bindings = Rc::new(RefCell::new(Bindings::extend(Rc::clone(&self.bindings))));
        for statement in statement.statements {
            statements.push(self.bind_statement(statement));
        }
        let old_bindings = self.bindings.borrow().outer.clone().unwrap();
        self.bindings = old_bindings;
        BoundStatement::Block(BoundBlockStatement::new(statements))
    }

    fn bind_if_statement(&mut self, statement: IfStatement) -> BoundStatement {
        let condition = self.bind_expression(statement.condition.clone());
        if condition.get_type() == Type::Boolean {
            let consequence = self.bind_statement(*statement.consequence);
            let else_clause =
                (*statement.else_clause).map(|statement| self.bind_statement(statement));
            BoundStatement::If(BoundIfStatement::new(condition, consequence, else_clause))
        } else {
            self.diagnostic_bag.borrow_mut().invalid_expression_type(
                condition.get_position(),
                Type::Boolean,
                condition.get_type(),
            );
            BoundStatement::Expression(BoundExpressionStatement::new(BoundExpression::Literal(
                BoundLiteralExpression::new(Object::Unit, statement.condition.get_position()),
            )))
        }
    }

    fn bind_while_statement(&mut self, statement: WhileStatement) -> BoundStatement {
        let condition = self.bind_expression(statement.condition.clone());
        if condition.get_type() == Type::Boolean {
            let body = self.bind_statement(*statement.body);
            BoundStatement::While(BoundWhileStatement::new(condition, body))
        } else {
            self.diagnostic_bag.borrow_mut().invalid_expression_type(
                condition.get_position(),
                Type::Boolean,
                condition.get_type(),
            );
            BoundStatement::Expression(BoundExpressionStatement::new(BoundExpression::Literal(
                BoundLiteralExpression::new(Object::Unit, statement.condition.get_position()),
            )))
        }
    }

    fn bind_for_statement(&mut self, statement: ForStatement) -> BoundStatement {
        let lower_bound = self.bind_expression(statement.lower_bound);
        let upper_bound = self.bind_expression(statement.upper_bound);
        match (lower_bound.get_type(), upper_bound.get_type()) {
            (Type::Number, Type::Number) => {
                self.bindings
                    .borrow_mut()
                    .set(statement.identifier.lexeme.clone(), Type::Number);
                let body = self.bind_statement(*statement.body);
                BoundStatement::For(BoundForStatement::new(
                    statement.identifier.lexeme,
                    lower_bound,
                    upper_bound,
                    body,
                ))
            }
            (_, _) => {
                self.diagnostic_bag.borrow_mut().invalid_expression_type(
                    lower_bound.get_position(),
                    Type::Number,
                    lower_bound.get_type(),
                );
                self.diagnostic_bag.borrow_mut().invalid_expression_type(
                    upper_bound.get_position(),
                    Type::Number,
                    upper_bound.get_type(),
                );
                BoundStatement::Expression(BoundExpressionStatement::new(BoundExpression::Literal(
                    BoundLiteralExpression::new(Object::Unit, statement.identifier.position),
                )))
            }
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
        BoundExpression::Literal(BoundLiteralExpression::new(
            expression.value.clone(),
            expression.get_position(),
        ))
    }

    fn bind_name_expression(&mut self, expression: NameExpression) -> BoundExpression {
        if let Some(typ) = self.bindings.borrow().get(&expression.identifier.lexeme) {
            BoundExpression::Variable(BoundVariableExpression::new(
                expression.identifier.lexeme.clone(),
                typ,
                expression.get_position(),
            ))
        } else {
            self.diagnostic_bag.borrow_mut().undefined_name(
                expression.get_position(),
                expression.identifier.lexeme.clone(),
            );
            BoundExpression::Literal(BoundLiteralExpression::new(
                Object::Unit,
                expression.get_position(),
            ))
        }
    }

    fn bind_parenthesized_expression(
        &mut self,
        expression: ParenthesizedExpression,
    ) -> BoundExpression {
        self.bind_expression(*expression.expression)
    }

    fn bind_unary_expression(&mut self, expression: UnaryExpression) -> BoundExpression {
        let right = self.bind_expression(*expression.right.clone());
        if let Some(operator) =
            BoundUnaryOperator::bind(expression.operator.kind.clone(), right.get_type())
        {
            BoundExpression::Unary(BoundUnaryExpression::new(
                operator,
                right,
                expression.get_position(),
            ))
        } else {
            self.diagnostic_bag.borrow_mut().invalid_unary_operator(
                expression.get_position(),
                expression.operator.kind.clone(),
                right.get_type(),
            );
            BoundExpression::Literal(BoundLiteralExpression::new(
                Object::Unit,
                expression.get_position(),
            ))
        }
    }

    fn bind_binary_expression(&mut self, expression: BinaryExpression) -> BoundExpression {
        let left = self.bind_expression(*expression.left.clone());
        let right = self.bind_expression(*expression.right.clone());
        if let Some(operator) = BoundBinaryOperator::bind(
            expression.operator.kind.clone(),
            left.get_type(),
            right.get_type(),
        ) {
            BoundExpression::Binary(BoundBinaryExpression::new(
                left,
                operator,
                right,
                expression.get_position(),
            ))
        } else {
            self.diagnostic_bag.borrow_mut().invalid_binary_operator(
                expression.get_position(),
                expression.operator.kind.clone(),
                left.get_type(),
                right.get_type(),
            );
            BoundExpression::Literal(BoundLiteralExpression::new(
                Object::Unit,
                expression.get_position(),
            ))
        }
    }

    fn bind_assignment_expression(&mut self, expression: AssignmentExpression) -> BoundExpression {
        let bound_expression = self.bind_expression(*expression.expression.clone());
        let typ = self.bindings.borrow().get(&expression.identifier.lexeme);
        if let Some(typ) = typ {
            if typ == bound_expression.get_type() {
                self.bindings.borrow_mut().reset(
                    expression.identifier.lexeme.clone(),
                    bound_expression.get_type(),
                );
                BoundExpression::Assignment(BoundAssignmentExpression::new(
                    expression.identifier.lexeme.clone(),
                    bound_expression,
                    expression.get_position(),
                ))
            } else {
                self.diagnostic_bag.borrow_mut().invalid_assignment(
                    expression.get_position(),
                    expression.identifier.lexeme.clone(),
                    typ,
                    bound_expression.get_type(),
                );
                BoundExpression::Literal(BoundLiteralExpression::new(
                    Object::Unit,
                    expression.get_position(),
                ))
            }
        } else {
            self.diagnostic_bag.borrow_mut().undefined_name(
                expression.get_position(),
                expression.identifier.lexeme.clone(),
            );
            BoundExpression::Literal(BoundLiteralExpression::new(
                Object::Unit,
                expression.get_position(),
            ))
        }
    }
}
