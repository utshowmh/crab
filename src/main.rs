use std::io::{stdin, stdout, Write};

use crab::{binding::binder::Binder, evaluator::evaluate, syntax::syntax_tree::SyntaxTree};

fn main() {
    let mut line = String::new();
    let mut show_tree = false;

    loop {
        print!("|> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();

        match line.trim() {
            "@exit" | "@e" => break,
            "@tree" | "@t" => show_tree = !show_tree,

            line => {
                if !line.is_empty() {
                    let syntax_tree = SyntaxTree::new(line);

                    let mut binder = Binder::new(syntax_tree.diagnostics);
                    let bound_tree = binder.bind(syntax_tree.root);

                    if show_tree {
                        println!("{bound_tree:#?}");
                    }

                    if !binder.diagnostics.is_empty() {
                        for diagnostic in binder.diagnostics {
                            eprintln!("ERROR: {diagnostic}.");
                        }
                    } else {
                        println!("{}", evaluate(bound_tree));
                    }
                }
            }
        };

        line.clear();
    }
}
