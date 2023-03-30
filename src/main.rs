use std::io::{stdin, stdout, Write};

use crab::syntax::syntax_tree::SyntaxTree;

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
                        println!("{:#?}", syntax_tree.root);
                    }

                    if !syntax_tree.diagnostics.is_empty() {
                        for diagnostic in syntax_tree.diagnostics {
                            eprintln!("ERROR: {diagnostic}.");
                        }
                    }
                }
            }
        };

        line.clear();
    }
}
