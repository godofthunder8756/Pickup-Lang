use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use crate::ast::AstNode;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct PickupParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("AST construction failed: {0}")]
    AstError(String),
}

pub fn tokenize(source: &str) -> Result<Pairs<Rule>, ParseError> {
    println!("Tokenizing source:\n{}", source);
    let pairs = PickupParser::parse(Rule::program, source)?;
    println!("Parsed pairs: {:?}", pairs);
    Ok(pairs)
}

pub fn parse_to_ast(pairs: Pairs<Rule>) -> Result<AstNode, ParseError> {
    let mut statements = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for inner_pair in pair.into_inner() {
                    if let Some(stmt) = parse_statement(inner_pair)? {
                        statements.push(stmt);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(AstNode::Program(statements))
}

fn parse_statement(pair: Pair<Rule>) -> Result<Option<AstNode>, ParseError> {
    println!("Parsing statement: {:?}", pair);
    match pair.as_rule() {
        Rule::assignment => {
            let inner: Vec<_> = pair.into_inner().collect();
            println!("Assignment inner: {:?}", inner);
            
            // First element is always the identifier
            let id_node = AstNode::Identifier(inner[0].as_str().to_string());
            
            // Check if this is a binary operation (x = a + b)
            if inner.len() >= 4 && (inner[2].as_rule() == Rule::op_add || 
                                    inner[2].as_rule() == Rule::op_sub || 
                                    inner[2].as_rule() == Rule::op_mul || 
                                    inner[2].as_rule() == Rule::op_div) {
                let left = parse_term(inner[1].clone())?;
                let op = inner[2].as_str().to_string();
                let right = parse_term(inner[3].clone())?;
                
                let binary_op = AstNode::BinaryOp(Box::new(left), op, Box::new(right));
                return Ok(Some(AstNode::Assignment(Box::new(id_node), Box::new(binary_op))));
            }
            
            // Regular assignment
            let value_node = parse_expression(inner[1].clone())?;
            Ok(Some(AstNode::Assignment(Box::new(id_node), Box::new(value_node))))
        },
        Rule::function_call => {
            let function_call = parse_function_call(pair)?;
            match function_call {
                AstNode::Print(_) => Ok(Some(function_call)),
                _ => Ok(Some(function_call))
            }
        },
        Rule::print_stmt => {
            let mut inner = pair.into_inner();
            let expr = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing expression in print statement".into()))?;
            let expr_node = parse_expression(expr)?;
            Ok(Some(AstNode::Print(Box::new(expr_node))))
        },
        Rule::expression => {
            let expr = parse_expression(pair)?;
            Ok(Some(expr))
        },
        _ => Ok(None),
    }
}

fn parse_function_call(pair: Pair<Rule>) -> Result<AstNode, ParseError> {
    println!("Parsing function call: {:?}", pair);
    let inner: Vec<_> = pair.into_inner().collect();
    
    if inner.is_empty() {
        return Err(ParseError::AstError("Empty function call".into()));
    }
    
    // First item is the function name
    let func_name = &inner[0];
    
    // Special case for 'print' with string concatenation
    if func_name.as_str() == "print" && inner.len() >= 4 {
        if inner.len() == 4 && inner[2].as_rule() == Rule::op_concat {
            // Handle print with concatenation: print(str .. var)
            let left = parse_term(inner[1].clone())?;
            let right = parse_term(inner[3].clone())?;
            let concat = AstNode::BinaryOp(
                Box::new(left),
                "..".to_string(),
                Box::new(right)
            );
            return Ok(AstNode::Print(Box::new(concat)));
        }
    }
    
    // Regular function call or print with single argument
    if inner.len() >= 2 {
        let arg = parse_expression(inner[1].clone())?;
        
        if func_name.as_str() == "print" {
            Ok(AstNode::Print(Box::new(arg)))
        } else {
            let mut args = Vec::new();
            args.push(arg);
            
            // Add any remaining arguments
            for i in 2..inner.len() {
                args.push(parse_expression(inner[i].clone())?);
            }
            
            Ok(AstNode::FunctionCall(func_name.as_str().to_string(), args))
        }
    } else {
        // Function call with no arguments
        Ok(AstNode::FunctionCall(func_name.as_str().to_string(), vec![]))
    }
}

fn parse_expression(pair: Pair<Rule>) -> Result<AstNode, ParseError> {
    println!("Parsing expression: {:?}", pair);
    
    match pair.as_rule() {
        Rule::expression => {
            let pairs = pair.into_inner().collect::<Vec<_>>();
            println!("Expression inner pairs: {:?}", pairs);
            
            if pairs.is_empty() {
                return Err(ParseError::AstError("Empty expression".into()));
            }
            
            // Handle binary expressions (a + b, etc.)
            if pairs.len() >= 3 {
                let mut left = parse_term(pairs[0].clone())?;
                
                let mut i = 1;
                while i + 1 < pairs.len() {
                    let op = pairs[i].as_str().to_string();
                    let right = parse_term(pairs[i+1].clone())?;
                    left = AstNode::BinaryOp(Box::new(left), op, Box::new(right));
                    i += 2;
                }
                
                return Ok(left);
            }
            
            // Single term
            parse_term(pairs[0].clone())
        },
        Rule::term => parse_term(pair),
        _ => parse_term(pair) // Try parsing as a term
    }
}

fn parse_term(pair: Pair<Rule>) -> Result<AstNode, ParseError> {
    match pair.as_rule() {
        Rule::number => {
            let value = pair.as_str().parse::<f64>().map_err(|_| {
                ParseError::AstError(format!("Failed to parse number: {}", pair.as_str()))
            })?;
            Ok(AstNode::Number(value))
        },
        Rule::string => {
            // Remove the quotes from the string literal
            let text = pair.as_str();
            let value = &text[1..text.len() - 1];
            Ok(AstNode::String(value.to_string()))
        },
        Rule::boolean => {
            let value = pair.as_str() == "true";
            Ok(AstNode::Boolean(value))
        },
        Rule::identifier => Ok(AstNode::Identifier(pair.as_str().to_string())),
        Rule::function_call => parse_function_call(pair),
        Rule::op_add | Rule::op_sub | Rule::op_mul | Rule::op_div | Rule::op_concat => {
            // Binary operators are handled in parse_expression, this should not happen
            Err(ParseError::AstError(format!("Unexpected operator rule: {:?}", pair.as_rule())))
        },
        _ => {
            // Try to handle inner expressions
            let rule = pair.as_rule(); // Store the rule before moving pair
            if let Some(inner) = pair.into_inner().next() {
                parse_expression(inner)
            } else {
                Err(ParseError::AstError(format!("Unexpected term rule: {:?}", rule)))
            }
        }
    }
}
