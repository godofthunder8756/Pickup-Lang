# Pickup

Pickup is a lightweight, production-ready scripting language inspired by Lua. It features a clean syntax, comprehensive standard library, and modern language features.

## Key Features

- **0-based indexing** - Arrays use 0-based indexing like most modern languages
- **Dot notation** - Access module functions and object properties with `math.floor(x)`
- **Complete standard library** - math, string, array, json, fs, os, and type modules
- **Error handling** - try-catch blocks with throw statements
- **Control flow** - if/elseif/else, while, for loops with break/continue
- **Functions** - User-defined functions with recursion support
- **Dictionaries** - Key-value objects with `{name = "John", age = 30}` syntax
- **Local variables** - Scoped variables with `local x = 10`
- **Multi-line REPL** - Interactive development with auto-detection of incomplete constructs

## Installation

### Download Pre-built Binaries

Download binaries for your platform from the [Releases page](https://github.com/godofthunder8756/Pickup-Lang/releases).

**Available Downloads:**
- **Nightly builds**: Automatically built from the latest main branch (pre-release)
- **Versioned releases**: Stable releases with semantic versioning (e.g., v0.2.0)

**Linux/macOS:**
```bash
chmod +x pick-*
sudo mv pick-* /usr/local/bin/pick
```

**Windows:**
Download the `.exe` file and add it to your PATH.

### Build from Source

```bash
git clone https://github.com/godofthunder8756/Pickup-Lang.git
cd Pickup-Lang
cargo build --release
# Binary at target/release/pick
```

## Quick Start

```bash
# Start the REPL
pick

# Execute a file
pick script.up
```

## Language Guide

### Variables and Types

```pickup
-- Variables
x = 42
name = "Alice"
active = true
nothing = nil

-- Local variables (scoped)
local counter = 0

-- Arrays (0-based indexing)
numbers = [1, 2, 3, 4, 5]
print(numbers[0])  -- 1

-- Dictionaries
person = {name = "John", age = 30}
print(person)
```

### Operators

```pickup
-- Arithmetic: + - * / %
result = 10 + 5 * 2 - 3 / 1  -- 17
remainder = 10 % 3  -- 1

-- String concatenation
greeting = "Hello" .. ", " .. "World!"

-- Comparison: == ~= < > <= >=
print(10 == 10)  -- true
print(10 ~= 5)   -- true

-- Logical: and or not
print(true and false)  -- false
print(true or false)   -- true
print(not false)       -- true
```

### Control Flow

```pickup
-- If-elseif-else
score = 85
if score >= 90 then
    print("A")
elseif score >= 80 then
    print("B")
elseif score >= 70 then
    print("C")
else
    print("F")
end

-- While loop
i = 1
while i <= 5 do
    print(i)
    i = i + 1
end

-- For loop
for j = 1, 10 do
    print(j)
end

-- For loop with step
for k = 0, 20, 2 do
    print(k)  -- 0, 2, 4, ...
end

-- Break and continue
for i = 1, 10 do
    if i == 5 then
        break
    end
    if i % 2 == 0 then
        continue
    end
    print(i)
end
```

### Functions

```pickup
-- Basic function
function add(a, b)
    return a + b
end
print(add(5, 3))  -- 8

-- Recursive function
function factorial(n)
    if n <= 1 then
        return 1
    end
    return n * factorial(n - 1)
end
print(factorial(5))  -- 120

-- Function with conditionals
function max(a, b)
    if a > b then
        return a
    else
        return b
    end
end
```

### Error Handling

```pickup
try
    -- Code that might fail
    throw "Something went wrong"
catch e
    print("Error: " .. e)
end
```

## Standard Library

### Math Module

```pickup
import "math"

-- Constants
print(math.pi)   -- 3.14159...
print(math.e)    -- 2.71828...

-- Functions
print(math.floor(3.7))    -- 3
print(math.ceil(3.2))     -- 4
print(math.round(3.5))    -- 4
print(math.abs(-5))       -- 5
print(math.sqrt(16))      -- 4
print(math.pow(2, 8))     -- 256
print(math.min(5, 3))     -- 3
print(math.max(5, 3))     -- 5
print(math.sin(0))        -- 0
print(math.cos(0))        -- 1
print(math.random())      -- 0.0-1.0
print(math.clamp(15, 0, 10))  -- 10
```

### String Module

```pickup
import "string"

print(string.length("hello"))              -- 5
print(string.upper("hello"))               -- HELLO
print(string.lower("HELLO"))               -- hello
print(string.trim("  hello  "))            -- hello
print(string.substring("hello", 1, 3))     -- el
print(string.replace("hello", "l", "L"))   -- heLLo
print(string.contains("hello", "ell"))     -- true
print(string.starts_with("hello", "he"))   -- true
print(string.ends_with("hello", "lo"))     -- true
print(string.split("a,b,c", ","))          -- [a, b, c]
print(string.reverse("hello"))             -- olleh
print(string.repeat("ab", 3))              -- ababab
```

### Array Module

```pickup
import "array"

arr = [3, 1, 4, 1, 5]

print(array.length(arr))           -- 5
print(array.push(arr, 9))          -- [3, 1, 4, 1, 5, 9]
print(array.pop(arr))              -- [3, 1, 4, 1]
print(array.reverse(arr))          -- [5, 1, 4, 1, 3]
print(array.sort(arr))             -- [1, 1, 3, 4, 5]
print(array.contains(arr, 4))      -- true
print(array.find(arr, 4))          -- 2
print(array.join(arr, "-"))        -- 3-1-4-1-5
print(array.slice(arr, 1, 3))      -- [1, 4]
print(array.range(1, 5))           -- [1, 2, 3, 4]
```

### JSON Module

```pickup
import "json"

-- Stringify values to JSON
print(json.stringify([1, 2, 3]))           -- [1,2,3]
print(json.stringify({name = "John"}))     -- {"name":"John"}

-- Parse JSON strings
data = json.parse("[1, 2, 3]")
```

### File System Module

```pickup
import "fs"

-- Read/write files
content = fs.read("file.txt")
fs.write("output.txt", "Hello!")
fs.append("log.txt", "New line\n")

-- File operations
print(fs.exists("file.txt"))     -- true/false
print(fs.isfile("file.txt"))     -- true/false
print(fs.isdir("folder"))        -- true/false
print(fs.readdir("."))           -- [file1, file2, ...]
fs.mkdir("new_folder")
fs.remove("old_file.txt")
```

### Type Module

```pickup
import "type"

print(type.typeof(42))           -- number
print(type.typeof("hello"))      -- string
print(type.typeof(true))         -- boolean
print(type.typeof([1,2,3]))      -- table
print(type.typeof(nil))          -- nil

print(type.tonumber("42"))       -- 42
print(type.tostring(42))         -- "42"
print(type.tobool(1))            -- true

print(type.isnil(nil))           -- true
print(type.isnumber(42))         -- true
print(type.isstring("hi"))       -- true
```

### OS Module

```pickup
import "os"

print(os.time())                 -- Unix timestamp
print(os.clock())                -- High-precision time
print(os.getenv("HOME"))         -- Environment variable
print(os.execute("ls"))          -- Run shell command
os.sleep(1000)                   -- Sleep for 1000ms
```

## REPL Commands

```
>>> help     -- Show help
>>> exit     -- Exit REPL
>>> clear    -- Clear variables
>>> vars     -- Show variables
```

Multi-line input is automatically detected for if/while/for/function blocks.

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run from source
cargo run                      # REPL
cargo run -- script.up         # Execute file
cargo run -- script.up --noise # Verbose mode
```

## Test Suite

The language includes a comprehensive test suite with 43 tests covering:
- Arithmetic and string operations
- Control flow (if/else/elseif, loops)
- Functions and recursion
- Standard library modules
- Error handling
- Complex expressions and programs

Run tests with `cargo test`.

## Architecture

Pickup uses a bytecode compiler and stack-based virtual machine:

1. **Lexer/Parser** - Pest PEG grammar tokenizes source into AST
2. **Compiler** - Transforms AST into bytecode instructions
3. **VM** - Stack-based virtual machine executes bytecode

## License

MIT License - see LICENSE file for details.


