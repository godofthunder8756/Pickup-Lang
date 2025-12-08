use crate::ast::AstNode;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
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
        Rule::import_stmt => {
            let mut inner = pair.into_inner();
            let module_path = inner.next().ok_or_else(|| {
                ParseError::AstError("Missing module path in import statement".into())
            })?;
            // Remove quotes from string
            let path_str = module_path.as_str();
            let module = &path_str[1..path_str.len() - 1];
            Ok(Some(AstNode::Import(module.to_string())))
        }
        Rule::assignment => {
            let inner: Vec<_> = pair.into_inner().collect();
            if verbose {
                println!("Assignment inner: {:?}", inner);
            }

            // First element is the target (identifier, member_access, or index_access)
            let target_node = parse_term(inner[0].clone(), verbose)?;

            // Parse the right-hand side as a full expression
            let value_node = parse_expression(inner[1].clone(), verbose)?;
            Ok(Some(AstNode::Assignment(
                Box::new(target_node),
                Box::new(value_node),
            )))
        }
        Rule::local_assignment => {
            let inner: Vec<_> = pair.into_inner().collect();
            if verbose {
                println!("Local assignment inner: {:?}", inner);
            }

            let var_name = inner[0].as_str().to_string();
            let value_node = parse_expression(inner[1].clone(), verbose)?;
            Ok(Some(AstNode::LocalAssignment(var_name, Box::new(value_node))))
        }
        Rule::function_call => {
            let function_call = parse_function_call(pair, verbose)?;
            match function_call {
                AstNode::Print(_) => Ok(Some(function_call)),
                _ => Ok(Some(function_call)),
            }
        }
        Rule::method_call => {
            let method_call = parse_method_call(pair, verbose)?;
            Ok(Some(method_call))
        }
        Rule::print_stmt => {
            let mut inner = pair.into_inner();
            let expr = inner.next().ok_or_else(|| {
                ParseError::AstError("Missing expression in print statement".into())
            })?;
            let expr_node = parse_expression(expr, verbose)?;
            Ok(Some(AstNode::Print(Box::new(expr_node))))
        }
        Rule::if_stmt => {
            let inner: Vec<_> = pair.into_inner().collect();
            if verbose {
                println!("If statement inner: {:?}", inner);
            }

            // Parse condition
            let condition = parse_expression(inner[0].clone(), verbose)?;

            // Parse then block
            let mut then_stmts = Vec::new();
            let mut idx = 1;

            // Collect statements until we hit elseif_clause, else_clause, or end
            while idx < inner.len() {
                match inner[idx].as_rule() {
                    Rule::elseif_clause | Rule::else_clause => break,
                    _ => {
                        if let Some(stmt) = parse_statement(inner[idx].clone(), verbose)? {
                            then_stmts.push(stmt);
                        }
                        idx += 1;
                    }
                }
            }

            // Parse elseif clauses
            let mut elseif_clauses = Vec::new();
            while idx < inner.len() && inner[idx].as_rule() == Rule::elseif_clause {
                let elseif_inner: Vec<_> = inner[idx].clone().into_inner().collect();
                let elseif_cond = parse_expression(elseif_inner[0].clone(), verbose)?;
                let mut elseif_stmts = Vec::new();
                for i in 1..elseif_inner.len() {
                    if let Some(stmt) = parse_statement(elseif_inner[i].clone(), verbose)? {
                        elseif_stmts.push(stmt);
                    }
                }
                elseif_clauses.push((elseif_cond, elseif_stmts));
                idx += 1;
            }

            // Parse else clause
            let else_block = if idx < inner.len() && inner[idx].as_rule() == Rule::else_clause {
                let else_inner: Vec<_> = inner[idx].clone().into_inner().collect();
                let mut else_stmts = Vec::new();
                for else_pair in else_inner {
                    if let Some(stmt) = parse_statement(else_pair, verbose)? {
                        else_stmts.push(stmt);
                    }
                }
                Some(else_stmts)
            } else {
                None
            };

            Ok(Some(AstNode::If(Box::new(condition), then_stmts, elseif_clauses, else_block)))
        }
        Rule::while_stmt => {
            let inner: Vec<_> = pair.into_inner().collect();
            let condition = parse_expression(inner[0].clone(), verbose)?;
            let mut body = Vec::new();
            for i in 1..inner.len() {
                if let Some(stmt) = parse_statement(inner[i].clone(), verbose)? {
                    body.push(stmt);
                }
            }
            Ok(Some(AstNode::While(Box::new(condition), body)))
        }
        Rule::for_stmt => {
            let inner: Vec<_> = pair.into_inner().collect();
            let var_name = inner[0].as_str().to_string();
            let start = parse_expression(inner[1].clone(), verbose)?;
            let end = parse_expression(inner[2].clone(), verbose)?;

            // Check if there's a step value
            let mut body_start = 3;
            let step = if inner.len() > 3 {
                // Check if inner[3] is an expression (step) or a statement (body)
                match inner[3].as_rule() {
                    Rule::expression => {
                        body_start = 4;
                        Some(Box::new(parse_expression(inner[3].clone(), verbose)?))
                    }
                    _ => None
                }
            } else {
                None
            };

            let mut body = Vec::new();
            for i in body_start..inner.len() {
                if let Some(stmt) = parse_statement(inner[i].clone(), verbose)? {
                    body.push(stmt);
                }
            }
            Ok(Some(AstNode::For(var_name, Box::new(start), Box::new(end), step, body)))
        }
        Rule::function_def => {
            let inner: Vec<_> = pair.into_inner().collect();
            let func_name = inner[0].as_str().to_string();

            // Parse parameters
            let mut params = Vec::new();
            let mut body_start = 1;

            // Collect parameters until we hit a statement
            while body_start < inner.len() {
                match inner[body_start].as_rule() {
                    Rule::identifier => {
                        params.push(inner[body_start].as_str().to_string());
                        body_start += 1;
                    }
                    _ => break
                }
            }

            // Parse body
            let mut body = Vec::new();
            for i in body_start..inner.len() {
                if let Some(stmt) = parse_statement(inner[i].clone(), verbose)? {
                    body.push(stmt);
                }
            }

            Ok(Some(AstNode::FunctionDef(func_name, params, body)))
        }
        Rule::return_stmt => {
            let mut inner = pair.into_inner();
            let value = if let Some(expr) = inner.next() {
                Some(Box::new(parse_expression(expr, verbose)?))
            } else {
                None
            };
            Ok(Some(AstNode::Return(value)))
        }
        Rule::break_stmt => {
            Ok(Some(AstNode::Break))
        }
        Rule::continue_stmt => {
            Ok(Some(AstNode::Continue))
        }
        Rule::try_stmt => {
            let inner: Vec<_> = pair.into_inner().collect();
            if verbose {
                println!("Try statement inner: {:?}", inner);
            }

            let mut try_body = Vec::new();
            let mut idx = 0;

            // Parse try block statements until catch clause
            while idx < inner.len() && inner[idx].as_rule() != Rule::catch_clause {
                if let Some(stmt) = parse_statement(inner[idx].clone(), verbose)? {
                    try_body.push(stmt);
                }
                idx += 1;
            }

            // Parse catch clause
            let (catch_var, catch_body) = if idx < inner.len() && inner[idx].as_rule() == Rule::catch_clause {
                let catch_inner: Vec<_> = inner[idx].clone().into_inner().collect();
                let mut catch_var = None;
                let mut catch_stmts = Vec::new();
                let mut catch_idx = 0;

                // Check for optional catch variable
                if !catch_inner.is_empty() && catch_inner[0].as_rule() == Rule::identifier {
                    catch_var = Some(catch_inner[0].as_str().to_string());
                    catch_idx = 1;
                }

                // Parse catch block statements
                for i in catch_idx..catch_inner.len() {
                    if let Some(stmt) = parse_statement(catch_inner[i].clone(), verbose)? {
                        catch_stmts.push(stmt);
                    }
                }

                (catch_var, catch_stmts)
            } else {
                (None, Vec::new())
            };

            Ok(Some(AstNode::TryCatch(try_body, catch_var, catch_body)))
        }
        Rule::throw_stmt => {
            let mut inner = pair.into_inner();
            let expr = inner.next().ok_or_else(|| {
                ParseError::AstError("Missing expression in throw statement".into())
            })?;
            let expr_node = parse_expression(expr, verbose)?;
            Ok(Some(AstNode::Throw(Box::new(expr_node))))
        }
        Rule::expression => {
            let expr = parse_expression(pair, verbose)?;
            Ok(Some(expr))
        }
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
            let concat = AstNode::BinaryOp(Box::new(left), "..".to_string(), Box::new(right));
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
        Ok(AstNode::FunctionCall(
            func_name.as_str().to_string(),
            vec![],
        ))
    }
}

fn parse_method_call(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    if verbose {
        println!("Parsing method call: {:?}", pair);
    }
    let inner: Vec<_> = pair.into_inner().collect();

    if inner.is_empty() {
        return Err(ParseError::AstError("Empty method call".into()));
    }

    // First element should be member_access
    let member_access_pair = inner[0].clone();
    let member_inner: Vec<_> = member_access_pair.into_inner().collect();

    // Get base object
    let base = parse_term(member_inner[0].clone(), verbose)?;

    // Get member chain
    let mut members = Vec::new();
    for i in 1..member_inner.len() {
        if member_inner[i].as_rule() == Rule::identifier {
            members.push(member_inner[i].as_str().to_string());
        }
    }

    // Parse arguments
    let mut args = Vec::new();
    for i in 1..inner.len() {
        args.push(parse_expression(inner[i].clone(), verbose)?);
    }

    Ok(AstNode::MethodCall(Box::new(base), members, args))
}

fn parse_member_access(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    if verbose {
        println!("Parsing member access: {:?}", pair);
    }
    let inner: Vec<_> = pair.into_inner().collect();

    if inner.is_empty() {
        return Err(ParseError::AstError("Empty member access".into()));
    }

    // Get base object
    let base = parse_term(inner[0].clone(), verbose)?;

    // Get member chain
    let mut members = Vec::new();
    for i in 1..inner.len() {
        if inner[i].as_rule() == Rule::identifier {
            members.push(inner[i].as_str().to_string());
        }
    }

    Ok(AstNode::MemberAccess(Box::new(base), members))
}

fn parse_lambda(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    if verbose {
        println!("Parsing lambda: {:?}", pair);
    }
    let inner: Vec<_> = pair.into_inner().collect();

    // Parse parameters
    let mut params = Vec::new();
    let mut body_start = 0;

    // Collect parameters until we hit a statement
    while body_start < inner.len() {
        match inner[body_start].as_rule() {
            Rule::identifier => {
                params.push(inner[body_start].as_str().to_string());
                body_start += 1;
            }
            _ => break
        }
    }

    // Parse body
    let mut body = Vec::new();
    for i in body_start..inner.len() {
        if let Some(stmt) = parse_statement(inner[i].clone(), verbose)? {
            body.push(stmt);
        }
    }

    Ok(AstNode::Lambda(params, body))
}

fn parse_dictionary(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    if verbose {
        println!("Parsing dictionary: {:?}", pair);
    }
    let inner: Vec<_> = pair.into_inner().collect();

    let mut entries = Vec::new();
    for entry_pair in inner {
        if entry_pair.as_rule() == Rule::dict_entry {
            let entry_inner: Vec<_> = entry_pair.into_inner().collect();
            if entry_inner.len() >= 2 {
                // Key can be identifier or string
                let key = if entry_inner[0].as_rule() == Rule::string {
                    let text = entry_inner[0].as_str();
                    text[1..text.len() - 1].to_string() // Remove quotes
                } else {
                    entry_inner[0].as_str().to_string()
                };
                let value = parse_expression(entry_inner[1].clone(), verbose)?;
                entries.push((key, value));
            }
        }
    }

    Ok(AstNode::Dictionary(entries))
}

// Operator precedence levels (higher number = higher precedence)
fn get_precedence(op: &str) -> u8 {
    match op {
        "or" => 1,
        "and" => 2,
        "==" | "~=" => 3,
        "<" | ">" | "<=" | ">=" => 4,
        ".." => 5,
        "+" | "-" => 6,
        "*" | "/" | "%" => 7,
        _ => 0,
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

            // Handle binary expressions with precedence
            if pairs.len() >= 3 {
                return parse_expression_with_precedence(&pairs, 0, verbose);
            }

            // Single term
            parse_term(pairs[0].clone(), verbose)
        }
        Rule::term => parse_term(pair, verbose),
        _ => parse_term(pair, verbose), // Try parsing as a term
    }
}

// Parse expression with operator precedence using precedence climbing
fn parse_expression_with_precedence(
    pairs: &[Pair<Rule>],
    min_precedence: u8,
    verbose: bool,
) -> Result<AstNode, ParseError> {
    if pairs.is_empty() {
        return Err(ParseError::AstError("Empty expression in precedence parsing".into()));
    }

    // Parse the first term
    let mut left = parse_term(pairs[0].clone(), verbose)?;
    let mut i = 1;

    // Process operators and terms
    while i < pairs.len() {
        if i + 1 >= pairs.len() {
            break;
        }

        let op = pairs[i].as_str();
        let precedence = get_precedence(op);

        if precedence < min_precedence {
            break;
        }

        let op_str = op.to_string();
        i += 1; // Move past operator

        // Parse the right side with higher precedence
        let mut right = parse_term(pairs[i].clone(), verbose)?;
        i += 1; // Move past right term

        // Look ahead for higher precedence operators
        while i < pairs.len() {
            if i + 1 >= pairs.len() {
                break;
            }

            let next_op = pairs[i].as_str();
            let next_precedence = get_precedence(next_op);

            if next_precedence <= precedence {
                break;
            }

            // Build the right subtree with remaining tokens
            let remaining = &pairs[i - 1..];
            right = parse_expression_with_precedence(remaining, precedence + 1, verbose)?;

            // Calculate how many tokens were consumed
            // This is simplified - we consumed everything from i-1 onwards into right
            i = pairs.len();
            break;
        }

        left = AstNode::BinaryOp(Box::new(left), op_str, Box::new(right));
    }

    Ok(left)
}

fn parse_term(pair: Pair<Rule>, verbose: bool) -> Result<AstNode, ParseError> {
    match pair.as_rule() {
        Rule::number => {
            let value = pair.as_str().parse::<f64>().map_err(|_| {
                ParseError::AstError(format!("Failed to parse number: {}", pair.as_str()))
            })?;
            Ok(AstNode::Number(value))
        }
        Rule::string => {
            // Remove the quotes from the string literal
            let text = pair.as_str();
            let value = &text[1..text.len() - 1];
            Ok(AstNode::String(value.to_string()))
        }
        Rule::boolean => {
            let value = pair.as_str() == "true";
            Ok(AstNode::Boolean(value))
        }
        Rule::nil => Ok(AstNode::Nil),
        Rule::not_expr => {
            let inner = pair.into_inner().next().ok_or_else(|| {
                ParseError::AstError("Missing operand for not expression".into())
            })?;
            let operand = parse_term(inner, verbose)?;
            Ok(AstNode::Not(Box::new(operand)))
        }
        Rule::identifier => Ok(AstNode::Identifier(pair.as_str().to_string())),
        Rule::function_call => parse_function_call(pair, verbose),
        Rule::method_call => parse_method_call(pair, verbose),
        Rule::member_access => parse_member_access(pair, verbose),
        Rule::lambda => parse_lambda(pair, verbose),
        Rule::dictionary => parse_dictionary(pair, verbose),
        Rule::table => {
            let mut elements = Vec::new();
            for inner_pair in pair.into_inner() {
                elements.push(parse_expression(inner_pair, verbose)?);
            }
            Ok(AstNode::Table(elements))
        }
        Rule::index_access => {
            let inner: Vec<_> = pair.into_inner().collect();
            if inner.len() < 2 {
                return Err(ParseError::AstError("Invalid index access".into()));
            }
            let base = parse_term(inner[0].clone(), verbose)?;
            let index = parse_expression(inner[1].clone(), verbose)?;
            Ok(AstNode::Index(Box::new(base), Box::new(index)))
        }
        Rule::op_add | Rule::op_sub | Rule::op_mul | Rule::op_div | Rule::op_mod | Rule::op_concat => {
            // Binary operators are handled in parse_expression, this should not happen
            Err(ParseError::AstError(format!(
                "Unexpected operator rule: {:?}",
                pair.as_rule()
            )))
        }
        _ => {
            // Try to handle inner expressions
            let rule = pair.as_rule(); // Store the rule before moving pair
            if let Some(inner) = pair.into_inner().next() {
                parse_expression(inner, verbose)
            } else {
                Err(ParseError::AstError(format!(
                    "Unexpected term rule: {:?}",
                    rule
                )))
            }
        }
    }
}
