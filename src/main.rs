use std::io::{stdin, stdout, Write};

use colored::Colorize;

use crabc::{compilation::Compilation, syntax::syntax_tree::SyntaxTree};

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

                    if show_tree {
                        println!(
                            "{}",
                            format!("{:#?}", syntax_tree.root).truecolor(155, 155, 155)
                        );
                    }

                    let evaluation_result = Compilation::evaluate(syntax_tree);

                    if evaluation_result.diagnostic_bag.diagnostics.is_empty() {
                        println!(
                            "{}",
                            format!("{}", evaluation_result.value).truecolor(255, 255, 255)
                        );
                    } else {
                        for diagnostic in evaluation_result.diagnostic_bag.diagnostics {
                            eprintln!(
                                "{}",
                                format!("Error: {}.", diagnostic.message).truecolor(255, 0, 0),
                            );
                            eprintln!("   {line}");
                            eprint!("   ");
                            for _ in 0..diagnostic.position.start {
                                eprint!(" ");
                            }
                            for _ in diagnostic.position.start..diagnostic.position.end {
                                eprint!("{}", "^".truecolor(255, 255, 0));
                            }
                            eprintln!("{}", " here".truecolor(255, 255, 0));
                        }
                    }
                }
            }
        };

        line.clear();
    }
}
