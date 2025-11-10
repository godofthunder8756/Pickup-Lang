# Changelog

All notable changes to Pickup-Lang will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Control Flow:** If-else-elseif statements for conditional execution
- **Loops:** While loops and for loops with optional step values
- **Comparison Operators:** ==, ~=, <, >, <=, >= for value comparisons
- **Logical Operators:** and, or, not for boolean logic
- **Nil Value:** Proper nil type support throughout the language
- **Enhanced Standard Library:**
  - Math module with constants (pi, e) and 13 mathematical functions
  - String module with 7 string manipulation utilities
  - Enhanced JSON module (parse, stringify)
  - Enhanced FS module (read, write, exists, readdir, mkdir)
- **Test Suite:** 12 comprehensive integration tests
- **Documentation:**
  - FEATURES.md - Complete language feature documentation
  - LIMITATIONS.md - Known limitations and future work
  - Enhanced README.md with examples
- **Library Interface:** Exposed public API for embedding Pickup in Rust projects

### Changed
- Updated grammar to support new control flow constructs
- Improved VM execution to support jumps and conditional branching
- Enhanced compiler to handle comparison and logical operators
- Updated AST to include new node types (If, While, For, Return, etc.)

### Fixed
- **Operator precedence:** Complex expressions like `x >= 10 and x <= 20` now evaluate correctly
- Operator precedence in grammar (longer operators matched first)
- Keyword boundary detection for 'and' and 'or' operators
- Parser handling of complex nested expressions

## [0.1.0] - Previous Release

### Added
- Basic REPL for interactive code execution
- File execution support
- Parser with Pest grammar
- AST-based compilation to bytecode
- Stack-based virtual machine
- Basic arithmetic operations (+, -, *, /)
- String concatenation (..)
- Variables and assignment
- Arrays with 0-based indexing
- Print statements
- Module system with import statements
- Basic JSON and FS module placeholders

### Infrastructure
- GitHub Actions CI/CD pipeline
- Automated release builds for multiple platforms
- Cross-platform support (Linux, macOS, Windows)

[Unreleased]: https://github.com/godofthunder8756/Pickup-Lang/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/godofthunder8756/Pickup-Lang/releases/tag/v0.1.0
