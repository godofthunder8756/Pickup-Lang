use crate::ast::AstNode;

/// Bytecode instructions for the Pickup VM.
#[derive(Debug, Clone)]
pub enum Instruction {
    PushNumber(f64),
    PushString(String),
    LoadVar(String),
    StoreVar(String),
    Add,
    Sub,
    Mul,
    Div,
    Concat,
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
                }
            }
            AstNode::Number(n) => code.push(Instruction::PushNumber(*n)),
            AstNode::String(s) => code.push(Instruction::PushString(s.clone())),
            AstNode::Identifier(id) => code.push(Instruction::LoadVar(id.clone())),
            AstNode::Assignment(var, expr) => {
                // var is Identifier
                if let AstNode::Identifier(name) = &**var {
                    Self::compile_node(expr, code);
                    code.push(Instruction::StoreVar(name.clone()));
                }
            }
            AstNode::BinaryOp(left, op, right) => {
                Self::compile_node(left, code);
                Self::compile_node(right, code);
                match op.as_str() {
                    "+" => code.push(Instruction::Add),
                    "-" => code.push(Instruction::Sub),
                    "*" => code.push(Instruction::Mul),
                    "/" => code.push(Instruction::Div),
                    ".." => code.push(Instruction::Concat),
                    _ => {}
                }
            }
            AstNode::Print(expr) => {
                Self::compile_node(expr, code);
                code.push(Instruction::Print);
            }
            _ => {}
        }
    }
}

/// Value types for the Pickup VM.
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
        }
    }
}

/// Simple bytecode interpreter.
pub struct Vm;

impl Vm {
    pub fn execute(code: &[Instruction]) {
        use std::collections::HashMap;
        let mut stack: Vec<String> = Vec::new();
        let mut vars: HashMap<String, String> = HashMap::new();
        for inst in code {
            match inst {
                Instruction::PushNumber(n) => stack.push(n.to_string()),
                Instruction::PushString(s) => stack.push(s.clone()),
                Instruction::LoadVar(name) => {
                    let val = vars.get(name).cloned().unwrap_or_default();
                    stack.push(val);
                }
                Instruction::StoreVar(name) => {
                    if let Some(val) = stack.pop() {
                        vars.insert(name.clone(), val);
                    }
                }
                Instruction::Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let res = a.parse::<f64>().unwrap() + b.parse::<f64>().unwrap();
                    stack.push(res.to_string());
                }
                Instruction::Sub => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let res = a.parse::<f64>().unwrap() - b.parse::<f64>().unwrap();
                    stack.push(res.to_string());
                }
                Instruction::Mul => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let res = a.parse::<f64>().unwrap() * b.parse::<f64>().unwrap();
                    stack.push(res.to_string());
                }
                Instruction::Div => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let res = a.parse::<f64>().unwrap() / b.parse::<f64>().unwrap();
                    stack.push(res.to_string());
                }
                Instruction::Concat => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(format!("{}{}", a, b));
                }
                Instruction::Print => {
                    if let Some(val) = stack.pop() {
                        println!("{}", val);
                    }
                }
            }
        }
    }
}
