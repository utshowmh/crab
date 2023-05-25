use std::{cell::RefCell, rc::Rc};

use crate::{
    binding::{binder::Binder, bindings::Bindings, bound_tree::BoundStatement},
    common::{diagnostic::DiagnosticBag, types::Object},
    evaluator::Evaluator,
    syntax::{lexer::Lexer, parser::Parser, syntax_tree::Statement},
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(source: &str, bindings: Rc<RefCell<Bindings>>) -> EvaluationResult {
        let diagnostic_bag = Rc::new(RefCell::new(DiagnosticBag::new()));
        let mut lexer = Lexer::new(source, Rc::clone(&diagnostic_bag));
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, Rc::clone(&diagnostic_bag));
        let program = parser.parse();
        let mut binder = Binder::new(Rc::clone(&bindings), Rc::clone(&diagnostic_bag));
        let bound_program = binder.bind(program.clone());
        let mut evaluator = Evaluator::new(bound_program.clone(), Rc::clone(&bindings));
        let evaluated_result = evaluator.evaluate();
        EvaluationResult {
            diagnostic_bag: Rc::clone(&diagnostic_bag),
            bindings,
            program,
            bound_program,
            evaluated_result,
        }
    }
}

pub struct EvaluationResult {
    pub diagnostic_bag: Rc<RefCell<DiagnosticBag>>,
    pub bindings: Rc<RefCell<Bindings>>,
    pub program: Vec<Statement>,
    pub bound_program: Vec<BoundStatement>,
    pub evaluated_result: Object,
}
