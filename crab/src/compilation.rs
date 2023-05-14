use std::collections::HashMap;

use crate::{
    binding::{binder::Binder, bound_tree::BoundStatement},
    common::{diagnostic::DiagnosticBag, types::Object},
    evaluator::Evaluator,
    syntax::syntax_tree::SyntaxTree,
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(
        syntax_tree: SyntaxTree,
        variables: HashMap<String, Object>,
    ) -> EvaluationResult {
        let mut binder = Binder::new(variables.clone(), syntax_tree.diagnostic_bag);
        let bound_tree = binder.bind(syntax_tree.program);
        let mut evaluator = Evaluator::new(bound_tree.clone(), variables);
        evaluator.evaluate();
        EvaluationResult {
            diagnostic_bag: binder.diagnostic_bag,
            variables: evaluator.variables,
            bound_tree,
        }
    }
}

pub struct EvaluationResult {
    pub diagnostic_bag: DiagnosticBag,
    pub variables: HashMap<String, Object>,
    pub bound_tree: Vec<BoundStatement>,
}
