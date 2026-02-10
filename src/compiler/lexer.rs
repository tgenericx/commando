use super::CompileError;
use super::token::Token;

/// Lexer for commit message tokenization
///
/// Converts raw commit message text into a stream of tokens.
/// The lexer identifies structural elements but does not validate
/// semantic correctness (e.g., whether a type is valid).
///
/// # Architecture
/// - Header: First line containing type, optional scope, and description
/// - Body: Optional multi-line detailed explanation (separated by blank line)
/// - Footer: Optional metadata lines (e.g., "BREAKING CHANGE:", "Refs:")
#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    /// Create a new lexer from input text
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
        }
    }

    /// Tokenize the entire input into a vector of tokens
    ///
    /// # Returns
    /// - `Ok(Vec<Token>)` - Successfully tokenized input
    /// - `Err(CompileError)` - Lexical error encountered
    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();

        // Split input into lines for processing
        let lines: Vec<&str> = self.input.lines().collect();

        if lines.is_empty() {
            tokens.push(Token::Eof);
            return Ok(tokens);
        }

        // Process header (first line)
        let header_tokens = self.tokenize_header(lines[0])?;
        tokens.extend(header_tokens);
        tokens.push(Token::Newline);

        // Determine if there's a body or footer
        let mut i = 1;

        // Skip blank lines after header
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }

        if i >= lines.len() {
            tokens.push(Token::Eof);
            return Ok(tokens);
        }

        // Collect remaining lines
        let remaining_lines: Vec<&str> = lines[i..].to_vec();

        // Separate body from footer
        let (body_lines, footer_lines) = self.split_body_and_footer(&remaining_lines);

        // Tokenize body if present
        if !body_lines.is_empty() {
            let body_text = body_lines.join("\n");
            if !body_text.trim().is_empty() {
                tokens.push(Token::Body(body_text));
                tokens.push(Token::Newline);
            }
        }

        // Tokenize footer if present
        for footer_line in footer_lines {
            if !footer_line.trim().is_empty() {
                tokens.push(Token::Footer(footer_line.to_string()));
                tokens.push(Token::Newline);
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    /// Tokenize the header line into type, scope, breaking, and description
    ///
    /// Expected format: `type(scope)!: description`
    /// - `type` is required
    /// - `(scope)` is optional
    /// - `!` is optional (breaking change indicator)
    /// - `:` separates header from description
    /// - `description` is required
    fn tokenize_header(&self, header: &str) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();

        let header = header.trim();
        if header.is_empty() {
            return Err(CompileError::LexerError("Empty header line".to_string()));
        }

        // Find the colon that separates header from description
        let colon_pos = header.find(':').ok_or_else(|| {
            CompileError::LexerError("Missing ':' separator in header".to_string())
        })?;

        let header_part = &header[..colon_pos];
        let description = header[colon_pos + 1..].trim();

        if description.is_empty() {
            return Err(CompileError::LexerError("Empty description".to_string()));
        }

        // Parse header_part for type, scope, and breaking
        let (commit_type, scope, has_breaking) = self.parse_header_part(header_part)?;

        tokens.push(Token::Type(commit_type));

        if let Some(scope_str) = scope {
            tokens.push(Token::Scope(scope_str));
        }

        if has_breaking {
            tokens.push(Token::Breaking);
        }

        tokens.push(Token::Description(description.to_string()));

        Ok(tokens)
    }

    /// Parse the header part before the colon
    ///
    /// Returns (type, optional_scope, has_breaking)
    fn parse_header_part(
        &self,
        header_part: &str,
    ) -> Result<(String, Option<String>, bool), CompileError> {
        let header_part = header_part.trim();

        // Check for breaking change indicator (!)
        let has_breaking = header_part.ends_with('!');
        let header_part = if has_breaking {
            &header_part[..header_part.len() - 1]
        } else {
            header_part
        };

        // Check for scope (enclosed in parentheses)
        if let Some(open_paren) = header_part.find('(') {
            let close_paren = header_part.find(')').ok_or_else(|| {
                CompileError::LexerError("Unclosed scope parenthesis".to_string())
            })?;

            if close_paren < open_paren {
                return Err(CompileError::LexerError("Malformed scope".to_string()));
            }

            let commit_type = header_part[..open_paren].trim().to_string();
            let scope = header_part[open_paren + 1..close_paren].trim().to_string();

            if commit_type.is_empty() {
                return Err(CompileError::LexerError("Empty commit type".to_string()));
            }

            if scope.is_empty() {
                return Err(CompileError::LexerError("Empty scope".to_string()));
            }

            // Check for extra content after closing parenthesis
            let after_paren = header_part[close_paren + 1..].trim();
            if !after_paren.is_empty() {
                return Err(CompileError::LexerError(
                    "Unexpected content after scope".to_string(),
                ));
            }

            Ok((commit_type, Some(scope), has_breaking))
        } else {
            // No scope
            let commit_type = header_part.trim().to_string();

            if commit_type.is_empty() {
                return Err(CompileError::LexerError("Empty commit type".to_string()));
            }

            Ok((commit_type, None, has_breaking))
        }
    }

    /// Split remaining lines into body and footer sections
    ///
    /// Footer lines are identified by the pattern "KEY: value" or "KEY #value"
    /// Common footer keys: BREAKING CHANGE, Refs, Closes, Fixes, etc.
    fn split_body_and_footer<'a>(&self, lines: &'a [&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
        // Find the first footer line
        let mut footer_start = None;

        for (i, line) in lines.iter().enumerate() {
            if self.is_footer_line(line) {
                footer_start = Some(i);
                break;
            }
        }

        match footer_start {
            Some(idx) => {
                let body_lines = lines[..idx].to_vec();
                let footer_lines = lines[idx..].to_vec();
                (body_lines, footer_lines)
            }
            None => {
                // No footer, everything is body
                (lines.to_vec(), Vec::new())
            }
        }
    }

    /// Check if a line matches the footer pattern
    ///
    /// Footer pattern: "KEY: value" or "KEY #value"
    /// KEY must be uppercase or kebab-case
    fn is_footer_line(&self, line: &str) -> bool {
        let line = line.trim();

        // Check for "KEY: value" pattern
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim();
            return self.is_valid_footer_key(key);
        }

        // Check for "KEY #value" pattern (e.g., "Refs #123")
        if let Some(hash_pos) = line.find('#') {
            let key = line[..hash_pos].trim();
            return self.is_valid_footer_key(key);
        }

        false
    }

    /// Validate if a string is a valid footer key
    ///
    /// Valid keys are uppercase words or kebab-case
    /// Examples: "BREAKING CHANGE", "Refs", "Co-authored-by"
    fn is_valid_footer_key(&self, key: &str) -> bool {
        if key.is_empty() {
            return false;
        }

        // Common footer keys
        let common_keys = [
            "BREAKING CHANGE",
            "BREAKING-CHANGE",
            "Refs",
            "Closes",
            "Fixes",
            "Co-authored-by",
            "Signed-off-by",
            "Reviewed-by",
        ];

        if common_keys.contains(&key) {
            return true;
        }

        // Check if it's uppercase or kebab-case
        let is_uppercase_or_kebab = key
            .chars()
            .all(|c| c.is_uppercase() || c.is_whitespace() || c == '-');

        // Or starts with capital letter (for "Refs", "Closes", etc.)
        let starts_with_capital = key
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false);

        is_uppercase_or_kebab || starts_with_capital
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_can_be_created() {
        let _lexer = Lexer::new("feat: add feature");
    }

    #[test]
    fn tokenize_simple_commit() {
        let mut lexer = Lexer::new("feat: add user authentication");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4); // Type, Description, Newline, Eof
        assert_eq!(tokens[0], Token::Type("feat".to_string()));
        assert_eq!(
            tokens[1],
            Token::Description("add user authentication".to_string())
        );
        assert!(tokens[2].is_newline());
        assert!(tokens[3].is_eof());
    }

    #[test]
    fn tokenize_commit_with_scope() {
        let mut lexer = Lexer::new("fix(parser): handle edge case");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Type("fix".to_string()));
        assert_eq!(tokens[1], Token::Scope("parser".to_string()));
        assert_eq!(
            tokens[2],
            Token::Description("handle edge case".to_string())
        );
    }

    #[test]
    fn tokenize_commit_with_breaking_change() {
        let mut lexer = Lexer::new("feat!: change API");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Type("feat".to_string()));
        assert_eq!(tokens[1], Token::Breaking);
        assert_eq!(tokens[2], Token::Description("change API".to_string()));
    }

    #[test]
    fn tokenize_commit_with_scope_and_breaking() {
        let mut lexer = Lexer::new("feat(api)!: remove endpoint");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::Type("feat".to_string()));
        assert_eq!(tokens[1], Token::Scope("api".to_string()));
        assert_eq!(tokens[2], Token::Breaking);
        assert_eq!(tokens[3], Token::Description("remove endpoint".to_string()));
    }

    #[test]
    fn tokenize_commit_with_body() {
        let input = "feat: add feature\n\nThis is a detailed explanation\nof the feature.";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Find the body token
        let body_token = tokens.iter().find(|t| matches!(t, Token::Body(_)));
        assert!(body_token.is_some());

        if let Some(Token::Body(body)) = body_token {
            assert!(body.contains("detailed explanation"));
        }
    }

    #[test]
    fn tokenize_commit_with_footer() {
        let input = "feat: add feature\n\nBREAKING CHANGE: API changed";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        let footer_token = tokens.iter().find(|t| matches!(t, Token::Footer(_)));
        assert!(footer_token.is_some());

        if let Some(Token::Footer(footer)) = footer_token {
            assert_eq!(footer, "BREAKING CHANGE: API changed");
        }
    }

    #[test]
    fn tokenize_commit_with_body_and_footer() {
        let input = "feat(auth): implement OAuth\n\nAdded OAuth 2.0 support with refresh tokens.\nImproved security.\n\nBREAKING CHANGE: Old auth removed\nRefs: #123";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        // Should have type, scope, description, newline, body, newline, footer, newline, footer, newline, eof
        let has_body = tokens.iter().any(|t| matches!(t, Token::Body(_)));
        let footer_count = tokens
            .iter()
            .filter(|t| matches!(t, Token::Footer(_)))
            .count();

        assert!(has_body);
        assert_eq!(footer_count, 2);
    }

    #[test]
    fn lexer_error_on_missing_colon() {
        let mut lexer = Lexer::new("feat add feature");
        let result = lexer.tokenize();

        assert!(result.is_err());
        if let Err(CompileError::LexerError(msg)) = result {
            assert!(msg.contains("Missing ':'"));
        }
    }

    #[test]
    fn lexer_error_on_empty_description() {
        let mut lexer = Lexer::new("feat: ");
        let result = lexer.tokenize();

        assert!(result.is_err());
    }

    #[test]
    fn lexer_error_on_unclosed_scope() {
        let mut lexer = Lexer::new("feat(api: add feature");
        let result = lexer.tokenize();

        assert!(result.is_err());
        if let Err(CompileError::LexerError(msg)) = result {
            assert!(msg.contains("Unclosed scope"));
        }
    }

    #[test]
    fn lexer_error_on_empty_scope() {
        let mut lexer = Lexer::new("feat(): add feature");
        let result = lexer.tokenize();

        assert!(result.is_err());
    }

    #[test]
    fn lexer_handles_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(tokens[0].is_eof());
    }

    #[test]
    fn is_footer_line_identifies_common_footers() {
        let lexer = Lexer::new("");

        assert!(lexer.is_footer_line("BREAKING CHANGE: details"));
        assert!(lexer.is_footer_line("Refs: #123"));
        assert!(lexer.is_footer_line("Closes #456"));
        assert!(lexer.is_footer_line("Co-authored-by: John Doe"));
        assert!(lexer.is_footer_line("Signed-off-by: Jane Smith"));
    }

    #[test]
    fn is_footer_line_rejects_regular_text() {
        let lexer = Lexer::new("");

        assert!(!lexer.is_footer_line("This is just a regular line"));
        assert!(!lexer.is_footer_line("no colon here"));
        assert!(!lexer.is_footer_line("lowercase: value"));
    }
}
