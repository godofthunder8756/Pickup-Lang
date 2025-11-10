# Contributing to Pickup-Lang

Thank you for your interest in contributing to Pickup-Lang! This document provides guidelines for contributing to the project.

## Development Setup

1. **Prerequisites**:
   - Rust (latest stable version)
   - Git
   - A code editor (VS Code, IntelliJ IDEA, etc.)

2. **Clone the repository**:
   ```bash
   git clone https://github.com/godofthunder8756/Pickup-Lang.git
   cd Pickup-Lang
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

## Making Changes

1. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style:
   ```bash
   # Format your code
   cargo fmt
   
   # Run clippy for linting
   cargo clippy
   ```

3. **Test your changes**:
   ```bash
   cargo test
   cargo run  # Test the REPL
   cargo run -- examples/basic.up  # Test file execution
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Description of your changes"
   ```

5. **Push your branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

## Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Write descriptive commit messages
- Add comments for complex logic
- Keep functions focused and concise

## Pull Request Guidelines

- **Title**: Use a clear, descriptive title
- **Description**: Explain what changes you made and why
- **Tests**: Add tests for new functionality
- **CI**: Ensure all CI checks pass

### CI Checks

When you create a pull request, GitHub Actions will automatically:
- Build your code on Linux, macOS, and Windows
- Run all tests
- Check code formatting
- Run clippy linting

Make sure all checks pass before requesting a review.

## Reporting Issues

When reporting bugs, please include:
- Pickup-Lang version (`pick --version`)
- Operating system
- Steps to reproduce the issue
- Expected vs. actual behavior
- Error messages (if any)

## Feature Requests

We welcome feature requests! Please:
- Check if the feature has already been requested
- Clearly describe the feature and its use case
- Explain how it would benefit users

## Questions?

If you have questions about contributing, feel free to:
- Open an issue for discussion
- Reach out to the maintainers

## License

By contributing to Pickup-Lang, you agree that your contributions will be licensed under the MIT License.
