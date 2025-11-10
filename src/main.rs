mod ast;
mod cli;
mod compiler;
mod parser;
mod repl;

fn main() {
    let args = cli::parse_args();
    let verbose = args.verbose_output;

    if let Some(path) = args.script {
        // File execution path
        let source = std::fs::read_to_string(path).expect("Failed to read script");
        if verbose {
            println!("Parsing source: {} bytes", source.len());
        }
        let tokens = parser::tokenize(&source, verbose).expect("Lex error");
        let ast = parser::parse_to_ast(tokens, verbose).expect("Parse error");
        if verbose {
            println!("Compiled AST: {:?}", ast);
        }
        let bytecode = compiler::Compiler::compile(&ast, verbose);
        compiler::Vm::execute(&bytecode, verbose);
    } else {
        // REPL path
        repl::run_repl(verbose);
    }
}
