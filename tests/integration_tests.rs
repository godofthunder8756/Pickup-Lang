use pickup_lang::{parser, compiler};

// ==================== BASIC LANGUAGE FEATURES ====================

#[test]
fn test_arithmetic_operations() {
    let source = r#"
        x = 10
        y = 5
        z = x + y
        print(z)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_modulo_operator() {
    let source = r#"
        x = 10 % 3
        print(x)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
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
        print(first)
        print(last)
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
        print(x)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== CONTROL FLOW ====================

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
fn test_if_elseif_else_statement() {
    let source = r#"
        score = 75
        if score >= 90 then
            print("A")
        elseif score >= 80 then
            print("B")
        elseif score >= 70 then
            print("C")
        else
            print("D")
        end
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== LOOPS ====================

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
fn test_for_loop_with_step() {
    let source = r#"
        for i = 0, 10, 2 do
            print(i)
        end
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_break_statement() {
    let source = r#"
        i = 1
        while i <= 10 do
            if i == 5 then
                break
            end
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
fn test_continue_statement() {
    let source = r#"
        for i = 1, 5 do
            if i == 3 then
                continue
            end
            print(i)
        end
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== OPERATORS ====================

#[test]
fn test_comparison_operators() {
    let source = r#"
        a = 10
        b = 20
        eq = a == b
        neq = a ~= b
        lt = a < b
        gt = a > b
        le = a <= b
        ge = a >= b
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
        print(andresult)
        print(orresult)
        print(notresult)
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
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== FUNCTIONS ====================

#[test]
fn test_function_definition_and_call() {
    let source = r#"
        function add(a, b)
            result = a + b
            return result
        end

        sum = add(5, 3)
        print(sum)
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
        print(greeting)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_recursive_function() {
    let source = r#"
        function factorial(n)
            if n <= 1 then
                return 1
            else
                return n * factorial(n - 1)
            end
        end

        result = factorial(5)
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_fibonacci() {
    let source = r#"
        function fib(n)
            if n <= 1 then
                return n
            else
                return fib(n - 1) + fib(n - 2)
            end
        end

        result = fib(10)
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== LOCAL VARIABLES ====================

#[test]
fn test_local_variable() {
    let source = r#"
        local x = 10
        local y = 20
        local z = x + y
        print(z)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== DICTIONARIES ====================

#[test]
fn test_dictionary_literal() {
    let source = r#"
        person = {name = "John", age = 30}
        print(person)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== MODULES AND DOT NOTATION ====================

#[test]
fn test_module_import() {
    let source = r#"
        import "math"
        import "string"
        import "json"
        import "fs"
        import "array"
        import "type"
        import "os"
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_math_module_constants() {
    let source = r#"
        import "math"
        pi = math.pi
        e = math.e
        print(pi)
        print(e)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_math_module_functions() {
    let source = r#"
        import "math"
        result = math.floor(3.7)
        print(result)
        result = math.ceil(3.2)
        print(result)
        result = math.sqrt(16)
        print(result)
        result = math.abs(-5)
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_string_module_functions() {
    let source = r#"
        import "string"
        result = string.upper("hello")
        print(result)
        result = string.lower("HELLO")
        print(result)
        result = string.length("hello")
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_string_split() {
    let source = r#"
        import "string"
        parts = string.split("a,b,c", ",")
        print(parts)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_string_contains() {
    let source = r#"
        import "string"
        result = string.contains("hello world", "world")
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_string_replace() {
    let source = r#"
        import "string"
        result = string.replace("hello world", "world", "Pickup")
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_array_module() {
    let source = r#"
        import "array"
        arr = [1, 2, 3]
        length = array.length(arr)
        print(length)
        arr2 = array.push(arr, 4)
        print(arr2)
        arr3 = array.reverse(arr)
        print(arr3)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_array_range() {
    let source = r#"
        import "array"
        nums = array.range(1, 5)
        print(nums)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_array_sort() {
    let source = r#"
        import "array"
        arr = [3, 1, 4, 1, 5, 9, 2, 6]
        sorted = array.sort(arr)
        print(sorted)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_type_module() {
    let source = r#"
        import "type"
        t1 = type.typeof(42)
        t2 = type.typeof("hello")
        t3 = type.typeof(true)
        print(t1)
        print(t2)
        print(t3)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_type_conversions() {
    let source = r#"
        import "type"
        num = type.tonumber("42")
        str = type.tostring(42)
        print(num)
        print(str)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_json_module() {
    let source = r#"
        import "json"
        str = json.stringify([1, 2, 3])
        print(str)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_os_module() {
    let source = r#"
        import "os"
        t = os.time()
        print(t)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== ERROR HANDLING ====================

#[test]
fn test_try_catch() {
    let source = r#"
        try
            print("In try block")
            throw "Custom error"
        catch e
            print("Caught error: " .. e)
        end
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_try_catch_no_throw() {
    let source = r#"
        try
            x = 10
            print(x)
        catch e
            print("This should not print")
        end
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

// ==================== COMPLEX PROGRAMS ====================

#[test]
fn test_prime_checker() {
    let source = r#"
        function isPrime(n)
            if n <= 1 then
                return false
            end
            if n <= 3 then
                return true
            end
            i = 2
            while i * i <= n do
                if n % i == 0 then
                    return false
                end
                i = i + 1
            end
            return true
        end

        print(isPrime(7))
        print(isPrime(10))
        print(isPrime(17))
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_complex_expressions() {
    let source = r#"
        result = (5 + 3) * 2 - 4 / 2
        print(result)
        check = ((10 > 5) and (3 < 7)) or false
        print(check)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_nested_function_calls() {
    let source = r#"
        function double(x)
            return x * 2
        end

        function square(x)
            return x * x
        end

        result = double(square(3))
        print(result)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_nested_loops() {
    let source = r#"
        for i = 1, 3 do
            for j = 1, 3 do
                print(i * j)
            end
        end
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}

#[test]
fn test_array_sum() {
    let source = r#"
        arr = [1, 2, 3, 4, 5]
        sum = 0
        i = 0
        while i < 5 do
            sum = sum + arr[i]
            i = i + 1
        end
        print(sum)
    "#;

    let tokens = parser::tokenize(source, false).expect("Tokenization failed");
    let ast = parser::parse_to_ast(tokens, false).expect("Parsing failed");
    let bytecode = compiler::Compiler::compile(&ast, false);
    compiler::Vm::execute(&bytecode, false);
}
