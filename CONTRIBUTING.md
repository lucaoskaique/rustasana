# Contributing to Rustasana

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR-USERNAME/rustasana.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- An Asana account with API access

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Style

We follow the standard Rust style guidelines. Please run:

```bash
cargo fmt
cargo clippy
```

Before submitting your pull request.

## Making Changes

1. Make your changes in your feature branch
2. Add tests if applicable
3. Ensure all tests pass
4. Update documentation if needed
5. Commit your changes with clear, descriptive commit messages

### Commit Message Guidelines

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Examples:
```
Add support for creating tasks

- Implement create_task API method
- Add 'create' command to CLI
- Update documentation
```

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Ensure your code follows the project's style guidelines
3. Make sure all tests pass
4. Submit a pull request with a clear description of the changes

### Pull Request Title Format

- `feat: Add new feature`
- `fix: Fix bug description`
- `docs: Update documentation`
- `refactor: Code refactoring`
- `test: Add or update tests`
- `chore: Maintenance tasks`

## Areas for Contribution

Here are some areas where contributions are especially welcome:

- **New Features**: See the roadmap in README.md
- **Bug Fixes**: Check the issues page
- **Documentation**: Improvements to README, code comments, or examples
- **Tests**: Increase test coverage
- **Performance**: Optimizations and improvements

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inspiring community for all.

### Our Standards

- Be respectful and inclusive
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

## Questions?

Feel free to open an issue with your question or reach out to the maintainers.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
