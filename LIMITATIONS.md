# Known Limitations and Future Work

This document tracks known limitations and planned improvements for Pickup-Lang.

## Current Limitations

### 1. REPL Multi-line Support
**Issue:** The REPL currently processes input line-by-line, which means multi-line constructs like if-statements and loops cannot be entered interactively.

**Workaround:** Use file-based execution for multi-line code.

**Example (won't work in REPL):**
```pickup
if x > 5 then
    print("Greater")
end
```

**Instead, use files:**
```bash
# Create a file with your code
echo 'x = 10
if x > 5 then
    print("Greater")
end' > test.up

# Execute it
pick test.up
```

### 2. Member Access (Dot Notation)
**Issue:** Module member access like `math.pi` or `json.parse()` is not yet supported. Modules are loaded but their members cannot be accessed directly.

**Status:** Modules load successfully, but individual function/constant access needs implementation.

### 3. Standard Library Native Implementations
**Issue:** Standard library functions (JSON, FS, String, Math) are currently placeholders. They're registered in the module system but don't perform actual operations.

**Status:** Framework is in place, native implementations need to be added.

### 4. Break and Continue
**Issue:** Loop control statements (break, continue) are not yet implemented.

**Workaround:** Use conditional logic to control loop execution.

### 5. Error Handling
**Issue:** No try-catch or error handling mechanism exists yet.

**Status:** Planned for future release.

### 6. Table Key-Value Access
**Issue:** Only array-style tables with numeric indices are supported. Dictionary-style tables with string keys are not yet implemented.

**Current Support:**
```pickup
arr = [1, 2, 3]
print(arr[0])  -- Works
```

**Not Yet Supported:**
```pickup
dict = {name = "Alice", age = 30}  -- Not supported
```

## Planned Improvements

### Short Term (Next Release)
1. Multi-line REPL support with continuation prompts
2. Member access (dot notation) for modules
3. Break and continue statements

### Medium Term
1. Native implementations for standard library functions
2. Basic error handling
3. Table key-value operations (dictionary support)
4. File I/O with real implementations

### Long Term
1. Optional static typing
2. Type inference
3. Advanced pattern matching
4. Module exports and private members
5. Async/await support
6. FFI (Foreign Function Interface) for C libraries
7. Debugger support
8. Performance optimizations (JIT compilation)

## Performance Considerations

### Current Performance Characteristics
- **Parsing:** Fast, uses Pest PEG parser
- **Compilation:** Single-pass bytecode generation
- **Execution:** Stack-based VM with simple instruction set
- **Memory:** No garbage collection (relies on Rust's ownership)

### Optimization Opportunities
1. **Constant Folding:** Evaluate constant expressions at compile time
2. **Dead Code Elimination:** Remove unreachable code
3. **Jump Threading:** Optimize control flow jumps
4. **Inline Caching:** Cache frequently accessed values
5. **JIT Compilation:** Compile hot code paths to native code

## Contributing

If you'd like to help address any of these limitations:
1. Check the GitHub issues for related discussions
2. Review the CONTRIBUTING.md file
3. Submit a pull request with your implementation
4. Include tests for new features

## Testing Coverage

Current test coverage focuses on:
- ✅ Arithmetic operations
- ✅ String concatenation
- ✅ Array operations
- ✅ Control flow (if/else)
- ✅ Loops (while, for)
- ✅ Comparison operators
- ✅ Logical operators
- ✅ Operator precedence
- ✅ Module imports
- ✅ Nil values
- ✅ User-defined functions
- ✅ Function calls with parameters
- ✅ Function return values

Areas needing more tests:
- ❌ Error conditions and edge cases
- ❌ Complex nested structures
- ❌ Standard library functions
- ❌ Performance benchmarks
