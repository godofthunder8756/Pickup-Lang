use rustyline::Editor;
use crate::{parser, compiler};

pub fn run_repl() {
    let mut rl = Editor::<()>::new();
    println!("Pickup REPL (type 'exit' or Ctrl+C to quit)");
    loop {
        let line = rl.readline(">>> ").unwrap_or_else(|_| String::new());
        if line.trim() == "exit" { break; }
        rl.add_history_entry(&line);
        match parser::tokenize(&line).and_then(|t| parser::parse_to_ast(t)) {
            Ok(ast) => {
                let code = compiler::Compiler::compile(&ast);
                compiler::Vm::execute(&code);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}