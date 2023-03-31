use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

use colored::Colorize;

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
                        print_colored_string(format!("{bound_tree:#?}"), (155, 155, 155));
                    }

                    if !binder.diagnostics.is_empty() {
                        for diagnostic in binder.diagnostics {
                            print_colored_string(format!("ERROR: {diagnostic}."), (255, 0, 0));
                        }
                    } else {
                        print_colored_string(evaluate(bound_tree), (255, 255, 255));
                    }
                }
            }
        };

        line.clear();
    }
}

fn print_colored_string<T>(output: T, color: (u8, u8, u8))
where
    T: Display,
{
    println!(
        "{}",
        format!("{output}").truecolor(color.0, color.1, color.2)
    );
}
