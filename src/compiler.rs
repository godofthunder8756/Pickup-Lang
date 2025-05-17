use crate::ast::AstNode;

/// Bytecode instructions for the Pickup VM.
#[derive(Debug, Clone)]
pub enum Instruction {
    PushNumber(f64),
    PushString(String),
    PushIdentifier(String),
    Print,
}

/// Compile an AST to bytecode instructions.
pub struct Compiler;

impl Compiler {
    pub fn compile(ast: &AstNode) -> Vec<Instruction> {
        let mut code = Vec::new();
        Self::compile_node(ast, &mut code);
        code
    }

    fn compile_node(node: &AstNode, code: &mut Vec<Instruction>) {
        match node {
            AstNode::Program(stmts) => {
                for stmt in stmts {
                    Self::compile_node(stmt, code);
                    code.push(Instruction::Print);
                }
            }
            AstNode::Number(n) => code.push(Instruction::PushNumber(*n)),
            AstNode::String(s) => code.push(Instruction::PushString(s.clone())),
            AstNode::Identifier(id) => code.push(Instruction::PushIdentifier(id.clone())),
        }
    }
}

/// Simple bytecode interpreter.
pub struct Vm;

impl Vm {
    pub fn execute(code: &[Instruction]) {
        let mut stack: Vec<String> = Vec::new();
        for inst in code {
            match inst {
                Instruction::PushNumber(n) => stack.push(n.to_string()),
                Instruction::PushString(s) => stack.push(s.clone()),
                Instruction::PushIdentifier(id) => stack.push(format!("<{}>", id)),
                Instruction::Print => {
                    if let Some(val) = stack.pop() {
                        println!("{}", val);
                    }
                }
            }
        }
    }
}
