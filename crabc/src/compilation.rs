use crate::{
    binding::binder::Binder, common::types::Object, evaluator::evaluate,
    syntax::syntax_tree::SyntaxTree,
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(syntax_tree: SyntaxTree) -> EvaluationResult {
        let mut binder = Binder::new(syntax_tree.diagnostics);
        let bound_expression = binder.bind(syntax_tree.root);
        let value = evaluate(bound_expression);
        EvaluationResult {
            diagnostics: binder.diagnostics,
            value,
        }
    }
}

pub struct EvaluationResult {
    pub diagnostics: Vec<String>,
    pub value: Object,
}
