# Contributing to svmai-cli

Thank you for your interest in contributing to the svmai CLI tool! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

## How to Contribute

There are many ways to contribute to svmai-cli:

1. **Reporting Bugs**: If you find a bug, please create an issue with a detailed description.
2. **Suggesting Enhancements**: Have an idea for a new feature? Open an issue to discuss it.
3. **Code Contributions**: Submit pull requests for bug fixes or new features.
4. **Documentation**: Help improve or translate documentation.
5. **Testing**: Help test the application on different platforms.

## Development Setup

### Prerequisites

* Rust and Cargo (latest stable version)
* Git
* Build essentials (for your platform)
* OpenSSL development libraries

### Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/svmai-cli.git
   cd svmai-cli
   ```
3. Add the upstream repository as a remote:
   ```bash
   git remote add upstream https://github.com/openSVM/svmai-cli.git
   ```
4. Create a new branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
5. Build the project:
   ```bash
   cargo build
   ```
6. Run tests:
   ```bash
   cargo test
   ```

## Making Changes

1. Make your changes in your feature branch
2. Add tests for your changes
3. Run the test suite to ensure everything passes:
   ```bash
   cargo test
   ```
4. Format your code:
   ```bash
   cargo fmt
   ```
5. Run clippy to catch common mistakes:
   ```bash
   cargo clippy
   ```

## Submitting Changes

1. Commit your changes with a descriptive commit message:
   ```bash
   git commit -m "Add feature: description of your changes"
   ```
2. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
3. Submit a pull request to the main repository

## Pull Request Guidelines

* Include a clear description of the changes
* Link to any related issues
* Include tests for new functionality
* Ensure all tests pass
* Follow the project's code style
* Keep pull requests focused on a single topic

## Code Style

* Follow Rust's official style guide
* Use `cargo fmt` to format your code
* Use `cargo clippy` to catch common mistakes
* Write clear, descriptive comments
* Document public API functions with rustdoc comments

## Testing

* Write unit tests for all new functionality
* Ensure existing tests continue to pass
* Consider edge cases in your tests
* For UI changes, include manual testing steps

## Documentation

* Update documentation for any changed functionality
* Document new features thoroughly
* Use clear, concise language
* Include examples where appropriate

## Release Process

The project maintainers will handle the release process, which includes:

1. Updating the version number
2. Creating a changelog entry
3. Building and publishing releases
4. Announcing the new release

## Getting Help

If you need help with the contribution process, feel free to:

* Open an issue with your question
* Reach out to the maintainers

Thank you for contributing to svmai-cli!
