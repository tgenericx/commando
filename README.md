# Commando

A Git commit message tool that enforces structured, conventional commit messages for individuals and teams.

**⚠️ Project Status: Early Development ⚠️**

This project is currently in its initial development phase. The core CLI structure is being implemented, but most functionality is not yet available.

## Current Features

- Basic CLI controller structure
- Exit code handling
- Project scaffolding with Rust

## Roadmap

- [x] Set up project structure
- [x] Create basic CLI controller
- [ ] Implement staging area checking
- [ ] Add commit message validation
- [ ] Create interactive commit message builder
- [ ] Add configuration system
- [ ] Implement Git hook integration

## Development Setup

```bash
# Clone the repository
git clone <repository-url>

# Build the project
cargo build

# Run tests
cargo test

# Run the CLI (currently minimal functionality)
cargo run
```

Contributing

Contributions are welcome! Please ensure:

· All tests pass
· Code follows established patterns
· Documentation is updated
· Validation rules are respected

Note: Since this is early-stage development, please check existing issues or discussions before implementing major features.
