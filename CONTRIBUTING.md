# Contributing to RustFlix

We welcome contributions to RustFlix! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.70+ (latest stable recommended)
- PostgreSQL 14+
- Redis 6+
- FFmpeg 5.0+
- Git

### Local Development

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/rustflix.git
   cd rustflix
   ```

3. Set up the development environment:
   ```bash
   # Install dependencies
   cargo build

   # Set up database (requires PostgreSQL running)
   createdb rustflix_dev
   export DATABASE_URL="postgresql://username:password@localhost/rustflix_dev"
   
   # Set up Redis (requires Redis running)
   export REDIS_URL="redis://localhost:6379"
   ```

4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Code Style

### Rust Guidelines

- Follow standard Rust formatting with `rustfmt`
- Use `clippy` for linting
- Maintain test coverage above 90%
- Document all public APIs with rustdoc comments
- Use meaningful variable and function names

### Code Organization

- Keep modules focused and cohesive
- Use the repository pattern for data access
- Implement proper error handling with typed errors
- Write async code throughout the stack
- Follow the established project structure

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with coverage
cargo tarpaulin --out html

# Run specific test suite
cargo test --package rustflix-database

# Run integration tests
cargo test --test integration
```

### Test Requirements

- Unit tests for all business logic
- Integration tests for API endpoints
- Database tests with test containers
- Performance benchmarks for critical paths
- Error case coverage

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_function_name() {
        // Arrange
        let input = setup_test_data();
        
        // Act
        let result = function_under_test(input).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

## Pull Request Process

### Before Submitting

1. Ensure all tests pass
2. Run `cargo fmt` and `cargo clippy`
3. Update documentation if needed
4. Add tests for new functionality
5. Verify performance impact

### PR Guidelines

1. Create a feature branch from `main`
2. Use descriptive commit messages
3. Keep PRs focused and atomic
4. Include tests and documentation
5. Update CHANGELOG.md if applicable

### Commit Message Format

```
type(scope): brief description

Longer description if needed

- List any breaking changes
- Reference issues: Fixes #123
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Architecture Guidelines

### Module Design

- Each crate should have a single responsibility
- Use traits for abstraction boundaries
- Implement proper dependency injection
- Maintain clear API boundaries

### Performance Considerations

- Use async/await throughout
- Implement connection pooling
- Cache frequently accessed data
- Profile critical paths
- Minimize allocations in hot paths

### Error Handling

- Use typed errors with `thiserror`
- Provide meaningful error messages
- Log errors at appropriate levels
- Handle errors gracefully

## Documentation

### Code Documentation

- Document all public APIs
- Include examples in rustdoc
- Explain complex algorithms
- Document performance characteristics

### Architecture Documentation

- Update architecture docs for significant changes
- Document API changes
- Maintain deployment guides
- Keep configuration examples current

## Security

### Security Guidelines

- Validate all inputs
- Use parameterized queries
- Implement proper authentication
- Follow secure coding practices
- Report security issues privately

### Reporting Security Issues

Email security issues to: security@rustflix.dev

Do not create public issues for security vulnerabilities.

## Performance

### Benchmarking

- Benchmark critical paths
- Compare against baseline performance
- Document performance characteristics
- Test under load

### Optimization Guidelines

- Profile before optimizing
- Focus on algorithmic improvements
- Use appropriate data structures
- Minimize memory allocations

## Release Process

### Version Numbering

We follow Semantic Versioning (SemVer):
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes (backward compatible)

### Release Checklist

1. Update version numbers
2. Update CHANGELOG.md
3. Run full test suite
4. Create release tag
5. Build and test release artifacts
6. Update documentation

## Community

### Communication

- Use GitHub Issues for bug reports
- Use GitHub Discussions for questions
- Be respectful and constructive
- Help others when possible

### Code of Conduct

We follow the Rust Code of Conduct. Be kind, respectful, and inclusive.

## Getting Help

- Check existing issues and documentation
- Ask questions in GitHub Discussions
- Join our community channels
- Read the architecture documentation

Thank you for contributing to RustFlix!
