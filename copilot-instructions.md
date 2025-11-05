# GitHub Copilot Coding Agent Instructions for svmai-cli

## Project Overview

`svmai` is a Rust-based command-line interface (CLI) tool for managing Solana wallets. It provides secure wallet management, multi-threaded wallet search, balance tracking, and batch operations with a text-based user interface (TUI).

## Language and Framework

- **Primary Language**: Rust (Edition 2021)
- **Build System**: Cargo
- **Key Frameworks**: 
  - `ratatui` with `crossterm` for TUI
  - `solana-sdk` for Solana blockchain integration
  - `keyring` for secure credential storage

## Coding Standards

### Code Style
- Follow Rust's official style guide and idiomatic Rust patterns
- Use `cargo fmt` to automatically format code before committing
- Run `cargo clippy -- -D warnings` to catch common mistakes and warnings
- All clippy warnings must be addressed before PR approval
- Write clear, descriptive comments for complex logic
- Use rustdoc comments (`///`) for all public API functions and structs

### Naming Conventions
- Use `snake_case` for functions, variables, and module names
- Use `PascalCase` for types, traits, and enum variants
- Use `SCREAMING_SNAKE_CASE` for constants
- Choose descriptive names that clearly indicate purpose

### Error Handling
- Use `Result<T, E>` for operations that can fail
- Define custom error types using `thiserror` when appropriate
- Provide meaningful error messages that help users understand what went wrong
- Handle errors gracefully in TUI components to prevent crashes

### Security
- Never log or display private keys in plain text
- Use secure storage mechanisms (system keychain) for sensitive data
- Validate all user inputs before processing
- Follow cryptographic best practices when handling encryption
- Use `zeroize` or similar for sensitive data in memory when appropriate

## Architecture

The project follows a modular architecture with clear separation of concerns:

- **`main.rs`**: CLI entry point and argument parsing
- **`file_searcher.rs`**: Multi-threaded wallet file discovery
- **`key_validator.rs`**: Solana private key validation
- **`secure_storage.rs`**: Encrypted storage using system keychain
- **`wallet_manager.rs`**: High-level wallet management operations
- **`transaction_handler.rs`**: Transaction creation and batch operations
- **`vanity_wallet.rs`**: Vanity address generation
- **`tui.rs`**: Text-based user interface using ratatui
- **`config.rs`**: Configuration management
- **`logging.rs`**: Logging utilities

### Module Guidelines
- Keep modules focused on a single responsibility
- Use public interfaces (`pub`) only for necessary exports
- Document module-level behavior with module comments
- Maintain clear boundaries between modules

## Dependencies

### Adding New Dependencies
- Only add dependencies that are actively maintained and widely used
- Prefer pure Rust implementations when available
- Check for security advisories before adding new crates
- Document why a dependency is needed in commit messages
- Update `Cargo.toml` with appropriate version constraints

### Version Management
- Use semantic versioning for dependencies
- Prefer exact versions for cryptographic libraries
- Use `~` or `^` operators for other dependencies as appropriate

## Testing

### Test Requirements
- Write unit tests for all new functionality
- Place tests in a `tests` module within each source file
- Use `#[cfg(test)]` to conditionally compile tests
- Aim for high code coverage, especially for critical paths
- Test edge cases and error conditions

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptive_name() {
        // Test implementation
    }
}
```

### Running Tests
- Run `cargo test` before submitting PRs
- Run `cargo test --verbose` for detailed output
- Ensure all tests pass; do not disable or remove existing tests
- Use `tempfile` crate for temporary file operations in tests

### Integration Testing
- For TUI changes, include manual testing steps in PR description
- Test on multiple platforms when possible (Linux, macOS, Windows)
- Verify keychain integration works correctly on target platforms

## Building and Linting

### Build Commands
```bash
# Development build
cargo build

# Release build
cargo build --release

# Clean build artifacts
cargo clean
```

### Pre-commit Checks
Always run these commands before committing:
```bash
# Format code
cargo fmt

# Check for common issues
cargo clippy -- -D warnings

# Run tests
cargo test
```

## Documentation

### Code Documentation
- Use rustdoc comments (`///`) for public APIs
- Include examples in documentation when helpful
- Document panics, errors, and safety requirements
- Keep documentation up-to-date with code changes

### External Documentation
- Update README.md for user-facing changes
- Update architecture.md for architectural changes
- Update CONTRIBUTING.md for process changes
- Maintain CHANGELOG.md for version history

## Performance Considerations

- Use `rayon` for parallel processing where appropriate
- Avoid unnecessary allocations in hot paths
- Profile code before optimizing (don't prematurely optimize)
- Consider memory usage for operations on large wallet collections

## Platform Compatibility

- Support Linux, macOS, and Windows
- Test keychain integration on all target platforms
- Handle platform-specific paths using the `dirs` crate
- Document any platform-specific limitations

## Git Workflow

### Commit Messages
- Use clear, descriptive commit messages
- Start with a verb in present tense (e.g., "Add", "Fix", "Update")
- Reference issue numbers when applicable
- Keep commits focused and atomic

### Pull Requests
- Include a clear description of changes
- Link to related issues
- Include test results and manual testing steps
- Ensure CI passes before requesting review
- Address review feedback promptly

## Common Patterns

### Wallet Operations
- Always validate wallet data before operations
- Use secure storage for private keys
- Provide progress feedback for long-running operations
- Allow cancellation of long-running operations

### TUI Development
- Use the `App` struct to manage application state
- Handle keyboard events in the main event loop
- Update UI state before rendering
- Provide visual feedback for all user actions
- Handle terminal resize events gracefully

### Async Operations
- Use threads for CPU-bound operations (e.g., vanity generation)
- Provide progress updates for user feedback
- Implement cancellation for long-running tasks
- Clean up resources properly on completion or cancellation

## Solana Integration

- Use `solana-sdk` version 3.x API patterns
- Validate keypairs before use
- Handle network errors gracefully
- Cache RPC responses when appropriate
- Respect rate limits for RPC endpoints

## Prohibited Actions

- Do not commit private keys or sensitive data
- Do not disable security features
- Do not remove or skip existing tests without justification
- Do not introduce dependencies with known vulnerabilities
- Do not bypass error handling
- Do not use `unwrap()` or `expect()` in production code paths (prefer `?` operator and proper error handling)

## Questions or Issues

- Review existing issues in the GitHub repository
- Check documentation in the `docs/` directory and markdown files
- Refer to the architecture.md file for design decisions
- Consult CONTRIBUTING.md for contribution guidelines
