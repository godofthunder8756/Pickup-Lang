# Pickup

Pickup is a lightweight scripting language inspired by Lua, designed as a modern drop-in replacement. It features:
- 0-based indexing
- Optional static typing
- Standard libraries (JSON, FS, etc.)
- Improved error handling & stack traces
- Clean ES-style module system

## Roadmap
1. REPL for syntax/feature testing
2. File execution for script usage
3. Parser → AST → Interpreter
4. Module system + std lib

## Getting Started
```bash
# Clone and run (starts REPL)
git clone https://github.com/yourusername/pickup-lang.git
cd pickup-lang
cargo run

# Execute a file
cargo run -- path/to/script.pickup

---

## Cargo.toml
```toml
[package]
name = "pickup-lang"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "Pickup: A modern Lua-inspired scripting language"
license = "MIT"

[dependencies]
# CLI parsing
clap = { version = "4.1", features = ["derive"] }
# Parsing
pest = "2.4"
pest_derive = "2.4"
# REPL
rustyline = "11.0"
# Error handling
thiserror = "1.0"
# JSON support
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Bytecode Compiler

The compiler transforms parsed AST nodes into a simple bytecode which is then
executed by a tiny virtual machine. Running `cargo run` will start the REPL
using this compiler.
