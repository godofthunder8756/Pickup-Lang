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
}