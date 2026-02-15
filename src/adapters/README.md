# Adapters

This directory contains adapter implementations that connect the application core to external systems and infrastructure.

## Overview

Adapters are part of the Hexagonal Architecture pattern, implementing the port interfaces defined in `src/ports/`. They handle all external communication and infrastructure concerns.

## Structure

```
adapters/
├── git/            # Git repository operations
│   ├── executor.rs # Commit execution
│   ├── staging.rs  # Staging area queries
│   ├── error.rs    # Git-specific errors
│   └── mod.rs      # Module exports
├── ui/             # User interface
│   ├── terminal.rs # Terminal-based UI
│   └── mod.rs      # Module exports
└── mod.rs          # Adapter exports
```

## Git Adapter

### Purpose
Provides concrete implementation of Git operations using the `git2` crate.

### Components

#### Executor (`git/executor.rs`)
```rust
pub struct GitExecutor {
    repo: Repository,
}

impl Executor for GitExecutor {
    fn commit(&self, message: &str) -> Result<Oid>;
    fn amend(&self, message: &str) -> Result<Oid>;
}
```

**Responsibilities:**
- Create new commits
- Amend existing commits
- Handle Git signatures
- Manage references (HEAD)

**Example Usage:**
```rust
use commando::adapters::git::GitExecutor;
use commando::ports::Executor;

let executor = GitExecutor::open(".")?;
let commit_id = executor.commit("feat: add feature")?;
println!("Created commit: {}", commit_id);
```

#### Staging (`git/staging.rs`)
```rust
pub struct GitStaging {
    repo: Repository,
}

impl Staging for GitStaging {
    fn has_staged_changes(&self) -> Result<bool>;
    fn get_diff(&self) -> Result<String>;
    fn stage_all(&self) -> Result<()>;
}
```

**Responsibilities:**
- Query staging area status
- Retrieve diff information
- Count staged files
- Stage/unstage files

**Example Usage:**
```rust
use commando::adapters::git::GitStaging;
use commando::ports::Staging;

let staging = GitStaging::open(".")?;
if staging.has_staged_changes()? {
    let diff = staging.get_diff()?;
    println!("Changes:\n{}", diff);
}
```

#### Error Handling (`git/error.rs`)
```rust
#[derive(Debug, thiserror::Error)]
pub enum GitError {
    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),
    
    #[error("No staged changes")]
    NoStagedChanges,
    
    #[error("Git configuration error: {0}")]
    ConfigError(String),
    
    #[error("Git operation failed: {0}")]
    OperationFailed(String),
}
```

**Error Types:**
- `RepositoryNotFound` - Not in a Git repository
- `NoStagedChanges` - Staging area is empty
- `ConfigError` - Git configuration issues
- `OperationFailed` - General Git operation errors

### Implementation Details

#### Repository Detection
```rust
impl GitExecutor {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)
            .map_err(|e| GitError::RepositoryNotFound(e.to_string()))?;
        Ok(Self { repo })
    }
}
```

The adapter uses `Repository::discover()` which walks up the directory tree to find the `.git` directory.

#### Commit Creation
```rust
pub fn commit(&self, message: &str) -> Result<Oid> {
    // Get signature from Git config
    let signature = self.repo.signature()?;
    
    // Get current index
    let mut index = self.repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = self.repo.find_tree(tree_id)?;
    
    // Get parent commit (if exists)
    let parent = match self.repo.head() {
        Ok(head) => Some(head.peel_to_commit()?),
        Err(_) => None,
    };
    
    // Create commit
    let commit_id = self.repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &parent.iter().collect::<Vec<_>>(),
    )?;
    
    Ok(commit_id)
}
```

## UI Adapter

### Purpose
Provides terminal-based user interface using ANSI escape codes and terminal I/O.

### Components

#### Terminal (`ui/terminal.rs`)
```rust
pub struct TerminalUI {
    stdout: io::Stdout,
    stderr: io::Stderr,
}

impl UI for TerminalUI {
    fn display_message(&self, message: &str);
    fn display_error(&self, error: &str);
    fn display_success(&self, message: &str);
    fn confirm(&self, prompt: &str) -> Result<bool>;
}
```

**Features:**
- Colored output (success = green, error = red, info = blue)
- Unicode support
- Progress indicators
- Confirmation prompts
- Multi-line formatting

**Example Usage:**
```rust
use commando::adapters::ui::TerminalUI;
use commando::ports::UI;

let ui = TerminalUI::new();
ui.display_message("Starting commit process...");
ui.display_success("Commit created successfully!");

if ui.confirm("Continue?")? {
    // Proceed
}
```

#### Color Support
```rust
pub struct TerminalUI {
    colors_enabled: bool,
}

impl TerminalUI {
    fn colorize(&self, text: &str, color: Color) -> String {
        if self.colors_enabled {
            format!("\x1b[{}m{}\x1b[0m", color.code(), text)
        } else {
            text.to_string()
        }
    }
}
```

The adapter detects terminal capabilities and disables colors for:
- Non-TTY output (pipes, redirects)
- `NO_COLOR` environment variable set
- Windows terminals without ANSI support

## Testing

### Unit Tests
Each adapter has comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_executor_create_commit() {
        let temp = TempDir::new().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        
        let executor = GitExecutor::open(temp.path()).unwrap();
        let result = executor.commit("test: initial commit");
        
        assert!(result.is_ok());
    }
}
```

### Integration Tests
Full end-to-end tests with real Git operations:

```rust
#[test]
fn test_commit_workflow() {
    // Setup repository
    let temp = setup_test_repo();
    
    // Create file and stage
    create_file(&temp, "test.txt", "content");
    stage_file(&temp, "test.txt");
    
    // Execute commit
    let executor = GitExecutor::open(&temp).unwrap();
    let commit_id = executor.commit("test: add file").unwrap();
    
    // Verify commit
    assert!(commit_exists(&temp, commit_id));
}
```

### Mock Adapters
For testing application logic without real Git:

```rust
pub struct MockExecutor {
    commits: RefCell<Vec<String>>,
}

impl Executor for MockExecutor {
    fn commit(&self, message: &str) -> Result<Oid> {
        self.commits.borrow_mut().push(message.to_string());
        Ok(Oid::zero())
    }
}
```

## Design Decisions

### Why git2 crate?
- **Mature**: Battle-tested Git library
- **Cross-platform**: Works on Linux, macOS, Windows
- **Safe**: Rust bindings with memory safety
- **Complete**: Supports all Git operations we need

### Why Not Call Git Binary?
- **Performance**: No process spawning overhead
- **Reliability**: No parsing Git output
- **Portability**: No Git installation required
- **Testing**: Easier to mock and test

### Error Handling Strategy
```rust
// Convert git2 errors to domain errors
impl From<git2::Error> for GitError {
    fn from(error: git2::Error) -> Self {
        match error.class() {
            git2::ErrorClass::Config => GitError::ConfigError(error.message().to_string()),
            git2::ErrorClass::Repository => GitError::RepositoryNotFound(error.message().to_string()),
            _ => GitError::OperationFailed(error.message().to_string()),
        }
    }
}
```

This provides:
- Clear error messages
- Type-safe error handling
- Easy error recovery
- Consistent error format

## Best Practices

### 1. Always Use Ports
```rust
// Good - depends on port
pub struct App<E: Executor> {
    executor: E,
}

// Bad - depends on concrete type
pub struct App {
    executor: GitExecutor,
}
```

### 2. Handle All Errors
```rust
// Good - handles error
let result = executor.commit(message)
    .map_err(|e| handle_commit_error(e))?;

// Bad - unwraps
let result = executor.commit(message).unwrap();
```

### 3. Resource Cleanup
```rust
impl Drop for GitExecutor {
    fn drop(&mut self) {
        // Cleanup resources
    }
}
```

### 4. Thread Safety
```rust
// Use Arc for shared state
pub struct GitExecutor {
    repo: Arc<Repository>,
}
```

## Performance Considerations

### Caching
```rust
pub struct GitExecutor {
    repo: Repository,
    signature_cache: Option<Signature<'static>>,
}

impl GitExecutor {
    fn get_signature(&mut self) -> Result<&Signature<'static>> {
        if self.signature_cache.is_none() {
            let sig = self.repo.signature()?;
            self.signature_cache = Some(sig);
        }
        Ok(self.signature_cache.as_ref().unwrap())
    }
}
```

### Lazy Loading
```rust
pub struct GitStaging {
    repo: Repository,
    diff: OnceCell<String>,
}

impl GitStaging {
    fn get_diff(&self) -> Result<&str> {
        self.diff.get_or_try_init(|| self.compute_diff())
    }
}
```

## Future Enhancements

### Potential Additions
1. **Progress Callbacks**: Long operations show progress
2. **Concurrent Operations**: Parallel file processing
3. **Hooks Support**: Run pre-commit hooks
4. **Submodule Support**: Handle Git submodules
5. **LFS Support**: Large File Storage integration

### Extensibility
```rust
// Future: Support other VCS
pub trait VcsExecutor {
    fn commit(&self, message: &str) -> Result<String>;
}

pub struct GitExecutor { /* ... */ }
pub struct MercurialExecutor { /* ... */ }

impl VcsExecutor for GitExecutor { /* ... */ }
impl VcsExecutor for MercurialExecutor { /* ... */ }
```

## Related Documentation

- [Ports](../ports/README.md) - Interface definitions
- [Architecture](../../docs/ARCHITECTURE.md) - Overall design
- [Testing Guide](../../docs/DEVELOPMENT.md#testing) - Testing practices

## Contributing

When adding new adapters:

1. Implement the corresponding port trait
2. Add comprehensive error handling
3. Write unit and integration tests
4. Document public APIs
5. Update this README

