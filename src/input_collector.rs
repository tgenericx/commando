use crate::commit_message::{CommitMessage, ValidationError};
use crate::commit_types::CommitType;
use crate::compiler::compile::{compile, validate};
use crate::compiler::error::CompileError;
use std::io::{self, Write};

/// Interactive input collector for commit messages
///
/// Guides users through creating a valid commit message step-by-step.
/// Validates each field individually and provides clear feedback.
///
/// Now integrates with the compiler module for advanced validation and formatting.
#[derive(Default, Debug)]
pub struct InputCollector;

#[derive(Debug, Clone)]
pub struct CommitData {
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub description: String,
    pub body: Option<String>,
    pub breaking_change: Option<String>,
}

impl CommitData {
    pub fn to_commit_message(&self) -> Result<CommitMessage, ValidationError> {
        CommitMessage::new(
            self.commit_type,
            self.scope.clone(),
            self.description.clone(),
            self.body.clone(),
            self.breaking_change.clone(),
        )
    }

    /// Convert CommitData to raw commit message string for compiler validation
    pub fn to_raw_message(&self) -> String {
        let mut msg = String::new();

        // Type
        msg.push_str(self.commit_type.as_str());

        // Scope
        if let Some(ref scope) = self.scope {
            msg.push('(');
            msg.push_str(scope);
            msg.push(')');
        }

        // Breaking indicator
        if self.breaking_change.is_some() {
            msg.push('!');
        }

        // Description
        msg.push_str(": ");
        msg.push_str(&self.description);

        // Body
        if let Some(ref body) = self.body {
            msg.push_str("\n\n");
            msg.push_str(body);
        }

        // Breaking change footer
        if let Some(ref breaking) = self.breaking_change {
            msg.push_str("\n\nBREAKING CHANGE: ");
            msg.push_str(breaking);
        }

        msg
    }
}

impl InputCollector {
    pub fn new() -> Self {
        Self
    }

    /// Collect all commit message data from the user interactively
    ///
    /// Returns a validated CommitMessage or an error if the user cancels.
    pub fn collect(&self) -> Result<CommitMessage, String> {
        println!("\n=== Create Commit Message ===\n");

        // Step 1: Collect type
        let commit_type = self.collect_type()?;

        // Step 2: Collect scope (optional)
        let scope = self.collect_scope()?;

        // Step 3: Collect description
        let description = self.collect_description()?;

        // Step 4: Collect body (optional)
        let body = self.collect_body()?;

        // Step 5: Collect breaking change (optional)
        let breaking_change = self.collect_breaking_change()?;

        // Create commit message (should always succeed since we validated each field)
        CommitMessage::new(commit_type, scope, description, body, breaking_change)
            .map_err(|e| format!("Unexpected validation error: {}", e))
    }

    /// Validate a raw commit message string using the compiler
    ///
    /// This allows validation of commit messages from any source (file, clipboard, etc.)
    ///
    /// # Arguments
    /// * `input` - Raw commit message text
    ///
    /// # Returns
    /// * `Ok(())` - Message is valid
    /// * `Err(String)` - Validation error with user-friendly message
    ///
    /// # Examples
    /// ```
    /// let collector = InputCollector::new();
    ///
    /// // Valid message
    /// assert!(collector.validate_raw("feat: add feature").is_ok());
    ///
    /// // Invalid message
    /// assert!(collector.validate_raw("invalid: bad type").is_err());
    /// ```
    pub fn validate_raw(&self, input: &str) -> Result<(), String> {
        validate(input).map_err(|e| self.format_compile_error(e))
    }

    /// Compile and format a raw commit message string
    ///
    /// Validates the message and returns a professionally formatted version.
    /// Useful for cleaning up manually written commit messages.
    ///
    /// # Arguments
    /// * `input` - Raw commit message text
    ///
    /// # Returns
    /// * `Ok(String)` - Formatted commit message
    /// * `Err(String)` - Compilation error with user-friendly message
    ///
    /// # Examples
    /// ```
    /// let collector = InputCollector::new();
    ///
    /// let formatted = collector.compile_raw(
    ///     "feat(api): add endpoint\n\nThis is a very long body text that needs wrapping..."
    /// )?;
    /// // Returns properly formatted message with text wrapping
    /// ```
    pub fn compile_raw(&self, input: &str) -> Result<String, String> {
        compile(input).map_err(|e| self.format_compile_error(e))
    }

    /// Validate collected CommitData using the compiler pipeline
    ///
    /// Performs advanced validation including:
    /// - Breaking change consistency (header ! matches footer)
    /// - Scope case validation (lowercase only)
    /// - All semantic rules from SemanticAnalyzer
    ///
    /// # Arguments
    /// * `data` - CommitData to validate
    ///
    /// # Returns
    /// * `Ok(())` - Data is valid
    /// * `Err(String)` - Validation error
    pub fn validate_data(&self, data: &CommitData) -> Result<(), String> {
        let raw_message = data.to_raw_message();
        self.validate_raw(&raw_message)
    }

    /// Compile CommitData into a formatted commit message
    ///
    /// Applies professional formatting including:
    /// - Text wrapping at 72 characters
    /// - Footer ordering (BREAKING CHANGE first)
    /// - Proper whitespace handling
    ///
    /// # Arguments
    /// * `data` - CommitData to compile
    ///
    /// # Returns
    /// * `Ok(String)` - Formatted commit message
    /// * `Err(String)` - Compilation error
    pub fn compile_data(&self, data: &CommitData) -> Result<String, String> {
        let raw_message = data.to_raw_message();
        self.compile_raw(&raw_message)
    }

    /// Interactive mode: Collect from file and validate
    ///
    /// Allows users to paste a commit message from a file and validates it.
    /// Useful for pre-written commit messages.
    ///
    /// # Returns
    /// * `Ok(String)` - Validated and formatted commit message
    /// * `Err(String)` - Error during collection or validation
    pub fn collect_from_paste(&self) -> Result<String, String> {
        println!("\n=== Paste Commit Message ===\n");
        println!("Paste your commit message below.");
        println!("Press Ctrl+D when done (Ctrl+Z on Windows):");
        println!();

        let mut input = String::new();
        loop {
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break, // EOF
                Ok(_) => input.push_str(&line),
                Err(e) => return Err(format!("Failed to read input: {}", e)),
            }
        }

        let input = input.trim();
        if input.is_empty() {
            return Err("No input provided".to_string());
        }

        println!();
        println!("Validating...");

        // Validate and format
        match self.compile_raw(input) {
            Ok(formatted) => {
                println!("✓ Valid commit message!");
                println!();
                Ok(formatted)
            }
            Err(e) => {
                println!("✗ Invalid commit message:");
                println!("  {}", e);
                Err(e)
            }
        }
    }

    /// Re-collect a specific field for editing
    ///
    /// Returns updated CommitData with the new field value
    pub fn edit_field(&self, data: &CommitData, field: &str) -> Result<CommitData, String> {
        let mut new_data = data.clone();

        match field {
            "type" | "1" => {
                println!("\nEditing commit type...\n");
                new_data.commit_type = self.collect_type()?;
            }
            "scope" | "2" => {
                println!("\nEditing scope...\n");
                new_data.scope = self.collect_scope()?;
            }
            "description" | "desc" | "3" => {
                println!("\nEditing description...\n");
                new_data.description = self.collect_description()?;
            }
            "body" | "4" => {
                println!("\nEditing body...\n");
                new_data.body = self.collect_body()?;
            }
            "breaking" | "5" => {
                println!("\nEditing breaking change...\n");
                new_data.breaking_change = self.collect_breaking_change()?;
            }
            _ => {
                return Err(format!(
                    "Unknown field: '{}'. Valid fields: type, scope, description, body, breaking",
                    field
                ));
            }
        }

        // Validate the updated data using compiler
        if let Err(e) = self.validate_data(&new_data) {
            println!("\n⚠ Warning: Updated data has validation issues:");
            println!("  {}", e);
            println!("You may need to fix other fields to maintain consistency.");
        }

        Ok(new_data)
    }

    /// Format a CompileError into a user-friendly error message
    fn format_compile_error(&self, error: CompileError) -> String {
        match error {
            CompileError::LexerError(msg) => {
                format!("Syntax error: {}", msg)
            }
            CompileError::ParseError(err) => {
                format!("Parse error: {}", err)
            }
            CompileError::SemanticError(err) => {
                format!("Validation error: {}", err)
            }
        }
    }

    // ========================================================================
    // Individual field collectors (unchanged from original)
    // ========================================================================

    fn collect_type(&self) -> Result<CommitType, String> {
        println!("1. What type of change is this?");
        println!("   · feat      (new feature)");
        println!("   · fix       (bug fix)");
        println!("   · docs      (documentation)");
        println!("   · style     (formatting, missing semicolons, etc.)");
        println!("   · refactor  (code restructuring)");
        println!("   · perf      (performance improvement)");
        println!("   · test      (adding tests)");
        println!("   · build     (build system changes)");
        println!("   · ci        (CI configuration)");
        println!("   · chore     (maintenance)");
        println!("   · revert    (revert previous commit)");
        println!();

        loop {
            print!("Your choice: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim();

            match CommitType::from_str(input) {
                Ok(commit_type) => {
                    println!();
                    return Ok(commit_type);
                }
                Err(ValidationError::InvalidCommitType(_)) => {
                    println!(
                        "Error: '{}' is not a valid type. Please choose from the list above.",
                        input
                    );
                }
                Err(e) => return Err(e.to_string()),
            }
        }
    }

    fn collect_scope(&self) -> Result<Option<String>, String> {
        println!("2. Which module/area does this affect?");
        println!("   (e.g., api, parser, auth-service)");
        println!("   Must be lowercase, alphanumeric with hyphens/underscores");
        println!("   Press Enter to skip");
        println!();

        loop {
            print!("Scope: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim();

            if input.is_empty() {
                println!();
                return Ok(None);
            }

            // Validate scope - now enforces lowercase
            if input.chars().any(|c| c.is_uppercase()) {
                println!("Error: Scope must be lowercase only.");
                println!("Try again or press Enter to skip.");
                continue;
            }

            if CommitMessage::validate_scope(input).is_ok() {
                println!();
                return Ok(Some(input.to_string()));
            } else {
                println!("Error: Scope must be alphanumeric with hyphens/underscores only.");
                println!("Try again or press Enter to skip.");
            }
        }
    }

    fn collect_description(&self) -> Result<String, String> {
        println!("3. What does this change do?");
        println!("   (Brief description, max 72 characters)");
        println!();

        loop {
            print!("Description: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim().to_string();

            if input.is_empty() {
                println!("Error: Description cannot be empty.");
                continue;
            }

            if input.len() > 72 {
                println!(
                    "Error: Description is too long ({} chars). Maximum is 72 characters.",
                    input.len()
                );
                println!("Please shorten your description.");
                continue;
            }

            println!();
            return Ok(input);
        }
    }

    fn collect_body(&self) -> Result<Option<String>, String> {
        println!("4. Would you like to add more details? (y/N)");
        print!("Choice: ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .map_err(|e| e.to_string())?;
        let choice = choice.trim().to_lowercase();

        if choice != "y" && choice != "yes" {
            println!();
            return Ok(None);
        }

        println!();
        println!("Enter detailed description (press Ctrl+D when done, Ctrl+C to cancel):");
        println!("Note: Text will be wrapped at 72 characters when formatted.");
        println!();

        let mut body = String::new();
        loop {
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(0) => break, // EOF (Ctrl+D)
                Ok(_) => body.push_str(&line),
                Err(e) => return Err(e.to_string()),
            }
        }

        let body = body.trim().to_string();
        if body.is_empty() {
            println!();
            return Ok(None);
        }

        println!();
        Ok(Some(body))
    }

    fn collect_breaking_change(&self) -> Result<Option<String>, String> {
        println!("5. Does this change break existing functionality? (y/N)");
        print!("Choice: ");
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .map_err(|e| e.to_string())?;
        let choice = choice.trim().to_lowercase();

        if choice != "y" && choice != "yes" {
            println!();
            return Ok(None);
        }

        println!();
        println!("Describe what breaks and how users should adapt:");
        println!("Note: This will add both a '!' to the header and a BREAKING CHANGE footer.");
        println!();

        loop {
            print!("Breaking change: ");
            io::stdout().flush().map_err(|e| e.to_string())?;

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .map_err(|e| e.to_string())?;
            let input = input.trim().to_string();

            if input.is_empty() {
                println!("Error: Breaking change description cannot be empty.");
                println!("Press Ctrl+C to cancel or provide a description.");
                continue;
            }

            println!();
            return Ok(Some(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_collector_can_be_created() {
        let _collector = InputCollector::new();
    }

    #[test]
    fn input_collector_has_default() {
        let _collector = InputCollector::default();
    }

    #[test]
    fn commit_data_to_raw_message_simple() {
        let data = CommitData {
            commit_type: CommitType::Feat,
            scope: None,
            description: "add feature".to_string(),
            body: None,
            breaking_change: None,
        };

        let raw = data.to_raw_message();
        assert_eq!(raw, "feat: add feature");
    }

    #[test]
    fn commit_data_to_raw_message_with_scope() {
        let data = CommitData {
            commit_type: CommitType::Fix,
            scope: Some("api".to_string()),
            description: "fix bug".to_string(),
            body: None,
            breaking_change: None,
        };

        let raw = data.to_raw_message();
        assert_eq!(raw, "fix(api): fix bug");
    }

    #[test]
    fn commit_data_to_raw_message_with_breaking() {
        let data = CommitData {
            commit_type: CommitType::Feat,
            scope: Some("auth".to_string()),
            description: "change auth".to_string(),
            body: None,
            breaking_change: Some("Removes old auth".to_string()),
        };

        let raw = data.to_raw_message();
        assert_eq!(
            raw,
            "feat(auth)!: change auth\n\nBREAKING CHANGE: Removes old auth"
        );
    }

    #[test]
    fn commit_data_to_raw_message_full() {
        let data = CommitData {
            commit_type: CommitType::Feat,
            scope: Some("api".to_string()),
            description: "add endpoint".to_string(),
            body: Some("Detailed explanation here".to_string()),
            breaking_change: Some("API changed".to_string()),
        };

        let raw = data.to_raw_message();
        assert!(raw.contains("feat(api)!:"));
        assert!(raw.contains("add endpoint"));
        assert!(raw.contains("Detailed explanation here"));
        assert!(raw.contains("BREAKING CHANGE: API changed"));
    }

    #[test]
    fn validate_raw_accepts_valid_message() {
        let collector = InputCollector::new();
        let result = collector.validate_raw("feat: add feature");
        assert!(result.is_ok());
    }

    #[test]
    fn validate_raw_rejects_invalid_type() {
        let collector = InputCollector::new();
        let result = collector.validate_raw("invalid: bad type");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_lowercase()
                .contains("invalid commit type")
        );
    }

    #[test]
    fn validate_raw_rejects_uppercase_scope() {
        let collector = InputCollector::new();
        let result = collector.validate_raw("feat(API): add feature");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_lowercase().contains("scope"));
    }

    #[test]
    fn validate_raw_rejects_long_description() {
        let collector = InputCollector::new();
        let long_desc = "a".repeat(80);
        let msg = format!("feat: {}", long_desc);
        let result = collector.validate_raw(&msg);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too long"));
    }

    #[test]
    fn compile_raw_formats_message() {
        let collector = InputCollector::new();
        let result = collector.compile_raw("feat: add feature");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat: add feature");
    }

    #[test]
    fn validate_data_checks_breaking_consistency() {
        let collector = InputCollector::new();

        // Valid: both header ! and footer present
        let valid_data = CommitData {
            commit_type: CommitType::Feat,
            scope: None,
            description: "change".to_string(),
            body: None,
            breaking_change: Some("Changed".to_string()),
        };
        assert!(collector.validate_data(&valid_data).is_ok());
    }

    #[test]
    fn compile_data_produces_formatted_output() {
        let collector = InputCollector::new();
        let data = CommitData {
            commit_type: CommitType::Feat,
            scope: Some("api".to_string()),
            description: "add endpoint".to_string(),
            body: None,
            breaking_change: None,
        };

        let result = collector.compile_data(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "feat(api): add endpoint");
    }
}
