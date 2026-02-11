use std::collections::HashSet;

use crate::commit_message::ValidationError;
use crate::compiler::ast::{CommitAst, FooterNode};
use crate::compiler::error::CompileError;

/// Semantic analyzer for commit ASTs
///
/// Validates that the commit follows conventional commit rules.
/// Does not modify the AST, only checks for correctness.
pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    /// Entry point
    pub fn analyze(ast: &CommitAst) -> Result<(), CompileError> {
        Self::validate_type(&ast.header.type_name)?;
        Self::validate_description(&ast.header.description)?;

        if let Some(scope) = &ast.header.scope {
            Self::validate_scope(scope)?;
        }

        Self::validate_footers(&ast.footers)?;
        Self::validate_breaking_changes(ast)?;

        Ok(())
    }

    /// Allowed commit types
    fn allowed_types() -> &'static [&'static str] {
        &[
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore",
        ]
    }

    fn validate_type(type_name: &str) -> Result<(), CompileError> {
        if !Self::allowed_types().contains(&type_name) {
            return Err(CompileError::SemanticError(
                ValidationError::InvalidCommitType(type_name.to_string()),
            ));
        }
        Ok(())
    }

    fn validate_description(desc: &str) -> Result<(), CompileError> {
        let trimmed = desc.trim();

        if trimmed.is_empty() {
            return Err(CompileError::SemanticError(
                ValidationError::EmptyDescription,
            ));
        }

        if trimmed.len() > 72 {
            return Err(CompileError::SemanticError(
                ValidationError::DescriptionTooLong(trimmed.len()),
            ));
        }

        Ok(())
    }

    fn validate_scope(scope: &str) -> Result<(), CompileError> {
        if scope.trim().is_empty() {
            return Err(CompileError::SemanticError(ValidationError::InvalidScope(
                scope.to_string(),
            )));
        }

        // Optional: enforce lowercase scopes
        if scope.chars().any(|c| c.is_uppercase()) {
            return Err(CompileError::SemanticError(ValidationError::InvalidScope(
                scope.to_string(),
            )));
        }

        Ok(())
    }

    /// Validate all footers together
    fn validate_footers(footers: &[FooterNode]) -> Result<(), CompileError> {
        let mut seen_keys = HashSet::new();

        for footer in footers {
            Self::validate_footer(footer)?;

            // Detect duplicate footer keys
            if !seen_keys.insert(footer.key.clone()) {
                return Err(CompileError::SemanticError(
                    ValidationError::DuplicateFooter(footer.key.clone()),
                ));
            }

            // Validate issue references (optional rule)
            if Self::is_issue_footer(&footer.key) {
                Self::validate_issue_reference(&footer.value)?;
            }
        }

        Ok(())
    }

    fn validate_footer(footer: &FooterNode) -> Result<(), CompileError> {
        if footer.key.trim().is_empty() || footer.value.trim().is_empty() {
            return Err(CompileError::SemanticError(ValidationError::InvalidFooter(
                format!("{}: {}", footer.key, footer.value),
            )));
        }

        Ok(())
    }

    fn validate_breaking_changes(ast: &CommitAst) -> Result<(), CompileError> {
        let breaking_footer = ast
            .footers
            .iter()
            .find(|f| f.key == "BREAKING CHANGE" || f.key == "BREAKING-CHANGE");

        // If header has !, footer must exist
        if ast.header.breaking && breaking_footer.is_none() {
            return Err(CompileError::SemanticError(
                ValidationError::EmptyBreakingChange,
            ));
        }

        // If footer exists, header must have !
        if breaking_footer.is_some() && !ast.header.breaking {
            return Err(CompileError::SemanticError(
                ValidationError::BreakingChangeMismatch,
            ));
        }

        // Ensure breaking footer has content
        if let Some(footer) = breaking_footer {
            if footer.value.trim().is_empty() {
                return Err(CompileError::SemanticError(
                    ValidationError::EmptyBreakingChange,
                ));
            }
        }

        Ok(())
    }

    fn is_issue_footer(key: &str) -> bool {
        matches!(key, "Refs" | "Closes" | "Fixes")
    }

    fn validate_issue_reference(value: &str) -> Result<(), CompileError> {
        // Basic rule: must contain #number
        if !value.contains('#') {
            return Err(CompileError::SemanticError(
                ValidationError::InvalidIssueReference(value.to_string()),
            ));
        }

        Ok(())
    }
}
