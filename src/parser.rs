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

pub fn tokenize(source: &str, verbose: bool) -> Result<Pairs<Rule>, ParseError> {
    if verbose {
        println!("Tokenizing source:\n{}", source);
    }
    let pairs = PickupParser::parse(Rule::program, source)?;
    if verbose {
        println!("Parsed pairs: {:?}", pairs);
    }
    Ok(pairs)
}

pub fn parse_to_ast(pairs: Pairs<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    let mut statements = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for inner_pair in pair.into_inner() {
                    if let Some(stmt) = parse_statement(inner_pair, verbose)? {
                        statements.push(stmt);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(AstNode::Program(statements))
}

fn parse_statement(pair: Pair<Rule>, verbose: bool) -> Result<Option<AstNode>, ParseError> {
    if verbose {
        println!("Parsing statement: {:?}", pair);
    }
    match pair.as_rule() {
        Rule::assignment => {
            let inner: Vec<_> = pair.into_inner().collect();
            if verbose {
                println!("Assignment inner: {:?}", inner);
            }

            // First element is always the identifier
            let id_node = AstNode::Identifier(inner[0].as_str().to_string());

            // Check if this is a binary operation with string concatenation (greeting = "Hello, " .. name)
            if inner.len() >= 4 && inner[2].as_rule() == Rule::op_concat {
                let left = parse_term(inner[1].clone(), verbose)?;
                let right = parse_term(inner[3].clone(), verbose)?;

                let binary_op = AstNode::BinaryOp(Box::new(left), "..".to_string(), Box::new(right));
                return Ok(Some(AstNode::Assignment(Box::new(id_node), Box::new(binary_op))));
            }

            // Check if this is a binary operation (x = a + b)
            if inner.len() >= 4 && (inner[2].as_rule() == Rule::op_add ||
                                    inner[2].as_rule() == Rule::op_sub ||
                                    inner[2].as_rule() == Rule::op_mul ||
                                    inner[2].as_rule() == Rule::op_div) {
                let left = parse_term(inner[1].clone(), verbose)?;
                let op = inner[2].as_str().to_string();
                let right = parse_term(inner[3].clone(), verbose)?;

                let binary_op = AstNode::BinaryOp(Box::new(left), op, Box::new(right));
                return Ok(Some(AstNode::Assignment(Box::new(id_node), Box::new(binary_op))));
            }

            // Regular assignment
            let value_node = parse_expression(inner[1].clone(), verbose)?;
            Ok(Some(AstNode::Assignment(Box::new(id_node), Box::new(value_node))))
        },
        Rule::function_call => {
            let function_call = parse_function_call(pair, verbose)?;
            match function_call {
                AstNode::Print(_) => Ok(Some(function_call)),
                _ => Ok(Some(function_call))
            }
        },
        Rule::print_stmt => {
            let mut inner = pair.into_inner();
            let expr = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing expression in print statement".into()))?;
            let expr_node = parse_expression(expr, verbose)?;
            Ok(Some(AstNode::Print(Box::new(expr_node))))
        },
        Rule::if_stmt => {
            let mut inner = pair.into_inner();

            // Parse condition
            let condition = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing condition in if statement".into()))?;
            let condition_node = parse_expression(condition, verbose)?;

            // Parse then block
            let mut then_block = Vec::new();
            let mut else_block = None;
            let mut in_else = false;

            for stmt_pair in inner {
                if verbose {
                    println!("If statement part: {:?}", stmt_pair);
                }

                // Check if we're entering the else block
                if stmt_pair.as_str() == "else" {
                    in_else = true;
                    continue;
                }

                if let Some(stmt) = parse_statement(stmt_pair, verbose)? {
                    if in_else {
                        if else_block.is_none() {
                            else_block = Some(Vec::new());
                        }
                        else_block.as_mut().unwrap().push(stmt);
                    } else {
                        then_block.push(stmt);
                    }
                }
            }

            Ok(Some(AstNode::If(Box::new(condition_node), then_block, else_block)))
        },
        Rule::while_stmt => {
            let mut inner = pair.into_inner();

            // Parse condition
            let condition = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing condition in while statement".into()))?;
            let condition_node = parse_expression(condition, verbose)?;

            // Parse body
            let mut body = Vec::new();
            for stmt_pair in inner {
                if let Some(stmt) = parse_statement(stmt_pair, verbose)? {
                    body.push(stmt);
                }
            }

            Ok(Some(AstNode::While(Box::new(condition_node), body)))
        },
        Rule::for_stmt => {
            let mut inner = pair.into_inner();

            // Parse loop variable
            let var_pair = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing variable in for statement".into()))?;
            let var_name = var_pair.as_str().to_string();

            // Parse start expression
            let start = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing start value in for statement".into()))?;
            let start_node = parse_expression(start, verbose)?;

            // Parse end expression
            let end = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing end value in for statement".into()))?;
            let end_node = parse_expression(end, verbose)?;

            // Parse body
            let mut body = Vec::new();
            for stmt_pair in inner {
                if let Some(stmt) = parse_statement(stmt_pair, verbose)? {
                    body.push(stmt);
                }
            }

            Ok(Some(AstNode::For(var_name, Box::new(start_node), Box::new(end_node), body)))
        },
        Rule::func_def => {
            let mut inner = pair.into_inner();

            // Parse function name
            let name_pair = inner.next()
                .ok_or_else(|| ParseError::AstError("Missing function name".into()))?;
            let func_name = name_pair.as_str().to_string();

            // Parse parameters
            let mut params = Vec::new();
            let mut body = Vec::new();
            let mut parsing_params = true;

            for p in inner {
                match p.as_rule() {
                    Rule::identifier if parsing_params => {
                        params.push(p.as_str().to_string());
                    },
                    _ => {
                        parsing_params = false;
                        if let Some(stmt) = parse_statement(p, verbose)? {
                            body.push(stmt);
                        }
                    }
                }
            }

            Ok(Some(AstNode::Function(func_name, params, body)))
        },
        Rule::return_stmt => {
            let mut inner = pair.into_inner();

            if let Some(expr_pair) = inner.next() {
                let expr_node = parse_expression(expr_pair, verbose)?;
                Ok(Some(AstNode::Return(Some(Box::new(expr_node)))))
            } else {
                Ok(Some(AstNode::Return(None)))
            }
        },
        Rule::expression => {
            let expr = parse_expression(pair, verbose)?;
            Ok(Some(expr))
        },
        _ => Ok(None),
    }
}

fn parse_function_call(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    if verbose {
        println!("Parsing function call: {:?}", pair);
    }
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
            let left = parse_term(inner[1].clone(), verbose)?;
            let right = parse_term(inner[3].clone(), verbose)?;
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
        let arg = parse_expression(inner[1].clone(), verbose)?;
        
        if func_name.as_str() == "print" {
            Ok(AstNode::Print(Box::new(arg)))
        } else {
            let mut args = Vec::new();
            args.push(arg);
            
            // Add any remaining arguments
            for i in 2..inner.len() {
                args.push(parse_expression(inner[i].clone(), verbose)?);
            }
            
            Ok(AstNode::FunctionCall(func_name.as_str().to_string(), args))
        }
    } else {
        // Function call with no arguments
        Ok(AstNode::FunctionCall(func_name.as_str().to_string(), vec![]))
    }
}

fn parse_expression(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    if verbose {
        println!("Parsing expression: {:?}", pair);
    }
    
    match pair.as_rule() {
        Rule::expression => {
            let pairs = pair.into_inner().collect::<Vec<_>>();
            if verbose {
                println!("Expression inner pairs: {:?}", pairs);
            }
            
            if pairs.is_empty() {
                return Err(ParseError::AstError("Empty expression".into()));
            }
            
            // Handle binary expressions (a + b, etc.)
            if pairs.len() >= 3 {
                let mut left = parse_term(pairs[0].clone(), verbose)?;
                
                let mut i = 1;
                while i + 1 < pairs.len() {
                    let op = pairs[i].as_str().to_string();
                    let right = parse_term(pairs[i+1].clone(), verbose)?;
                    left = AstNode::BinaryOp(Box::new(left), op, Box::new(right));
                    i += 2;
                }
                
                return Ok(left);
            }
            
            // Single term
            parse_term(pairs[0].clone(), verbose)
        },
        Rule::term => parse_term(pair, verbose),
        _ => parse_term(pair, verbose) // Try parsing as a term
    }
}

fn parse_term(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
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
        Rule::nil_literal => {
            Ok(AstNode::Nil)
        },
        Rule::identifier => Ok(AstNode::Identifier(pair.as_str().to_string())),
        Rule::function_call => parse_function_call(pair, verbose),
        Rule::table_literal => {
            let mut entries = Vec::new();
            let mut index = 0;

            for entry_pair in pair.into_inner() {
                if entry_pair.as_rule() == Rule::table_entry {
                    let mut inner = entry_pair.into_inner();

                    // Check if it's a key-value pair or just a value
                    let first = inner.next().unwrap();
                    if let Some(second) = inner.next() {
                        // Key-value pair: [key] = value
                        let key = parse_expression(first, verbose)?;
                        let value = parse_expression(second, verbose)?;
                        entries.push((key, value));
                    } else {
                        // Just a value, use auto-incrementing index
                        let key = AstNode::Number(index as f64);
                        let value = parse_expression(first, verbose)?;
                        entries.push((key, value));
                        index += 1;
                    }
                }
            }

            Ok(AstNode::Table(entries))
        },
        Rule::table_access => {
            let mut inner = pair.into_inner();
            let table = inner.next().unwrap();
            let key = inner.next().unwrap();

            let table_node = AstNode::Identifier(table.as_str().to_string());
            let key_node = parse_expression(key, verbose)?;

            Ok(AstNode::TableAccess(Box::new(table_node), Box::new(key_node)))
        },
        Rule::op_add | Rule::op_sub | Rule::op_mul | Rule::op_div | Rule::op_concat => {
            // Binary operators are handled in parse_expression, this should not happen
            Err(ParseError::AstError(format!("Unexpected operator rule: {:?}", pair.as_rule())))
        },
        _ => {
            // Try to handle inner expressions
            let rule = pair.as_rule(); // Store the rule before moving pair
            if let Some(inner) = pair.into_inner().next() {
                parse_expression(inner, verbose)
            } else {
                Err(ParseError::AstError(format!("Unexpected term rule: {:?}", rule)))
            }
        }
    }
}
