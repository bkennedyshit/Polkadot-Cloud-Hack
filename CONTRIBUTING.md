# Contributing to ReputeChain

Thank you for your interest in contributing to ReputeChain! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker (optional)

### Setup Development Environment

```bash
# Clone the repository
git clone https://github.com/yourusername/reputechain.git
cd reputechain

# Install Rust dependencies
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Install frontend dependencies
cd frontend
npm install
cd ..
```

### Running Tests

```bash
# Run all tests
make test

# Run specific tests
cargo test --package pallet-reputation

# Run frontend tests
cd frontend && npm test
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

Use descriptive branch names:
- `feature/add-dispute-resolution`
- `fix/rating-calculation-bug`
- `docs/update-readme`

### 2. Make Your Changes

Follow these guidelines:
- Write clean, idiomatic code
- Add tests for new functionality
- Update documentation
- Follow existing code style

### 3. Format and Lint

```bash
# Format code
make fmt

# Run linter
make lint

# Fix issues automatically
cargo fix --allow-dirty
cargo clippy --fix
```

### 4. Commit Your Changes

Use conventional commits:

```bash
git commit -m "feat: add dispute resolution mechanism"
git commit -m "fix: correct reputation calculation"
git commit -m "docs: update API documentation"
```

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub with:
- Clear description of changes
- Link to related issues
- Screenshots (if UI changes)
- Test results

## Contribution Areas

### Backend (Rust/Substrate)

- Pallet improvements
- Performance optimizations
- Security enhancements
- Test coverage

### Frontend (React/TypeScript)

- UI/UX improvements
- New components
- Performance optimization
- Accessibility

### Documentation

- README updates
- API documentation
- Architecture diagrams
- Tutorial guides

### Community

- Bug reports
- Feature requests
- Community support
- Marketing content

## Reporting Bugs

When reporting bugs, include:
1. Clear description of the issue
2. Steps to reproduce
3. Expected behavior
4. Actual behavior
5. Environment details
6. Screenshots/logs

## Feature Requests

For feature requests:
1. Describe the use case
2. Explain the benefit
3. Provide examples
4. Discuss implementation approach

## Pull Request Process

1. Ensure all tests pass
2. Update documentation
3. Add changelog entry
4. Request review from maintainers
5. Address feedback
6. Squash commits if requested
7. Merge when approved

## Code Style

### Rust
- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add doc comments for public APIs

### TypeScript/React
- Use TypeScript for type safety
- Follow ESLint rules
- Use functional components
- Add PropTypes or interfaces

### Documentation
- Use clear, concise language
- Include code examples
- Keep README up to date
- Document breaking changes

## Testing

### Rust Tests
```bash
#Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'

# Specific test
cargo test test_name
```

### Frontend Tests
```bash
cd frontend
npm test
npm run test:coverage
```

## Performance

- Profile code before optimizing
- Benchmark critical paths
- Document performance implications
- Consider blockchain gas costs

## Security

- Report security issues privately
- Don't commit secrets
- Validate all inputs
- Use secure dependencies

## Questions?

- Open an issue for discussion
- Join our Discord community
- Email: hello@reputechain.io

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Thank you for contributing to ReputeChain! 🚀**
