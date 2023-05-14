use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use colored::Colorize;

use crab::{compilation::Compilation, syntax::syntax_tree::SyntaxTree};

fn main() {
    let mut source = String::new();
    let mut variables = HashMap::new();
    let mut show_tree = false;

    loop {
        print!("➜  ");
        stdout().flush().unwrap();
        stdin().read_line(&mut source).unwrap();

        match source.trim() {
            "@exit" | "@e" => break,
            "@tree" | "@t" => show_tree = !show_tree,

            source => {
                if !source.is_empty() {
                    let syntax_tree = SyntaxTree::new(source);

                    if show_tree {
                        println!(
                            "{}",
                            format!("{:#?}", syntax_tree.root).truecolor(155, 155, 155)
                        );
                    }

                    let evaluation_result = Compilation::evaluate(syntax_tree, variables.clone());

                    if evaluation_result.diagnostic_bag.diagnostics.is_empty() {
                        println!(
                            "{}",
                            format!("{}", evaluation_result.value).truecolor(255, 255, 255)
                        );
                    } else {
                        for diagnostic in evaluation_result.diagnostic_bag.diagnostics {
                            let (line, column) = diagnostic.position.get_line_and_column(&source);
                            eprintln!(
                                "{}",
                                format!("[line: {line}, column: {column}]").truecolor(255, 255, 0)
                            );
                            eprintln!(
                                "{}",
                                format!("Error: {}", diagnostic.message).truecolor(255, 0, 0)
                            );
                            eprintln!(
                                "\t{}",
                                &source[diagnostic.position.start..diagnostic.position.end]
                            );
                            eprint!("\t");
                            for _ in diagnostic.position.start..diagnostic.position.end {
                                eprint!("{}", "^".truecolor(255, 255, 0));
                            }
                            eprintln!("{}", " --- near here".truecolor(255, 255, 0));
                        }
                    }
                    variables = evaluation_result.variables;
                }
            }
        };
        source.clear();
    }
}
