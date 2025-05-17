use rustyline::DefaultEditor;
use crate::{parser, compiler};

pub fn run_repl(verbose: bool) {
    let mut rl = DefaultEditor::new().unwrap();
    println!("Pickup REPL (type 'exit' or Ctrl+C to quit)");
    loop {
        let line = rl.readline(">>> ").unwrap_or_else(|_| String::new());
        if line.trim() == "exit" { break; }
        let _ = rl.add_history_entry(&line);
        match parser::tokenize(&line, verbose).and_then(|t| parser::parse_to_ast(t, verbose)) {
            Ok(ast) => {
                let code = compiler::Compiler::compile(&ast, verbose);
                compiler::Vm::execute(&code, verbose);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}