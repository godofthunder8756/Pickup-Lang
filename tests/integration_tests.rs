use pickup_lang::{parser, compiler};

#[test]
fn test_arithmetic_operations() {
    let source = r#"
        x = 10
        y = 5
        z = x + y
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    
    // Just verify it compiles without panic
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_string_concatenation() {
    let source = r#"
        greeting = "Hello"
        name = "World"
        message = greeting .. ", " .. name
        print(message)
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_array_operations() {
    let source = r#"
        arr = [1, 2, 3, 4, 5]
        first = arr[0]
        last = arr[4]
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_if_statement() {
    let source = r#"
        x = 10
        if x > 5 then
            print("x is greater than 5")
        end
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_if_else_statement() {
    let source = r#"
        x = 3
        if x > 5 then
            print("x is greater than 5")
        else
            print("x is not greater than 5")
        end
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_while_loop() {
    let source = r#"
        i = 1
        while i <= 3 do
            print(i)
            i = i + 1
        end
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_for_loop() {
    let source = r#"
        for i = 1, 5 do
            print(i)
        end
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_comparison_operators() {
    let source = r#"
        a = 10
        b = 20
        eq = a == b
        neq = a ~= b
        lt = a < b
        gt = a > b
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_logical_operators() {
    let source = r#"
        t = true
        f = false
        andresult = t and f
        orresult = t or f
        notresult = not f
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_operator_precedence() {
    let source = r#"
        x = 15
        in_range = x >= 10 and x <= 20
        age = 25
        is_adult = age >= 18 and age < 65
        result = 2 + 3 * 4
        check = 5 + 5 == 10
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_function_definition_and_call() {
    let source = r#"
        function add(a, b)
            result = a + b
            return result
        end
        
        sum = add(5, 3)
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_function_with_string_return() {
    let source = r#"
        function greet(name)
            message = "Hello, " .. name
            return message
        end
        
        greeting = greet("World")
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_module_import() {
    let source = r#"
        import "json"
        import "fs"
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_nil_value() {
    let source = r#"
        x = nil
    "#;
    
    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}
