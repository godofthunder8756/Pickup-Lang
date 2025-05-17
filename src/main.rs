mod cli;
mod repl;
mod parser;
mod ast;
mod interpreter;

fn main() {
    let args = cli::parse_args();
    if let Some(path) = args.script {
        // File execution path
        let source = std::fs::read_to_string(path).expect("Failed to read script");
        let tokens = parser::tokenize(&source).expect("Lex error");
        let ast = parser::parse_to_ast(tokens).expect("Parse error");
        let mut interp = interpreter::Interpreter::new();
        interp.run_ast(&ast);
    } else {
        // REPL path
        repl::run_repl();
    }
}