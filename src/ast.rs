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
    /// Nil literal
    Nil,
    /// Assignment operation (identifier, value)
    Assignment(Box<AstNode>, Box<AstNode>),
    /// Binary operations (left, operator, right)
    BinaryOp(Box<AstNode>, String, Box<AstNode>),
    /// Function call (function name, arguments)
    FunctionCall(String, Vec<AstNode>),
    /// Print statement
    Print(Box<AstNode>),
    /// If statement (condition, then_block, else_block)
    If(Box<AstNode>, Vec<AstNode>, Option<Vec<AstNode>>),
    /// While loop (condition, body)
    While(Box<AstNode>, Vec<AstNode>),
    /// For loop (variable, start, end, body)
    For(String, Box<AstNode>, Box<AstNode>, Vec<AstNode>),
    /// Function definition (name, parameters, body)
    Function(String, Vec<String>, Vec<AstNode>),
    /// Return statement (optional value)
    Return(Option<Box<AstNode>>),
    /// Block of statements
    Block(Vec<AstNode>),
    /// Table/Array literal (key-value pairs or indexed values)
    Table(Vec<(AstNode, AstNode)>),
    /// Table access (table, key)
    TableAccess(Box<AstNode>, Box<AstNode>),
}