use std::collections::HashMap;

use crate::{
    binding::binder::Binder,
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
        let bound_expression = binder.bind(syntax_tree.root);
        let mut evaluator = Evaluator::new(bound_expression, variables);
        let value = evaluator.evaluate();
        EvaluationResult {
            diagnostic_bag: binder.diagnostic_bag,
            value,
            variables: evaluator.variables,
        }
    }
}

pub struct EvaluationResult {
    pub diagnostic_bag: DiagnosticBag,
    pub value: Object,
    pub variables: HashMap<String, Object>,
}
