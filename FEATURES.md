# Pickup Language Features

This document provides a comprehensive overview of all features available in the Pickup programming language.

## Table of Contents
- [Basic Syntax](#basic-syntax)
- [Data Types](#data-types)
- [Variables](#variables)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Loops](#loops)
- [Functions](#functions)
- [Arrays](#arrays)
- [Dictionaries](#dictionaries)
- [Modules](#modules)
- [Standard Library](#standard-library)
- [Error Handling](#error-handling)

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

### Dictionaries
```pickup
person = {name = "Alice", age = 30}
```

## Variables

### Variable Assignment
```pickup
x = 10
name = "Bob"
active = true
```

### Local Variables
```pickup
function example()
    local x = 10  -- Local to this function
end
```

## Operators

### Arithmetic Operators
```pickup
a = 10 + 5      -- Addition: 15
b = 10 - 5      -- Subtraction: 5
c = 10 * 5      -- Multiplication: 50
d = 10 / 5      -- Division: 2
e = 10 % 3      -- Modulo: 1
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
for i = 1, 10 do
    print(i)
end
```

### For Loop with Step
```pickup
for i = 0, 20, 2 do
    print(i)  -- Prints even numbers
end
```

### Break and Continue
```pickup
for i = 1, 10 do
    if i == 5 then break end
    print(i)
end
```

## Functions

### Function Definition
```pickup
function greet(name)
    print("Hello, " .. name)
end

greet("Alice")
```

### Return Values
```pickup
function add(a, b)
    return a + b
end

result = add(3, 5)
```

### Recursive Functions
```pickup
function factorial(n)
    if n <= 1 then
        return 1
    else
        return n * factorial(n - 1)
    end
end
```

## Arrays

Arrays in Pickup use **0-based indexing**.

### Creating Arrays
```pickup
numbers = [1, 2, 3, 4, 5]
fruits = ["apple", "banana", "cherry"]
```

### Accessing Array Elements
```pickup
arr = [10, 20, 30, 40, 50]
first = arr[0]      -- 10
second = arr[1]     -- 20
```

## Dictionaries

### Creating Dictionaries
```pickup
person = {name = "Alice", age = 30, active = true}
```

### Accessing Dictionary Values
```pickup
print(person.name)  -- "Alice"
print(person["age"]) -- 30
```

## Modules

### Importing Modules
```pickup
import "math"
import "string"
import "array"
import "json"
import "fs"
import "os"
import "type"
import "table"
import "base64"
import "datetime"
```

## Standard Library

### Math Module

#### Constants
- `math.pi` - Pi (3.141592653589793)
- `math.e` - Euler's number (2.718281828459045)
- `math.inf` - Infinity
- `math.nan` - Not a Number

#### Functions
- `floor(x)` - Largest integer ≤ x
- `ceil(x)` - Smallest integer ≥ x
- `round(x)` - Round to nearest integer
- `trunc(x)` - Truncate decimal part
- `fract(x)` - Get fractional part
- `abs(x)` - Absolute value
- `sign(x)` - Sign of x (-1, 0, or 1)
- `min(a, b)` - Minimum of a and b
- `max(a, b)` - Maximum of a and b
- `clamp(x, min, max)` - Clamp x between min and max
- `sqrt(x)` - Square root
- `cbrt(x)` - Cube root
- `pow(x, y)` - x raised to power y
- `sin(x)`, `cos(x)`, `tan(x)` - Trigonometric functions
- `asin(x)`, `acos(x)`, `atan(x)` - Inverse trigonometric
- `atan2(y, x)` - Two-argument arctangent
- `sinh(x)`, `cosh(x)`, `tanh(x)` - Hyperbolic functions
- `log(x)` - Natural logarithm
- `log2(x)` - Base 2 logarithm
- `log10(x)` - Base 10 logarithm
- `exp(x)` - e^x
- `deg(x)` - Radians to degrees
- `rad(x)` - Degrees to radians
- `hypot(x, y)` - Hypotenuse (sqrt(x² + y²))
- `lerp(a, b, t)` - Linear interpolation
- `fmod(x, y)` - Floating-point modulo
- `isnan(x)` - Check if NaN
- `isinf(x)` - Check if infinite
- `random()` - Random number between 0 and 1
- `randomint(min, max)` - Random integer in range

### String Module

- `length(s)` - String length
- `upper(s)` - Convert to uppercase
- `lower(s)` - Convert to lowercase
- `trim(s)` - Remove leading/trailing whitespace
- `ltrim(s)` - Remove leading whitespace
- `rtrim(s)` - Remove trailing whitespace
- `capitalize(s)` - Capitalize first letter
- `title(s)` - Title case
- `substring(s, start, end)` - Extract substring
- `split(s, delimiter)` - Split string into array
- `replace(s, old, new)` - Replace all occurrences
- `replace_first(s, old, new)` - Replace first occurrence
- `contains(s, sub)` - Check if contains substring
- `starts_with(s, prefix)` - Check if starts with
- `ends_with(s, suffix)` - Check if ends with
- `find(s, sub)` - Find index of substring (-1 if not found)
- `count(s, sub)` - Count occurrences of substring
- `reverse(s)` - Reverse string
- `repeat(s, n)` - Repeat string n times
- `pad_left(s, width, char)` - Left pad string
- `pad_right(s, width, char)` - Right pad string
- `char(code)` - Character from code point
- `byte(s)` - Code point of first character
- `lines(s)` - Split into lines
- `chars(s)` - Split into characters
- `is_empty(s)` - Check if empty
- `is_numeric(s)` - Check if numeric
- `is_alpha(s)` - Check if alphabetic
- `is_alphanumeric(s)` - Check if alphanumeric
- `insert(s, pos, str)` - Insert string at position
- `remove(s, start, end)` - Remove substring
- `match(s, pattern)` - Glob pattern matching

### Array Module

- `length(arr)` - Array length
- `first(arr)` - First element
- `last(arr)` - Last element
- `push(arr, val)` - Add element to end
- `pop(arr)` - Remove last element
- `shift(arr)` - Remove first element
- `unshift(arr, val)` - Add element to beginning
- `insert(arr, index, val)` - Insert at index
- `remove_at(arr, index)` - Remove at index
- `slice(arr, start, end)` - Extract portion
- `concat(arr1, arr2)` - Concatenate arrays
- `join(arr, delimiter)` - Join into string
- `reverse(arr)` - Reverse array
- `sort(arr)` - Sort array
- `contains(arr, val)` - Check if contains value
- `find(arr, val)` - Find index of value
- `count(arr, val)` - Count occurrences
- `unique(arr)` - Remove duplicates
- `flatten(arr)` - Flatten nested arrays
- `filter_nil(arr)` - Remove nil values
- `fill(size, val)` - Create array filled with value
- `range(start, end, step)` - Create range array
- `min(arr)` - Minimum value
- `max(arr)` - Maximum value
- `sum(arr)` - Sum of all numbers
- `avg(arr)` - Average of all numbers
- `zip(arr1, arr2)` - Zip two arrays together
- `swap(arr, i, j)` - Swap elements
- `copy(arr)` - Shallow copy
- `clear(arr)` - Return empty array

### Table Module (Dictionary Operations)

- `keys(dict)` - Get all keys
- `values(dict)` - Get all values
- `entries(dict)` - Get key-value pairs as array
- `has(dict, key)` - Check if key exists
- `get(dict, key, default)` - Get value with default
- `set(dict, key, val)` - Set key-value pair
- `delete(dict, key)` - Delete key
- `merge(dict1, dict2)` - Merge dictionaries
- `size(dict)` - Number of entries
- `copy(dict)` - Shallow copy
- `clear(dict)` - Return empty dictionary
- `from_entries(arr)` - Create dict from entries
- `invert(dict)` - Swap keys and values

### JSON Module

- `parse(str)` - Parse JSON string
- `stringify(val)` - Convert to JSON string
- `pretty(val)` - Convert to pretty JSON string
- `valid(str)` - Check if valid JSON

### FS Module (File System)

- `read(path)` - Read file contents
- `write(path, content)` - Write to file
- `append(path, content)` - Append to file
- `exists(path)` - Check if path exists
- `isfile(path)` - Check if is file
- `isdir(path)` - Check if is directory
- `is_symlink(path)` - Check if is symlink
- `readdir(path)` - List directory contents
- `mkdir(path)` - Create directory
- `remove(path)` - Remove file/directory
- `copy(src, dst)` - Copy file
- `rename(src, dst)` - Move/rename file
- `basename(path)` - Get filename from path
- `dirname(path)` - Get directory from path
- `extension(path)` - Get file extension
- `join_path(p1, p2, ...)` - Join path components
- `absolute(path)` - Get absolute path
- `filesize(path)` - Get file size in bytes
- `modified(path)` - Get modification timestamp
- `created(path)` - Get creation timestamp
- `read_bytes(path)` - Read as byte array
- `write_bytes(path, bytes)` - Write byte array

### OS Module

- `time()` - Unix timestamp (seconds)
- `clock()` - High-precision timestamp
- `sleep(ms)` - Sleep for milliseconds
- `exit(code)` - Exit program
- `getenv(name)` - Get environment variable
- `setenv(name, val)` - Set environment variable
- `unsetenv(name)` - Unset environment variable
- `envvars()` - Get all environment variables
- `execute(cmd)` - Execute shell command
- `platform()` - Get OS name
- `arch()` - Get CPU architecture
- `hostname()` - Get hostname
- `cwd()` - Get current working directory
- `chdir(path)` - Change directory
- `home()` - Get home directory
- `tmpdir()` - Get temp directory
- `pid()` - Get process ID
- `args()` - Get command line arguments
- `user()` - Get current username

### Type Module

- `typeof(val)` - Get type name as string
- `tonumber(val)` - Convert to number
- `tostring(val)` - Convert to string
- `tobool(val)` - Convert to boolean
- `isnil(val)` - Check if nil
- `isnumber(val)` - Check if number
- `isstring(val)` - Check if string
- `isbool(val)` - Check if boolean
- `istable(val)` - Check if array
- `isarray(val)` - Check if array (alias)
- `isdict(val)` - Check if dictionary
- `isdictionary(val)` - Check if dictionary (alias)
- `isfunction(val)` - Check if function
- `ismodule(val)` - Check if module
- `isempty(val)` - Check if empty
- `default(val, def)` - Return default if nil

### Base64 Module

- `encode(str)` - Encode string to base64
- `decode(str)` - Decode base64 to string

### DateTime Module

- `now()` - Current timestamp
- `timestamp()` - Current timestamp (alias)
- `format(ts, fmt)` - Format timestamp to string
- `parse(str, fmt)` - Parse string to timestamp
- `year(ts)` - Get year
- `month(ts)` - Get month (1-12)
- `day(ts)` - Get day of month
- `hour(ts)` - Get hour (0-23)
- `minute(ts)` - Get minute (0-59)
- `second(ts)` - Get second (0-59)
- `weekday(ts)` - Get day of week (0=Sunday)
- `day_of_year(ts)` - Get day of year (1-366)
- `is_leap_year(year)` - Check if leap year
- `add_days(ts, days)` - Add days to timestamp
- `add_hours(ts, hours)` - Add hours to timestamp
- `add_minutes(ts, mins)` - Add minutes to timestamp
- `add_seconds(ts, secs)` - Add seconds to timestamp
- `diff(ts1, ts2)` - Difference in seconds
- `from_timestamp(ts)` - Convert to dict with components

## Error Handling

### Try-Catch
```pickup
try
    result = risky_operation()
catch error
    print("Error: " .. error)
end
```

### Throw
```pickup
throw "Something went wrong!"
```

## Examples

### Example: Working with Dates
```pickup
import "datetime"

now = datetime.now()
print("Current time: " .. datetime.format(now))
print("Year: " .. datetime.year(now))

future = datetime.add_days(now, 7)
print("Next week: " .. datetime.format(future))
```

### Example: JSON Processing
```pickup
import "json"

data = {name = "Alice", scores = [95, 87, 92]}
json_str = json.pretty(data)
print(json_str)

parsed = json.parse('{"x": 10, "y": 20}')
print(parsed.x + parsed.y)
```

### Example: File Operations
```pickup
import "fs"

content = fs.read("input.txt")
lines = string.lines(content)

for i = 0, array.length(lines) - 1 do
    print("Line " .. i .. ": " .. lines[i])
end

fs.write("output.txt", "Processed!")
```

### Example: Data Processing
```pickup
import "array"
import "math"

data = [45, 67, 23, 89, 34, 56, 78, 12]

print("Sum: " .. array.sum(data))
print("Average: " .. array.avg(data))
print("Min: " .. array.min(data))
print("Max: " .. array.max(data))

sorted = array.sort(data)
print("Sorted: " .. array.join(sorted, ", "))
```
