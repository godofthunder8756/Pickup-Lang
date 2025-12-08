use crate::{compiler, parser};
use rustyline::DefaultEditor;
use std::collections::HashMap;

pub fn run_repl(verbose: bool) {
    let mut rl = DefaultEditor::new().unwrap();
    let mut vars: HashMap<String, compiler::Value> = HashMap::new();

    println!("Pickup REPL v0.2.0");
    println!("Type 'exit' or Ctrl+C to quit");
    println!("Use '\\' at end of line for multi-line input");
    println!("");

    let mut buffer = String::new();
    let mut in_multiline = false;

    loop {
        let prompt = if in_multiline { "... " } else { ">>> " };
        let line = match rl.readline(prompt) {
            Ok(line) => line,
            Err(_) => break,
        };

        // Check for exit command
        if !in_multiline && line.trim() == "exit" {
            break;
        }

        // Check for help command
        if !in_multiline && line.trim() == "help" {
            print_help();
            continue;
        }

        // Check for clear command
        if !in_multiline && line.trim() == "clear" {
            vars.clear();
            println!("Variables cleared.");
            continue;
        }

        // Check for vars command
        if !in_multiline && line.trim() == "vars" {
            if vars.is_empty() {
                println!("No variables defined.");
            } else {
                println!("Variables:");
                for (name, value) in &vars {
                    println!("  {} = {}", name, value);
                }
            }
            continue;
        }

        let _ = rl.add_history_entry(&line);

        // Handle multi-line input
        if line.ends_with('\\') {
            buffer.push_str(&line[..line.len() - 1]);
            buffer.push('\n');
            in_multiline = true;
            continue;
        }

        // Check for incomplete constructs
        buffer.push_str(&line);

        // Count open constructs
        let open_count = count_open_constructs(&buffer);

        if open_count > 0 {
            buffer.push('\n');
            in_multiline = true;
            continue;
        }

        // Execute the complete input
        in_multiline = false;
        let input = std::mem::take(&mut buffer);

        if input.trim().is_empty() {
            continue;
        }

        match execute_input(&input, verbose) {
            Ok(result) => {
                if let Some(val) = result {
                    if !matches!(val, compiler::Value::Nil) {
                        println!("=> {}", val);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    println!("Goodbye!");
}

fn count_open_constructs(input: &str) -> i32 {
    let mut count = 0;
    let words: Vec<&str> = input.split_whitespace().collect();

    for word in words {
        match word {
            "if" | "while" | "for" | "function" | "try" => count += 1,
            "end" => count -= 1,
            _ => {}
        }
    }

    // Also count brackets
    for ch in input.chars() {
        match ch {
            '[' | '{' | '(' => count += 1,
            ']' | '}' | ')' => count -= 1,
            _ => {}
        }
    }

    count.max(0)
}

fn execute_input(input: &str, verbose: bool) -> Result<Option<compiler::Value>, String> {
    let tokens = parser::tokenize(input, verbose)
        .map_err(|e| format!("{}", e))?;
    let ast = parser::parse_to_ast(tokens, verbose)
        .map_err(|e| format!("{}", e))?;
    let code = compiler::Compiler::compile(&ast, verbose);
    compiler::Vm::execute(&code, verbose);
    Ok(None)
}

fn print_help() {
    println!("Pickup REPL Commands:");
    println!("  help   - Show this help message");
    println!("  exit   - Exit the REPL");
    println!("  clear  - Clear all variables");
    println!("  vars   - Show all defined variables");
    println!("");
    println!("Language Features:");
    println!("  Variables:    x = 10");
    println!("  Local vars:   local x = 10");
    println!("  Strings:      \"hello\"");
    println!("  Arrays:       [1, 2, 3]");
    println!("  Dictionaries: {{name = \"John\", age = 30}}");
    println!("  Functions:    function add(a, b) return a + b end");
    println!("  Lambdas:      f = function(x) return x * 2 end");
    println!("  If:           if x > 0 then print(\"positive\") end");
    println!("  While:        while i < 10 do i = i + 1 end");
    println!("  For:          for i = 1, 10 do print(i) end");
    println!("  Try/Catch:    try throw \"error\" catch e print(e) end");
    println!("");
    println!("Standard Library Modules:");
    println!("  import \"math\"   - Math functions (floor, ceil, sqrt, sin, cos, etc.)");
    println!("  import \"string\" - String functions (upper, lower, split, replace, etc.)");
    println!("  import \"array\"  - Array functions (push, pop, sort, reverse, etc.)");
    println!("  import \"fs\"     - File system (read, write, exists, mkdir, etc.)");
    println!("  import \"json\"   - JSON parsing (parse, stringify)");
    println!("  import \"type\"   - Type utilities (typeof, tonumber, tostring, etc.)");
    println!("  import \"os\"     - OS utilities (time, getenv, execute, sleep)");
    println!("");
    println!("Multi-line input:");
    println!("  End a line with '\\' to continue on the next line");
    println!("  Or simply type if/while/for/function and press Enter");
}
