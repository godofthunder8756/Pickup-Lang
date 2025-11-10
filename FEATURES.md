# Pickup Language Features

This document provides a comprehensive overview of all features available in the Pickup programming language.

## Table of Contents
- [Basic Syntax](#basic-syntax)
- [Data Types](#data-types)
- [Variables](#variables)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Loops](#loops)
- [Arrays](#arrays)
- [Modules](#modules)
- [Standard Library](#standard-library)

## Basic Syntax

### Comments
```pickup
-- This is a single-line comment
```

### Print Statement
```pickup
print("Hello, World!")
print(42)
print(variable_name)
```

## Data Types

Pickup supports the following data types:

### Numbers
```pickup
x = 42          -- Integer
y = 3.14        -- Float
z = -17.5       -- Negative number
```

### Strings
```pickup
name = "Alice"
greeting = "Hello, World!"
```

### Booleans
```pickup
is_active = true
is_complete = false
```

### Nil
```pickup
nothing = nil   -- Represents absence of value
```

### Arrays (Tables)
```pickup
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", true, nil]
```

## Variables

### Variable Assignment
```pickup
x = 10
name = "Bob"
active = true
```

### Variable Scope
Variables are function-scoped (when functions are implemented) and globally accessible within the script.

## Operators

### Arithmetic Operators
```pickup
a = 10 + 5      -- Addition: 15
b = 10 - 5      -- Subtraction: 5
c = 10 * 5      -- Multiplication: 50
d = 10 / 5      -- Division: 2
```

### Comparison Operators
```pickup
x == y          -- Equal to
x ~= y          -- Not equal to
x < y           -- Less than
x > y           -- Greater than
x <= y          -- Less than or equal to
x >= y          -- Greater than or equal to
```

### Logical Operators
```pickup
true and false  -- Logical AND: false
true or false   -- Logical OR: true
not true        -- Logical NOT: false
```

### String Concatenation
```pickup
greeting = "Hello" .. ", " .. "World!"
result = "Answer: " .. 42
```

## Control Flow

### If Statement
```pickup
if x > 10 then
    print("x is greater than 10")
end
```

### If-Else Statement
```pickup
if x > 10 then
    print("x is greater than 10")
else
    print("x is not greater than 10")
end
```

### If-Elseif-Else Statement
```pickup
score = 85
if score >= 90 then
    print("Grade: A")
elseif score >= 80 then
    print("Grade: B")
elseif score >= 70 then
    print("Grade: C")
elseif score >= 60 then
    print("Grade: D")
else
    print("Grade: F")
end
```

## Loops

### While Loop
```pickup
i = 1
while i <= 5 do
    print(i)
    i = i + 1
end
```

### For Loop (Basic)
```pickup
-- Loop from 1 to 10
for i = 1, 10 do
    print(i)
end
```

### For Loop with Step
```pickup
-- Loop from 0 to 20 with step of 2
for i = 0, 20, 2 do
    print(i)  -- Prints even numbers
end
```

## Arrays

Arrays in Pickup use **0-based indexing** (unlike Lua which uses 1-based indexing).

### Creating Arrays
```pickup
numbers = [1, 2, 3, 4, 5]
fruits = ["apple", "banana", "cherry"]
mixed = [42, "hello", true, nil]
```

### Accessing Array Elements
```pickup
arr = [10, 20, 30, 40, 50]
first = arr[0]      -- 10
second = arr[1]     -- 20
last = arr[4]       -- 50
```

### Nested Arrays
```pickup
matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
element = matrix[1][2]  -- 6
```

## Modules

Pickup supports a module system for organizing code and accessing standard libraries.

### Importing Modules
```pickup
import "json"
import "fs"
import "string"
import "math"
```

## Standard Library

### Math Module

#### Constants
```pickup
import "math"
-- math.pi = 3.141592653589793
-- math.e = 2.718281828459045
```

#### Functions
- `floor(x)` - Returns the largest integer less than or equal to x
- `ceil(x)` - Returns the smallest integer greater than or equal to x
- `round(x)` - Rounds x to the nearest integer
- `abs(x)` - Returns the absolute value of x
- `min(a, b)` - Returns the minimum of a and b
- `max(a, b)` - Returns the maximum of a and b
- `sqrt(x)` - Returns the square root of x
- `pow(x, y)` - Returns x raised to the power of y
- `sin(x)` - Returns the sine of x (in radians)
- `cos(x)` - Returns the cosine of x (in radians)
- `tan(x)` - Returns the tangent of x (in radians)
- `random()` - Returns a random number between 0 and 1

### String Module

Functions for string manipulation:
- `length(s)` - Returns the length of string s
- `upper(s)` - Converts string s to uppercase
- `lower(s)` - Converts string s to lowercase
- `substring(s, start, end)` - Returns a substring
- `split(s, delimiter)` - Splits string by delimiter
- `trim(s)` - Removes leading/trailing whitespace
- `replace(s, old, new)` - Replaces occurrences in string

### JSON Module

Functions for JSON processing:
- `parse(json_string)` - Parses a JSON string into a value
- `stringify(value)` - Converts a value to a JSON string

### FS Module

Functions for file system operations:
- `read(path)` - Reads a file and returns its contents
- `write(path, content)` - Writes content to a file
- `exists(path)` - Checks if a file or directory exists
- `readdir(path)` - Lists files in a directory
- `mkdir(path)` - Creates a directory

**Note:** Standard library functions are currently placeholders. Full native implementations are planned for future releases.

## Examples

### Example 1: FizzBuzz
```pickup
for i = 1, 100 do
    if i % 15 == 0 then
        print("FizzBuzz")
    elseif i % 3 == 0 then
        print("Fizz")
    elseif i % 5 == 0 then
        print("Buzz")
    else
        print(i)
    end
end
```

### Example 2: Array Processing
```pickup
numbers = [5, 12, 8, 130, 44]

-- Find maximum
max = numbers[0]
for i = 1, 4 do
    if numbers[i] > max then
        max = numbers[i]
    end
end
print("Maximum: " .. max)
```

### Example 3: String Processing
```pickup
text = "Hello"
for i = 1, 5 do
    print(text .. " " .. i)
end
```

## Future Features

Planned features for upcoming releases:
- User-defined functions with parameters and return values
- Full function call support with native implementations
- Table/dictionary operations (key-value pairs)
- Error handling (try/catch)
- Module definition and exports
- Type annotations (optional static typing)
- Native implementations for standard library functions
- More advanced control flow (break, continue)
- Pattern matching
