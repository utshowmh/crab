use crate::{
    binding::binder::Binder,
    common::{diagnostic::DiagnosticBag, types::Object},
    evaluator::evaluate,
    syntax::syntax_tree::SyntaxTree,
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(syntax_tree: SyntaxTree) -> EvaluationResult {
        let mut binder = Binder::new(syntax_tree.diagnostic_bag);
        let bound_expression = binder.bind(syntax_tree.root);
        let value = evaluate(bound_expression);
        EvaluationResult {
            diagnostic_bag: binder.diagnostic_bag,
            value,
        }
    }
}

pub struct EvaluationResult {
    pub diagnostic_bag: DiagnosticBag,
    pub value: Object,
}
