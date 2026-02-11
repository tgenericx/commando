use std::collections::HashSet;

use crate::compiler::ast::{CommitAst, FooterNode};
use crate::compiler::error::CompileError;
use crate::validation::ValidationError;

/// Semantic analyzer for commit ASTs
///
/// Validates that the commit follows conventional commit rules.
/// Does not modify the AST, only checks for correctness.
pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    /// Entry point
    pub fn analyze(ast: &CommitAst) -> Result<(), CompileError> {
        Self::validate_description(&ast.header.description)?;

        if let Some(scope) = &ast.header.scope {
            Self::validate_scope(scope)?;
        }

        Self::validate_footers(&ast.footers)?;
        Self::validate_breaking_changes(ast)?;

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

            if !seen_keys.insert(footer.key.clone()) {
                return Err(CompileError::SemanticError(
                    ValidationError::DuplicateFooter(footer.key.clone()),
                ));
            }

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

        let has_breaking_footer = breaking_footer.is_some();
        let has_breaking_header = ast.header.breaking;

        if has_breaking_header != has_breaking_footer {
            return Err(CompileError::SemanticError(
                ValidationError::BreakingChangeMismatch,
            ));
        }

        if has_breaking_header {
            if let Some(footer) = breaking_footer {
                if footer.value.trim().is_empty() {
                    return Err(CompileError::SemanticError(
                        ValidationError::EmptyBreakingChange,
                    ));
                }
            }
        }

        Ok(())
    }

    fn is_issue_footer(key: &str) -> bool {
        matches!(key, "Refs" | "Closes" | "Fixes")
    }

    fn validate_issue_reference(value: &str) -> Result<(), CompileError> {
        if !value.contains('#') {
            return Err(CompileError::SemanticError(
                ValidationError::InvalidIssueReference(value.to_string()),
            ));
        }

        Ok(())
    }
}
