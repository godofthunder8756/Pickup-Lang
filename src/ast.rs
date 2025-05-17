/// AST node types for Pickup
#[derive(Debug)]
pub enum AstNode {
    // TODO: define expressions, statements, function defs, etc.
    Program(Vec<AstNode>),
}