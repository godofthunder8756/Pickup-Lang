use pest::Parser;
use crate::ast::AstNode;
use thiserror::Error;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct PickupParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("AST construction failed: {0}")]
    AstError(String),
}

pub fn tokenize(source: &str) -> Result<pest::iterators::Pairs<Rule>, ParseError> {
    Ok(PickupParser::parse(Rule::program, source)?)
}

pub fn parse_to_ast(pairs: pest::iterators::Pairs<Rule>) -> Result<AstNode, ParseError> {
    // TODO: walk Pairs and build AST
    unimplemented!()
}