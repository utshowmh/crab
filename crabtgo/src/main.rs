mod compiler;

use std::{
    cell::RefCell,
    env::args,
    fs::{read_to_string, File},
    io::Write,
    process::exit,
    rc::Rc,
};

use colored::Colorize;

use crate::compiler::Compiler;
use crab::{binding::bindings::Bindings, compilation::Compilation};

const FILE_EXTENSION: &str = ".crab";

fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Invalid number of arguments.");
            exit(65);
        }
    }
}

fn run_file(path: &str) {
    let source = read_to_string(path).unwrap();
    let compilation = Compilation::compile(&source, Rc::new(RefCell::new(Bindings::default())));
    if compilation.diagnostic_bag.borrow().diagnostics.is_empty() {
        let mut evaluator = Compiler::new(compilation.bound_program.clone());
        let compiled_code = evaluator.compile();
        let output_path: Vec<&str> = path.split(FILE_EXTENSION).collect();
        let output_path = output_path[0];
        let mut file = File::create(format!("{output_path}.go")).unwrap();
        file.write_all(&compiled_code.as_bytes()).unwrap();
    }
    for diagnostic in &compilation.diagnostic_bag.borrow().diagnostics {
        let line = diagnostic.position.get_line(&source);
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
}
