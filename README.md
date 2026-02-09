# Commando

A terminal tool for creating **structured, conventional Git commits**
through an interactive, safe, and user-friendly workflow.

Commando guides you step by step, validates your input, lets you preview
and edit before committing, and guarantees you never create an invalid
commit message.

---

## ✨ Features

- **Interactive Workflow** – Guided prompts for every commit field  
- **Preview & Edit** – Review and fix any field before committing  
- **Type-Safe Domain Model** – Invalid commits are impossible  
- **Conventional Commits** – Fully compliant with the specification  
- **Git-Native Execution** – Runs real `git commit` commands  
- **Zero Dependencies** – Pure Rust, standard library only  

---

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/tgenericx/commando.git
cd commando
cargo install --path .
````

### Usage

```bash
# Stage your changes
git add <files>

# Run Commando
commando
```

Or without installing:

```bash
cargo run
```

---

## 🧭 Example

```text
Checking for staged changes...
✓ Staged changes detected

=== Create Commit Message ===

Type: feat
Scope: auth
Description: implement OAuth 2.0 authentication

=== Preview ===

feat(auth)!: implement OAuth 2.0 authentication

Choice (y/e/n): y

✓ Commit created successfully!
SHA: abc1234
```

---

## 📝 Commit Message Format

Commando follows the **Conventional Commits** format:

```text
<type>[optional scope][!]: <description>

[optional body]

[optional footer(s)]
```

### Examples

```text
feat: add user authentication
fix(parser): handle tokenizer edge case
feat(api)!: redesign authentication endpoints
```

---

## ✏️ Edit Before Commit

After previewing your commit message, you can:

- **Proceed** with the commit
- **Edit** any field (type, scope, description, body, breaking change)
- **Abort** safely without side effects

All valid input is preserved while editing.

---

## 🧠 Architecture (High-Level)

Commando uses a clean, layered design:

- **CliController** – Orchestrates the workflow
- **InputCollector** – Handles interactive input
- **CommitData** – Mutable intermediate state
- **CommitMessage** – Immutable, validated domain model
- **CommitExecutor** – Executes Git commands

📖 Full diagrams and internals:  
→ [`docs/architecture.md`](docs/architecture.md)

---

## 🧪 Development

```bash
cargo build
cargo test
cargo clippy
cargo fmt
```

**Requirements**

- Rust 1.70+
- Git

---

## 🛣 Roadmap

- Configuration file support
- Custom commit types
- Commit linting for CI
- Git hooks integration
- TUI interface

📋 Full roadmap:  
→ [`docs/roadmap.md`](docs/roadmap.md)

---

## ❓ FAQ

**Is Commando required for Git?**  
No. It’s an optional wrapper around `git commit`.

**Does it work with existing hooks and workflows?**  
Yes. Commando executes native Git commands.

**Can I skip Commando for quick commits?**  
Absolutely. Use `git commit` as usual.

---
