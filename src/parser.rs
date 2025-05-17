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
    fn build_expr(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, ParseError> {
        match pair.as_rule() {
            Rule::number => {
                let n: f64 = pair.as_str().parse().map_err(|e| {
                    ParseError::AstError(format!("invalid number: {}", e))
                })?;
                Ok(AstNode::Number(n))
            }
            Rule::string => {
                let s = pair.as_str();
                // Trim the surrounding quotes
                let inner = &s[1..s.len() - 1];
                Ok(AstNode::String(inner.to_string()))
            }
            Rule::identifier => Ok(AstNode::Identifier(pair.as_str().to_string())),
            _ => Err(ParseError::AstError(format!(
                "unexpected rule: {:?}",
                pair.as_rule()
            ))),
        }
    }

    let mut stmts = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                let expr_pair = pair.into_inner().next().ok_or_else(|| {
                    ParseError::AstError("empty statement".into())
                })?;
                stmts.push(build_expr(expr_pair)?);
            }
            Rule::EOI => {}
            _ => {}
        }
    }
    Ok(AstNode::Program(stmts))
}