# Contributing to CID

Thank you for your interest in contributing to CID! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Documentation](#documentation)
- [Pull Requests](#pull-requests)
- [Issues](#issues)
- [Style Guidelines](#style-guidelines)

## Code of Conduct

Please be respectful and constructive in all interactions. We are committed to providing a welcoming and inclusive experience for everyone.

## Getting Started

1. **Fork the repository** on Codeberg
2. **Clone your fork** locally:
   ```bash
   git clone https://codeberg.org/YOUR_USERNAME/cid.git
   cd cid
   ```
3. **Add upstream remote**:
   ```bash
   git remote add upstream https://codeberg.org/NutypeBuddha/cid.git
   ```
4. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With proxy feature
cargo build --release --features proxy
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_math_validation

# Run benchmarks
cargo bench
```

### Code Quality

```bash
# Lint with Clippy
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

## Making Changes

### Branch Naming

Use descriptive branch names:
- `feature/add-new-gate` - New features
- `fix/math-parser-bug` - Bug fixes
- `docs/update-readme` - Documentation
- `refactor/cleanup-gates` - Code refactoring

### Commit Messages

Write clear, concise commit messages:
- Use imperative mood ("Add feature" not "Added feature")
- Keep first line under 72 characters
- Reference issues when applicable

Example:
```
Add formal verification gate

- Implement symbolic logic validation
- Add 15 new patterns for formal reasoning
- Update documentation with new gate

Closes #42
```

### Code Structure

Follow the existing code structure:
- `src/core/` - Core types (Pin, Ball, Pocket)
- `src/gates/` - Validation gates
- `src/inference/` - Pipeline and proxy
- `src/mcp/` - MCP server and tools
- `src/tanto/` - Math operations
- `src/kb/` - Knowledge base
- `src/plugin/` - WASM plugin system
- `cid-wasm/` - WASM build target

## Testing

### Writing Tests

Add tests for new functionality:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_feature() {
        // Arrange
        let input = "test input";
        
        // Act
        let result = my_new_function(input);
        
        // Assert
        assert_eq!(result, expected_output);
    }
}
```

### Test Categories

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test component interactions
- **Benchmarks**: Performance testing

### Running Specific Tests

```bash
# By test name
cargo test test_math

# By module
cargo test gates::math

# By exact match
cargo test -- --exact test_name
```

## Documentation

### Code Documentation

Add documentation comments for public items:
```rust
/// Validates a mathematical expression.
///
/// # Arguments
///
/// * `expr` - The mathematical expression to validate
///
/// # Returns
///
/// A tuple of (is_valid, confidence_score).
///
/// # Examples
///
/// ```
/// let (valid, score) = validate_math("2 + 3 = 5");
/// assert!(valid);
/// ```
pub fn validate_math(expr: &str) -> (bool, f64) {
    // Implementation
}
```

### README Updates

Update README.md when adding:
- New features
- New commands
- New MCP tools
- Changed installation steps

## Pull Requests

### Before Submitting

1. **Run tests**: `cargo test`
2. **Run linter**: `cargo clippy -- -D warnings`
3. **Format code**: `cargo fmt`
4. **Update documentation** if needed
5. **Update CHANGELOG.md** with your changes

### PR Template

```markdown
## Description

Brief description of changes

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist

- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
```

### Review Process

1. Submit PR with clear description
2. Address review feedback
3. Ensure all checks pass
4. Maintainer will merge

## Issues

### Reporting Bugs

Include:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment (OS, Rust version)
- Error messages if applicable

### Suggesting Features

Include:
- Use case
- Proposed solution
- Alternatives considered
- Additional context

## Style Guidelines

### Rust Style

- Follow Rust API Guidelines
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Prefer explicit over implicit
- Write meaningful variable names

### Naming Conventions

- `snake_case` for functions and variables
- `PascalCase` for types and traits
- `SCREAMING_SNAKE_CASE` for constants
- `kebab-case` for feature flags

### Error Handling

- Use `Result` for fallible operations
- Provide meaningful error messages
- Use `thiserror` for custom errors (if added)

## Getting Help

- **Issues**: For bug reports and feature requests
- **Discussions**: For questions and ideas
- **Codeberg**: https://codeberg.org/NutypeBuddha/cid

## License

By contributing, you agree that your contributions will be licensed under the Unlicense (public domain).
