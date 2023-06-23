use std::{
    cell::RefCell,
    io::{stdin, stdout, Write},
    rc::Rc,
};

use colored::Colorize;

use crab::{
    binding::bindings::Bindings, compilation::Compilation, interpreter::environment::Environment,
};

fn main() {
    let mut source = String::new();
    let mut bindings = Rc::new(RefCell::new(Bindings::default()));
    let mut environment = Rc::new(RefCell::new(Environment::default()));
    let mut show_syntax_tree = false;
    let mut show_bound_tree = false;
    let mut stdout = stdout();

    loop {
        print!("➜  ");
        stdout.flush().unwrap();
        for line in stdin().lines() {
            let line = line.unwrap();
            if line.trim().is_empty() {
                break;
            }
            source.push_str(&line);
            print!(".. ");
            stdout.flush().unwrap();
        }

        match source.trim() {
            "@exit" | "@e" => break,
            "@syntax_tree" | "@st" => show_syntax_tree = !show_syntax_tree,
            "@bound_tree" | "@bt" => show_bound_tree = !show_bound_tree,

            source => {
                if !source.is_empty() {
                    let compilation_result = Compilation::evaluate(
                        source,
                        Rc::clone(&bindings),
                        Rc::clone(&environment),
                    );

                    if compilation_result
                        .diagnostic_bag
                        .borrow()
                        .diagnostics
                        .is_empty()
                    {
                        println!(
                            "{}",
                            format!("{}", compilation_result.evaluated_result)
                                .truecolor(255, 255, 255)
                        )
                    }

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
                        let line = diagnostic.position.get_line(source);
                        eprintln!(
                            "{}",
                            format!("[error in line: {line}]").truecolor(255, 255, 0)
                        );
                        eprintln!(
                            "{}",
                            format!("Error: {}.", diagnostic.message).truecolor(255, 0, 0)
                        );
                        eprintln!(
                            "\t{}",
                            &source[diagnostic.position.start..diagnostic.position.end]
                        );
                        eprint!("\t");
                        for _ in diagnostic.position.start..diagnostic.position.end {
                            eprint!("{}", "^".truecolor(255, 255, 0));
                        }
                        eprintln!("{}", " --- here".truecolor(255, 255, 0));
                    }

                    bindings = compilation_result.bindings;
                    environment = compilation_result.environment;
                }
            }
        };
        source.clear();
    }
}
