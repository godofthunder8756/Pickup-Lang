/// Abstract syntax tree node types for Pickup.
#[derive(Debug, Clone)]
pub enum AstNode {
    /// Top level program consisting of a list of statements/expressions.
    Program(Vec<AstNode>),
    /// Numeric literal.
    Number(f64),
    /// String literal.
    String(String),
    /// Identifier token.
    Identifier(String),
    /// Boolean literal
    Boolean(bool),
    /// Assignment operation (identifier, value)
    Assignment(Box<AstNode>, Box<AstNode>),
    /// Binary operations (left, operator, right)
    BinaryOp(Box<AstNode>, String, Box<AstNode>),
    /// Function call (function name, arguments)
    FunctionCall(String, Vec<AstNode>),
    /// Print statement
    Print(Box<AstNode>),
}