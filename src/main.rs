use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

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
                        print_colored_string(format!("{:#?}", syntax_tree.root), (155, 155, 155));
                    }

                    let evaluation_result = Compilation::evaluate(syntax_tree);

                    if evaluation_result.diagnostics.is_empty() {
                        print_colored_string(evaluation_result.value, (255, 255, 255));
                    } else {
                        for diagnostic in evaluation_result.diagnostics {
                            print_colored_string(format!("ERROR: {diagnostic}."), (255, 0, 0));
                        }
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
