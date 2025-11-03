# Pickup Programming Language

Pickup is a lightweight, expressive scripting language with clean syntax and powerful features. Implemented in Rust, it compiles to bytecode and runs on a stack-based virtual machine for efficient execution.

## Features

- **Clean Syntax**: Easy to read and write, inspired by Lua
- **Dynamic Typing**: Flexible type system with type introspection
- **Control Flow**: If/else conditionals, while loops, and for loops
- **Functions**: User-defined functions with parameters and return values
- **Standard Library**: Built-in functions for common operations
- **REPL**: Interactive development environment
- **Bytecode Compilation**: Fast execution through bytecode compilation
- **Comprehensive Operators**: Arithmetic, comparison, logical, and string operations

## Quick Start

### Installation

```bash
git clone https://github.com/godofthunder8756/Pickup-Lang.git
cd Pickup-Lang
cargo build --release
```

### Running Pickup

Start the REPL:
```bash
cargo run
```

Execute a script:
```bash
cargo run examples/comprehensive.up
```

Run with verbose output (debug mode):
```bash
cargo run -- examples/basic.up --noise
```

## Language Guide

### Data Types

Pickup supports the following data types:

- **Numbers**: Floating-point numbers (f64)
  ```pickup
  x = 42
  pi = 3.14159
  negative = -10
  ```

- **Strings**: Text enclosed in double quotes
  ```pickup
  name = "Pickup"
  message = "Hello, World!"
  ```

- **Booleans**: `true` and `false`
  ```pickup
  flag = true
  isValid = false
  ```

- **Nil**: Represents absence of value
  ```pickup
  nothing = nil
  ```

### Variables

Variables are dynamically typed and don't require declaration:

```pickup
x = 10           -- number
name = "Alice"   -- string
isActive = true  -- boolean
```

### Comments

Single-line comments start with `--`:

```pickup
-- This is a comment
x = 5  -- This is also a comment
```

### Operators

#### Arithmetic Operators

```pickup
x = 10 + 5   -- Addition: 15
x = 10 - 5   -- Subtraction: 5
x = 10 * 5   -- Multiplication: 50
x = 10 / 5   -- Division: 2
```

#### Comparison Operators

```pickup
x == y   -- Equal to
x ~= y   -- Not equal to
x < y    -- Less than
x > y    -- Greater than
x <= y   -- Less than or equal to
x >= y   -- Greater than or equal to
```

#### Logical Operators

```pickup
true and false   -- Logical AND
true or false    -- Logical OR
```

#### String Concatenation

```pickup
greeting = "Hello" .. " " .. "World"  -- "Hello World"
```

### Control Flow

#### If Statements

```pickup
if x > 10 then
    print("x is greater than 10")
end

if age >= 18 then
    print("Adult")
else
    print("Minor")
end

-- Nested conditionals
if score >= 90 then
    print("A")
else
    if score >= 80 then
        print("B")
    else
        print("C")
    end
end
```

#### While Loops

```pickup
counter = 5
while counter > 0 do
    print(counter)
    counter = counter - 1
end
```

#### For Loops

```pickup
-- Count from 1 to 10
for i = 1, 10 do
    print(i)
end

-- Calculate sum
sum = 0
for i = 1, 100 do
    sum = sum + i
end
print(sum)
```

### Functions

#### User-Defined Functions

```pickup
function greet(name)
    print("Hello, " .. name)
end

function add(a, b)
    return a + b
end

function fibonacci(n)
    if n <= 1 then
        return n
    else
        return fibonacci(n - 1) + fibonacci(n - 2)
    end
end
```

### Standard Library

Pickup includes several built-in functions:

#### `print(value)`
Outputs a value to standard output.

```pickup
print("Hello, World!")
print(42)
```

#### `type(value)`
Returns the type of a value as a string.

```pickup
print(type(42))        -- "number"
print(type("hello"))   -- "string"
print(type(true))      -- "boolean"
print(type(nil))       -- "nil"
```

#### `len(value)`
Returns the length of a string or table.

```pickup
message = "Hello"
print(len(message))    -- 5
```

#### `tostring(value)`
Converts a value to a string.

```pickup
num = 42
str = tostring(num)
print(str)             -- "42"
```

#### `tonumber(value)`
Converts a value to a number.

```pickup
str = "123"
num = tonumber(str)
print(num + 1)         -- 124
```

## Examples

### Hello World

```pickup
print("Hello, World!")
```

### Variables and Arithmetic

```pickup
x = 10
y = 5
sum = x + y
print("The sum is: " .. sum)
```

### Conditionals

```pickup
age = 18

if age >= 18 then
    print("You can vote")
else
    print("You cannot vote yet")
end
```

### Loops

```pickup
-- For loop
for i = 1, 5 do
    print(i)
end

-- While loop
counter = 10
while counter > 0 do
    print(counter)
    counter = counter - 1
end
```

### Functions

```pickup
function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)
    end
end

print(factorial(5))  -- 120
```

## Example Files

The `examples/` directory contains several example programs:

- `basic.up` - Basic variable assignment and printing
- `conditionals.up` - If/else statements and comparisons
- `loops.up` - For and while loops
- `stdlib.up` - Standard library function demonstrations
- `comprehensive.up` - Complete language feature showcase

## Language Implementation

### Architecture

Pickup uses a three-stage compilation pipeline:

1. **Tokenizer**: Converts source code into tokens using PEG grammar (Pest)
2. **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
3. **Compiler**: Generates bytecode from the AST
4. **Virtual Machine**: Executes bytecode on a stack-based VM

### Bytecode Instructions

The VM supports the following instruction types:

- **Stack Operations**: Push/Pop values
- **Variables**: Load/Store variables
- **Arithmetic**: Add, Sub, Mul, Div
- **Comparison**: Equal, NotEqual, LessThan, GreaterThan, etc.
- **Logical**: And, Or, Not
- **Control Flow**: Jump, JumpIfFalse
- **Functions**: Call, Return
- **I/O**: Print

### File Structure

```
Pickup-Lang/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli.rs           # Command-line interface
│   ├── parser.rs        # Tokenizer and parser
│   ├── ast.rs           # AST node definitions
│   ├── compiler.rs      # Bytecode compiler and VM
│   ├── repl.rs          # Interactive REPL
│   └── interpreter.rs   # (Placeholder)
├── examples/            # Example programs
├── grammar.pest         # PEG grammar definition
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## Development

### Building from Source

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Output

```bash
cargo run -- script.up --noise
```

This will show:
- Tokenization output
- AST structure
- Generated bytecode
- VM execution trace

## Grammar

Pickup uses a PEG (Parsing Expression Grammar) defined in `grammar.pest`:

```
Literals:
  - Numbers: 42, 3.14, -10
  - Strings: "hello"
  - Booleans: true, false
  - Nil: nil

Keywords:
  if, then, else, end, while, do, for, function, return

Operators:
  +, -, *, /           (arithmetic)
  ==, ~=, <, >, <=, >= (comparison)
  and, or              (logical)
  ..                   (string concatenation)
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

### Future Enhancements

- [ ] Full function implementation with call stack
- [ ] Table/array data structures
- [ ] Table indexing and manipulation
- [ ] Break and continue statements
- [ ] Module system
- [ ] File I/O library
- [ ] JSON library
- [ ] More standard library functions
- [ ] Error handling and stack traces
- [ ] Optional static typing
- [ ] Closures and first-class functions

## License

MIT License - See LICENSE file for details

## Credits

Created by godofthunder8756

Inspired by Lua and designed for simplicity and extensibility.
