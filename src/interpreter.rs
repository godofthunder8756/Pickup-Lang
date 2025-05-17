use crate::ast::AstNode;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self { Interpreter }

    pub fn run_ast(&mut self, ast: &AstNode) {
        match ast {
            AstNode::Program(stmts) => {
                for stmt in stmts {
                    self.eval(stmt);
                }
            }
        }
    }

    fn eval(&mut self, node: &AstNode) {
        // TODO: evaluate AST nodes
        println!("Evaluating: {:?}", node);
    }
}