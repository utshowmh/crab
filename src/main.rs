mod common;
mod syntax;

use std::io::{stdin, stdout, Write};

use crate::syntax::lexer::Lexer;

fn main() {
    let mut line = String::new();
    loop {
        print!("|> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();

        match line.trim() {
            "@exit" => break,
            line => {
                let mut lexer = Lexer::new(line);
                let tokens = lexer.lex();
                for token in tokens {
                    println!("{token:?}");
                }
            }
        };

        line.clear();
    }
}
