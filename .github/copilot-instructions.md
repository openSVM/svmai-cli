# Copilot Instructions for svmai-cli

## Project Overview

`svmai` is a command-line interface (CLI) tool built with Rust for managing Solana wallets. It provides secure wallet management with system keychain integration, multi-threaded wallet search, vanity wallet generation, and a comprehensive text-based user interface (TUI) built with `ratatui`.

## Key Features

- Multi-threaded wallet search and validation
- Secure keychain storage using AES-GCM encryption
- Text-based User Interface (TUI) for wallet management
- Vanity wallet generation with "ai" prefix
- Batch operations for token transfers
- Token mixing simulation for privacy enhancement
- SOL and SPL token balance viewing

## Project Structure

```
src/
├── main.rs                  # Application entry point and CLI parsing
├── tui.rs                   # Text-based User Interface implementation
├── file_searcher.rs         # Multi-threaded wallet file search
├── key_validator.rs         # Solana private key validation
├── secure_storage.rs        # Keychain integration and encryption
├── wallet_manager.rs        # Wallet CRUD operations
├── vanity_wallet.rs         # Vanity address generation
├── config.rs                # Configuration management (not currently integrated)
├── logging.rs               # Logging utilities (not currently integrated)
└── transaction_handler.rs   # Batch send and token mixing logic (not currently integrated)
```

Note: Some modules (`config.rs`, `logging.rs`, `transaction_handler.rs`) exist as files but are not yet imported in `main.rs`, indicating they may be planned features or incomplete integrations.

## Build Process

### Prerequisites

- Rust and Cargo (latest stable version)
- Build essentials for your platform
- OpenSSL development libraries
  - Ubuntu/Debian: `sudo apt-get install libssl-dev pkg-config build-essential`
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

The compiled binary will be located at:
- Debug: `target/debug/svmai`
- Release: `target/release/svmai`

## Testing

Run the test suite with:

```bash
cargo test
```

Run tests with verbose output:

```bash
cargo test -- --nocapture
```

Run tests with coverage (CI uses cargo-tarpaulin):

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --verbose --workspace --timeout 120
```

## Linting and Code Quality

### Format Code

Always format code before committing:

```bash
cargo fmt
```

Check formatting without making changes:

```bash
cargo fmt -- --check
```

### Clippy (Lint)

Run Clippy to catch common mistakes and improve code quality:

```bash
cargo clippy
```

Run Clippy with warnings as errors (used in CI):

```bash
cargo clippy -- -D warnings
```

## Code Style Guidelines

- Follow Rust's official style guide
- Use `cargo fmt` to automatically format code
- Use `cargo clippy` to catch common mistakes and anti-patterns
- Write clear, descriptive comments for complex logic
- Document public API functions with rustdoc comments (`///`)
- Keep functions focused and modular
- Use meaningful variable and function names

### Naming Conventions

- Use `snake_case` for functions, variables, and module names
- Use `PascalCase` for types, traits, and enum variants
- Use `SCREAMING_SNAKE_CASE` for constants and statics
- Prefix unused variables with underscore: `_variable`

### Error Handling

- Use `Result<T, E>` for fallible operations
- Propagate errors with `?` operator where appropriate
- Use `anyhow` for flexible error handling in application code
- Provide user-friendly error messages in the TUI

## Dependencies

Key dependencies:

- `ratatui` & `crossterm` - Text-based User Interface
- `rayon` - Parallel processing for file search and vanity generation
- `solana-sdk` - Solana blockchain interactions
- `keyring` - System keychain integration
- `aes-gcm` - AES-GCM encryption for private keys
- `serde` & `serde_json` - Serialization/deserialization
- `walkdir` - Recursive directory traversal

## Security Considerations

**CRITICAL**: This application handles Solana private keys. Follow these security practices:

1. **Private Key Handling**:
   - Keep decrypted private keys in memory for the shortest time necessary
   - Never log or display private keys
   - Use secure_storage module for all key operations

2. **Encryption**:
   - All private keys must be encrypted using AES-GCM before storage
   - Master encryption key is stored in system keychain

3. **Input Validation**:
   - Rigorously validate all user inputs (paths, addresses, amounts)
   - Validate Solana addresses before use
   - Sanitize file paths to prevent directory traversal

4. **Testing with Secrets**:
   - Never commit test private keys or sensitive data
   - Use generated/temporary keys for tests
   - Mark sensitive test data clearly

## CI/CD Workflow

The project uses GitHub Actions with the following checks:

1. **Build** - Compiles the project on Ubuntu
2. **Test** - Runs the full test suite
3. **Coverage** - Generates code coverage report (uploaded to Codecov)
4. **Clippy** - Runs linting checks (fails on warnings)
5. **Format** - Checks code formatting compliance

All checks must pass before merging to master branch.

## Contributing Guidelines

When making changes:

1. Create a feature branch from `master`
2. Make focused, atomic commits with descriptive messages
3. Add tests for new functionality
4. Run `cargo fmt` before committing
5. Run `cargo clippy` and address all warnings
6. Run `cargo test` to ensure all tests pass
7. Update documentation for any changed functionality
8. Keep pull requests focused on a single topic

## Common Development Tasks

### Adding a New Feature

1. Determine which module(s) need modification
2. Add tests first (TDD approach when possible)
3. Implement the feature
4. Update documentation
5. Run linting and tests

### Debugging

- Use `RUST_LOG=debug cargo run` for detailed logging
- The TUI can be challenging to debug; consider adding log statements
- Use `cargo test -- --nocapture` to see test output

### Performance Considerations

- File search is optimized for large directories using `rayon`
- Vanity wallet generation uses multiple CPU cores (configurable)
- API calls are rate-limited to avoid service disruptions
- Consider caching for frequently accessed data

## TUI Development

The TUI is built with `ratatui` and `crossterm`. Key concepts:

- **Event Loop**: Main loop handles drawing and input events
- **View System**: Organized into different views (WalletList, WalletDetail, Help, etc.)
- **App State**: The `App` struct maintains application state
- **Responsive Layout**: UI adapts to terminal size using constraints

When modifying the TUI:
- Test in different terminal sizes
- Ensure keyboard shortcuts don't conflict
- Update help screen (`H` key) with new shortcuts
- Handle edge cases (empty lists, long text, etc.)

## Known Issues

- **Compilation Error**: The project currently has a compilation error in `wallet_manager.rs` at line 162. The code uses `Keypair::from_bytes(&key_bytes)` which does not exist in solana-sdk 3.0.0. The available methods are:
  - `Keypair::new()` - Creates a new random keypair
  - `Keypair::new_from_array([u8; 32])` - Creates from a 32-byte seed (not 64-byte keypair)
  - `Keypair::from_base58_string(&str)` - Creates from base58 string
  
  The keypair bytes need to be split: first 32 bytes are the secret key seed, which should be used with `new_from_array()`.

- **Deprecation Warning**: The TUI uses `frame.size()` at line 539 in `src/tui.rs`. While this currently works, ratatui has marked it as deprecated in favor of `frame.area()`. This is a minor issue that only generates a warning.

When addressing issues:
- Check if they're already known and documented
- Consider backward compatibility
- Update tests to reflect changes
- Document breaking changes in CHANGELOG.md
