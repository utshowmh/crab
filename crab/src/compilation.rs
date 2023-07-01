use std::{cell::RefCell, rc::Rc};

use crate::{
    binding::{binder::Binder, bindings::Bindings, bound_tree::BoundStatement},
    common::diagnostic::DiagnosticBag,
    syntax::{lexer::Lexer, parser::Parser, syntax_tree::Statement},
};

pub struct Compilation {
    pub diagnostic_bag: Rc<RefCell<DiagnosticBag>>,
    pub bindings: Rc<RefCell<Bindings>>,
    pub unbound_program: Vec<Statement>,
    pub bound_program: Vec<BoundStatement>,
}

impl Compilation {
    pub fn compile(source: &str, bindings: Rc<RefCell<Bindings>>) -> Self {
        let diagnostic_bag = Rc::new(RefCell::new(DiagnosticBag::new()));
        let mut lexer = Lexer::new(source, Rc::clone(&diagnostic_bag));
        let tokens = lexer.lex();
        let mut parser = Parser::new(tokens, Rc::clone(&diagnostic_bag));
        let unbound_program = parser.parse();
        let mut binder = Binder::new(Rc::clone(&bindings), Rc::clone(&diagnostic_bag));
        let bound_program = binder.bind(unbound_program.clone());
        Self {
            diagnostic_bag: Rc::clone(&diagnostic_bag),
            bindings,
            unbound_program,
            bound_program,
        }
    }
}
