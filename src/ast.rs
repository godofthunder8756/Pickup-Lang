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
    /// Nil value
    Nil,
    /// Assignment operation (identifier, value)
    Assignment(Box<AstNode>, Box<AstNode>),
    /// Local variable assignment (identifier name, value)
    LocalAssignment(String, Box<AstNode>),
    /// Binary operations (left, operator, right)
    BinaryOp(Box<AstNode>, String, Box<AstNode>),
    /// Unary not operation
    Not(Box<AstNode>),
    /// Function call (function name, arguments)
    FunctionCall(String, Vec<AstNode>),
    /// Method call (object, method chain, arguments)
    MethodCall(Box<AstNode>, Vec<String>, Vec<AstNode>),
    /// Print statement
    Print(Box<AstNode>),
    /// Table/Array literal [element1, element2, ...]
    Table(Vec<AstNode>),
    /// Dictionary/Object literal { key = value, ... }
    Dictionary(Vec<(String, AstNode)>),
    /// Table/Array index access (table, index)
    Index(Box<AstNode>, Box<AstNode>),
    /// Member access with dot notation (object, field chain)
    MemberAccess(Box<AstNode>, Vec<String>),
    /// Import statement (module path)
    Import(String),
    /// If statement (condition, then_block, elseif_clauses, else_block)
    If(Box<AstNode>, Vec<AstNode>, Vec<(AstNode, Vec<AstNode>)>, Option<Vec<AstNode>>),
    /// While loop (condition, body)
    While(Box<AstNode>, Vec<AstNode>),
    /// For loop (variable, start, end, step, body)
    For(String, Box<AstNode>, Box<AstNode>, Option<Box<AstNode>>, Vec<AstNode>),
    /// Function definition (name, parameters, body)
    FunctionDef(String, Vec<String>, Vec<AstNode>),
    /// Lambda/Anonymous function (parameters, body)
    Lambda(Vec<String>, Vec<AstNode>),
    /// Return statement (optional value)
    Return(Option<Box<AstNode>>),
    /// Break statement
    Break,
    /// Continue statement
    Continue,
    /// Try-catch statement (try_block, catch_var, catch_block)
    TryCatch(Vec<AstNode>, Option<String>, Vec<AstNode>),
    /// Throw statement (error value)
    Throw(Box<AstNode>),
}
