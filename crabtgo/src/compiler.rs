use crab::binding::bound_tree::{BoundExpression, BoundStatement};

pub(crate) struct Compiler {
    bound_statements: Vec<BoundStatement>,
}

impl Compiler {
    pub(crate) fn new(bound_statements: Vec<BoundStatement>) -> Self {
        Self { bound_statements }
    }

    pub(crate) fn compile(&mut self) -> String {
        let mut compiled_code = String::from("package main\nimport (\"fmt\")\n");
        compiled_code.push_str(&format!(
            "func main() {{\n{}}}\n",
            self.compile_statements(&self.bound_statements.clone())
        ));
        compiled_code
    }

    fn compile_statements(&mut self, statements: &[BoundStatement]) -> String {
        let mut compiled_code = String::new();
        for statement in statements {
            compiled_code.push_str(&self.compile_statement(statement));
        }
        compiled_code
    }

    fn compile_statement(&mut self, statement: &BoundStatement) -> String {
        let mut compiled_statement = String::new();
        match statement {
            BoundStatement::Expression(statement) => compiled_statement.push_str(&format!(
                "{}\n",
                self.compile_expression(&statement.expression)
            )),
            BoundStatement::Print(statement) => compiled_statement.push_str(&format!(
                "fmt.Println({})\n",
                self.compile_expression(&statement.expression)
            )),
            BoundStatement::Var(statement) => {
                compiled_statement.push_str(&format!(
                    "{}:={}\n",
                    statement.name,
                    self.compile_expression(&statement.expression)
                ));
            }
            BoundStatement::Block(statement) => compiled_statement.push_str(&format!(
                "{{{}}}",
                self.compile_statements(&statement.statements)
            )),
            BoundStatement::If(statement) => {
                compiled_statement.push_str(&format!(
                    "if {} {{\n {} }}\n",
                    self.compile_expression(&statement.condition),
                    self.compile_statement(&statement.consequence)
                ));
                if let Some(alternative) = *statement.else_clause.clone() {
                    compiled_statement.push_str(&format!(
                        "else {{\n {} }}\n",
                        self.compile_statement(&alternative)
                    ));
                }
            }
            BoundStatement::While(statement) => {
                compiled_statement.push_str(&format!(
                    "for {} {{\n {} }}\n",
                    self.compile_expression(&statement.condition),
                    self.compile_statement(&statement.body)
                ));
            }
            BoundStatement::For(statement) => {
                compiled_statement.push_str(&format!(
                    "for {} := {}; {} < {}; {}++ {{\n {} }}\n",
                    &statement.identifier,
                    self.compile_expression(&statement.lower_bound),
                    self.compile_expression(&statement.lower_bound),
                    self.compile_expression(&statement.upper_bound),
                    &statement.identifier,
                    self.compile_statement(&statement.body)
                ));
            }
        }
        compiled_statement
    }

    fn compile_expression(&mut self, bound_expression: &BoundExpression) -> String {
        let mut compiled_expression = String::new();
        match bound_expression {
            BoundExpression::Literal(expression) => {
                compiled_expression.push_str(&format!("{}", expression.value));
            }
            BoundExpression::Variable(expression) => {
                compiled_expression.push_str(&format!("{}", expression.name))
            }
            BoundExpression::Unary(expression) => {
                compiled_expression.push_str(&format!("{}", expression.operator.operation_kind));
                compiled_expression
                    .push_str(&format!("{}", self.compile_expression(&expression.right)));
            }
            BoundExpression::Binary(expression) => {
                compiled_expression
                    .push_str(&format!("{}", self.compile_expression(&expression.left)));
                compiled_expression.push_str(&format!("{}", expression.operator.operation_kind));
                compiled_expression
                    .push_str(&format!("{}", self.compile_expression(&expression.right)));
            }
            BoundExpression::Assignment(expression) => compiled_expression.push_str(&format!(
                "{}={}",
                expression.name,
                self.compile_expression(&expression.expression)
            )),
        }
        compiled_expression
    }
}
