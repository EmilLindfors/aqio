# Contributing to Aqio

Thank you for your interest in contributing to Aqio! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites
- Rust 1.75 or later
- SQLx CLI: `cargo install sqlx-cli`
- Git

### Development Setup
```bash
# Clone the repository
git clone https://github.com/EmilLindfors/aqio.git
cd aqio

# Setup database
sqlx database create
sqlx migrate run --source aqio-database/migrations

# Run tests
cargo test --workspace

# Start development server
cargo run --bin aqio-api
```

## 📋 How to Contribute

### Reporting Bugs
- Use our [bug report template](.github/ISSUE_TEMPLATE/bug_report.md)
- Include steps to reproduce, expected behavior, and your environment
- Check existing issues first to avoid duplicates

### Suggesting Features
- Use our [feature request template](.github/ISSUE_TEMPLATE/feature_request.md)
- See our [roadmap issues](https://github.com/EmilLindfors/aqio/issues) for planned features
- Discuss major changes in issues before starting work

### Code Contributions

#### Pull Request Process
1. **Fork** the repository
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes** with clear, focused commits
4. **Add tests** for new functionality
5. **Ensure all tests pass**: `cargo test --workspace`
6. **Format code**: `cargo fmt --all`
7. **Run clippy**: `cargo clippy --all-targets --all-features`
8. **Submit a pull request** with a clear description

#### Code Standards
- Follow Rust naming conventions and idioms
- Write comprehensive tests for new features
- Include documentation for public APIs
- Keep functions focused and modules well-organized
- Use meaningful commit messages

#### Database Changes
- Always create migrations for schema changes
- Test migrations both up and down
- Document any breaking changes
- Consider backwards compatibility

## 🏗️ Architecture Guidelines

### Project Structure
- `aqio-core/` - Shared models and business logic
- `aqio-api/` - REST API server (Axum)
- `aqio-database/` - Database layer and migrations
- `aqio-frontend/` - Web interface (Dioxus)

### Database Design Principles
- Use descriptive table and column names
- Include proper foreign key constraints
- Add indexes for performance-critical queries
- Follow normalization best practices
- Include audit fields (created_at, updated_at)

### API Design
- Follow RESTful conventions
- Use proper HTTP status codes
- Include comprehensive error handling
- Document endpoints with examples
- Validate input data thoroughly

## 🧪 Testing

### Test Categories
- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test API endpoints and database operations
- **Migration tests**: Verify database schema changes
- **Performance tests**: Ensure scalability requirements

### Running Tests
```bash
# Run all tests
cargo test --workspace

# Run specific test suite
cargo test --package aqio-api
cargo test --package aqio-database

# Run with coverage
cargo tarpaulin --workspace
```

## 📦 Release Process

### Versioning
We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Release Checklist
- [ ] All tests pass
- [ ] Documentation updated
- [ ] Migration scripts tested
- [ ] Version numbers bumped
- [ ] Changelog updated
- [ ] Git tag created

## 🎯 Focus Areas

We're particularly interested in contributions to:

### High Priority
- Payment system integration (Stripe, Vipps)
- Role-based access control
- Email template system improvements
- API rate limiting and security

### Medium Priority
- Recurring events functionality
- Venue management system
- Third-party integrations (Slack, Google Calendar)
- Mobile-responsive frontend improvements

### Documentation
- API documentation
- Deployment guides
- User tutorials
- Developer onboarding

## 💬 Communication

### Getting Help
- Create an [issue](https://github.com/EmilLindfors/aqio/issues) for questions
- Check existing issues and discussions
- Be specific about your environment and problem

### Code Reviews
- Be constructive and respectful
- Focus on code quality, not personal preferences
- Explain the reasoning behind suggestions
- Acknowledge good practices and improvements

## 📄 License

By contributing to Aqio, you agree that your contributions will be licensed under the same [MIT License](LICENSE) that covers the project.

## 🙏 Recognition

Contributors will be recognized in:
- Release notes for significant contributions
- README acknowledgments
- GitHub contributor statistics

Thank you for helping make Aqio better! 🎉