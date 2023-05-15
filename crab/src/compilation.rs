use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    binding::{binder::Binder, bound_tree::BoundStatement},
    common::{diagnostic::DiagnosticBag, types::Object},
    evaluator::Evaluator,
    syntax::{lexer::Lexer, parser::Parser, syntax_tree::Statement},
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(source: &str, variables: HashMap<String, Object>) -> CompilationResult {
        let diagnostic_bag = Rc::new(RefCell::new(DiagnosticBag::new()));
        let mut lexer = Lexer::new(source, Rc::clone(&diagnostic_bag));
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, Rc::clone(&diagnostic_bag));
        let program = parser.parse();
        let mut binder = Binder::new(variables.clone(), Rc::clone(&diagnostic_bag));
        let bound_program = binder.bind(program.clone());
        let mut evaluator = Evaluator::new(bound_program.clone(), variables.clone());
        evaluator.evaluate();
        CompilationResult {
            diagnostic_bag: Rc::clone(&diagnostic_bag),
            variables,
            program,
            bound_program,
        }
    }
}

pub struct CompilationResult {
    pub diagnostic_bag: Rc<RefCell<DiagnosticBag>>,
    pub variables: HashMap<String, Object>,
    pub program: Vec<Statement>,
    pub bound_program: Vec<BoundStatement>,
}
