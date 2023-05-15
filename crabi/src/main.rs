use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use colored::Colorize;

use crab::compilation::Compilation;

fn main() {
    let mut source = String::new();
    let mut variables = HashMap::new();
    let mut show_syntax_tree = false;
    let mut show_bound_tree = false;

    loop {
        print!("➜  ");
        stdout().flush().unwrap();
        stdin().read_line(&mut source).unwrap();

        match source.trim() {
            "@exit" | "@e" => break,
            "@syntax_tree" | "@st" => show_syntax_tree = !show_syntax_tree,
            "@bound_tree" | "@bt" => show_bound_tree = !show_bound_tree,

            source => {
                if !source.is_empty() {
                    let compilation_result = Compilation::evaluate(source, variables.clone());

                    if show_syntax_tree {
                        println!(
                            "{}",
                            format!("{:#?}", compilation_result.program).truecolor(155, 155, 155)
                        );
                    }

                    if show_bound_tree {
                        println!(
                            "{}",
                            format!("{:#?}", compilation_result.bound_program)
                                .truecolor(155, 155, 155)
                        );
                    }

                    for diagnostic in &compilation_result.diagnostic_bag.borrow().diagnostics {
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

                    variables = compilation_result.variables;
                }
            }
        };
        source.clear();
    }
}
