# Pickup

Pickup is a lightweight scripting language inspired by Lua, designed as a modern drop-in replacement. It features:
- 0-based indexing
- Optional static typing
- Standard libraries (JSON, FS, etc.)
- Improved error handling & stack traces
- Clean ES-style module system

## Installation

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/godofthunder8756/Pickup-Lang/releases).

**Linux/macOS:**
```bash
# Download the appropriate binary for your platform
chmod +x pick-*
sudo mv pick-* /usr/local/bin/pick
```

**Windows:**
Download the `.exe` file and add it to your PATH.

### Build from Source

```bash
# Clone and build
git clone https://github.com/godofthunder8756/Pickup-Lang.git
cd Pickup-Lang
cargo build --release

# The binary will be at target/release/pick
```

## Roadmap

All roadmap features are now implemented! ✅

1. ✅ REPL for syntax/feature testing
2. ✅ File execution for script usage
3. ✅ Parser → AST → Interpreter (bytecode compiler & VM)
4. ✅ Module system + std lib

## Language Features

### Arrays and Tables (0-based indexing)
```pickup
-- Create arrays with 0-based indexing
numbers = [1, 2, 3, 4, 5]
print(numbers[0])  -- Prints: 1

fruits = ["apple", "banana", "cherry"]
print(fruits[2])  -- Prints: cherry
```

### Module System
```pickup
-- Import standard library modules
import "json"
import "fs"

-- Modules are loaded and available
-- (Native function implementations coming soon)
```

### String Operations
```pickup
-- String concatenation with ..
greeting = "Hello" .. ", " .. "World!"
print(greeting)  -- Prints: Hello, World!

-- String interpolation
name = "Pickup"
message = "Welcome to " .. name
print(message)
```

### Arithmetic Operations
```pickup
x = 10
y = 20
sum = x + y
print(sum)  -- Prints: 30

-- Supported operators: +, -, *, /
```

### Variables and Assignment
```pickup
-- Simple assignment
x = 42
name = "Alice"
active = true

-- Expression assignment
result = x + y * 2
```

## Getting Started
```bash
# Start the REPL
pick

# Execute a file
pick path/to/script.pickup
```

## Development

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Running from Source
```bash
# Start REPL
cargo run

# Execute a file
cargo run -- path/to/script.pickup
```

## Bytecode Compiler

The compiler transforms parsed AST nodes into a simple bytecode which is then
executed by a tiny virtual machine. Running `cargo run` will start the REPL
using this compiler.

## Releases

This project uses automated releases via GitHub Actions. To create a new release:

1. Update the version in `Cargo.toml`
2. Commit the change: `git commit -am "Bump version to X.Y.Z"`
3. Create and push a tag: `git tag vX.Y.Z && git push origin vX.Y.Z`
4. GitHub Actions will automatically build binaries for all platforms and create a release

The release pipeline builds binaries for:
- Linux (x86_64, both glibc and musl)
- macOS (x86_64 Intel and aarch64 Apple Silicon)
- Windows (x86_64)

## CI/CD

This project uses GitHub Actions for continuous integration and deployment:
- **CI Pipeline**: Runs on every push and pull request, building and testing on Linux, macOS, and Windows
- **Release Pipeline**: Automatically creates releases with pre-built binaries when version tags are pushed
