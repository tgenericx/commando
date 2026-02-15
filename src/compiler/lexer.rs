use super::error::CompileError;
use super::token::Token;

/// Lexer — converts raw commit message text into a token stream.
///
/// Identifies structural elements but does NOT validate semantic correctness.
/// "feat", "feat2", "invalid-type" all produce Token::Type — the domain
/// decides whether the value is a valid CommitType.
///
/// Structure it handles:
///   Line 1:       header  (type, optional scope, optional '!', description)
///   Blank line:   section separator
///   Next section: body (free text) or footer (key: value lines)
///   Blank line:   separator between body and footer
///   Last section: footer lines
#[derive(Debug)]
pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }

    /// Tokenize the entire input into a Vec<Token>.
    pub fn tokenize(&self) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        let lines: Vec<&str> = self.input.lines().collect();

        if lines.is_empty() {
            return Err(CompileError::Lex("Empty input".to_string()));
        }

        // ── Header (line 0) ──────────────────────────────────────────
        let header_tokens = self.tokenize_header(lines[0])?;
        tokens.extend(header_tokens);
        tokens.push(Token::Newline);

        // ── Skip blank lines after header ────────────────────────────
        let mut i = 1;
        while i < lines.len() && lines[i].trim().is_empty() {
            i += 1;
        }

        if i >= lines.len() {
            tokens.push(Token::Eof);
            return Ok(tokens);
        }

        // ── Body and/or footer ───────────────────────────────────────
        let remaining: Vec<&str> = lines[i..].to_vec();
        let (body_lines, footer_lines) = self.split_body_and_footer(&remaining);

        if !body_lines.is_empty() {
            let body_text = body_lines.join("\n");
            let trimmed = body_text.trim().to_string();
            if !trimmed.is_empty() {
                tokens.push(Token::Body(trimmed));
                tokens.push(Token::Newline);
            }
        }

        for line in footer_lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                tokens.push(Token::Footer(trimmed.to_string()));
                tokens.push(Token::Newline);
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    /// Tokenize the header line: `type[(scope)][!]: description`
    fn tokenize_header(&self, header: &str) -> Result<Vec<Token>, CompileError> {
        let mut tokens = Vec::new();
        let header = header.trim();

        if header.is_empty() {
            return Err(CompileError::Lex("Empty header line".to_string()));
        }

        let colon_pos = header
            .find(':')
            .ok_or_else(|| CompileError::Lex("Missing ':' in header".to_string()))?;

        let before_colon = &header[..colon_pos];
        let after_colon = header[colon_pos + 1..].trim();

        if after_colon.is_empty() {
            return Err(CompileError::Lex("Empty description".to_string()));
        }

        let (commit_type, scope, breaking) = self.parse_type_scope_breaking(before_colon)?;

        tokens.push(Token::Type(commit_type));
        if let Some(s) = scope {
            tokens.push(Token::Scope(s));
        }
        if breaking {
            tokens.push(Token::Breaking);
        }
        tokens.push(Token::Description(after_colon.to_string()));

        Ok(tokens)
    }

    /// Parse `type[(scope)][!]` from the part before ':'.
    ///
    /// Returns (type_string, optional_scope, has_breaking).
    fn parse_type_scope_breaking(
        &self,
        part: &str,
    ) -> Result<(String, Option<String>, bool), CompileError> {
        let part = part.trim();

        // Strip trailing '!'
        let (part, breaking) = if part.ends_with('!') {
            (&part[..part.len() - 1], true)
        } else {
            (part, false)
        };

        // Check for scope: type(scope)
        if let Some(open) = part.find('(') {
            let close = part
                .rfind(')')
                .ok_or_else(|| CompileError::Lex("Unclosed '(' in scope".to_string()))?;

            if close < open {
                return Err(CompileError::Lex("Malformed scope parentheses".to_string()));
            }

            let commit_type = part[..open].trim().to_string();
            let scope = part[open + 1..close].trim().to_string();
            let after_close = part[close + 1..].trim();

            if commit_type.is_empty() {
                return Err(CompileError::Lex("Empty commit type".to_string()));
            }
            if scope.is_empty() {
                return Err(CompileError::Lex("Empty scope".to_string()));
            }
            if !after_close.is_empty() {
                return Err(CompileError::Lex(
                    "Unexpected content after scope ')'".to_string(),
                ));
            }

            Ok((commit_type, Some(scope), breaking))
        } else {
            let commit_type = part.trim().to_string();
            if commit_type.is_empty() {
                return Err(CompileError::Lex("Empty commit type".to_string()));
            }
            Ok((commit_type, None, breaking))
        }
    }

    /// Split remaining lines into body lines and footer lines.
    ///
    /// Body comes first. Footers start at the first line that looks like
    /// a footer token. A blank line may separate body from footer.
    fn split_body_and_footer<'a>(&self, lines: &'a [&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
        // Find the first blank-line-then-footer boundary, or just the first footer line.
        // Strategy: scan forward; once we see a footer-looking line after any content,
        // everything from there is footer.
        let mut footer_start = None;

        for (i, line) in lines.iter().enumerate() {
            if self.is_footer_line(line) {
                footer_start = Some(i);
                break;
            }
        }

        match footer_start {
            Some(idx) => {
                // Trim trailing blank lines from body
                let body = &lines[..idx];
                let footer = &lines[idx..];
                (body.to_vec(), footer.to_vec())
            }
            None => (lines.to_vec(), Vec::new()),
        }
    }

    /// Determine whether a line is a footer token line.
    ///
    /// Per the conventional commits spec, a footer token is:
    ///   - "BREAKING CHANGE" (with a space — special case)
    ///   - A word-token: one or more chars, none of which are whitespace or ':'
    ///     followed by ': ' or ' #'
    ///
    /// Heuristic: the part before ':' or ' #' must contain no lowercase
    /// letters mixed with spaces (which would make it prose, not a token).
    /// Single-word tokens starting with a capital letter are fine (Refs, Closes).
    /// Multi-word tokens must be ALL-CAPS-WITH-SPACES (BREAKING CHANGE).
    fn is_footer_line(&self, line: &str) -> bool {
        let line = line.trim();
        if line.is_empty() {
            return false;
        }

        // "BREAKING CHANGE: ..." — explicit special case
        if line.starts_with("BREAKING CHANGE:") || line.starts_with("BREAKING-CHANGE:") {
            return true;
        }

        // "KEY: value" or "KEY #value"
        let key = if let Some(colon_pos) = line.find(": ") {
            &line[..colon_pos]
        } else if let Some(hash_pos) = line.find(" #") {
            &line[..hash_pos]
        } else {
            return false;
        };

        let key = key.trim();
        if key.is_empty() {
            return false;
        }

        // A valid footer key is either:
        //   - A single word (no spaces), which may be CamelCase or lowercase: Refs, closes
        //   - All-caps words separated by spaces or hyphens: BREAKING CHANGE
        //   - Kebab-case starting with capital: Co-authored-by
        let has_space = key.contains(' ');

        if has_space {
            // Multi-word keys must be all-caps
            key.chars()
                .all(|c| c.is_uppercase() || c == ' ' || c == '-')
        } else {
            // Single-word: must start with a capital letter or be all-caps
            // Rejects: lines that start with lowercase (those are prose)
            key.chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(input: &str) -> Vec<Token> {
        Lexer::new(input).tokenize().expect("tokenize failed")
    }

    #[test]
    fn minimal_commit() {
        let tokens = lex("feat: add login");
        assert_eq!(tokens[0], Token::Type("feat".into()));
        assert_eq!(tokens[1], Token::Description("add login".into()));
        assert_eq!(tokens[2], Token::Newline);
        assert_eq!(tokens[3], Token::Eof);
    }

    #[test]
    fn commit_with_scope() {
        let tokens = lex("fix(auth): correct token expiry");
        assert_eq!(tokens[0], Token::Type("fix".into()));
        assert_eq!(tokens[1], Token::Scope("auth".into()));
        assert_eq!(tokens[2], Token::Description("correct token expiry".into()));
    }

    #[test]
    fn commit_with_breaking_marker() {
        let tokens = lex("feat(api)!: remove v1 endpoints");
        assert_eq!(tokens[0], Token::Type("feat".into()));
        assert_eq!(tokens[1], Token::Scope("api".into()));
        assert_eq!(tokens[2], Token::Breaking);
        assert_eq!(tokens[3], Token::Description("remove v1 endpoints".into()));
    }

    #[test]
    fn commit_with_body() {
        let input = "feat: add search\n\nFull-text search using inverted index.";
        let tokens = lex(input);
        assert!(tokens.contains(&Token::Body(
            "Full-text search using inverted index.".into()
        )));
    }

    #[test]
    fn commit_with_footer() {
        let input = "fix: patch null pointer\n\nRefs: #42";
        let tokens = lex(input);
        assert!(tokens.contains(&Token::Footer("Refs: #42".into())));
    }

    #[test]
    fn breaking_change_footer() {
        let input = "feat!: redesign API\n\nBREAKING CHANGE: all endpoints changed";
        let tokens = lex(input);
        assert!(tokens.contains(&Token::Footer(
            "BREAKING CHANGE: all endpoints changed".into()
        )));
    }

    #[test]
    fn missing_colon_is_error() {
        let result = Lexer::new("feat add login").tokenize();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CompileError::Lex(_)));
    }

    #[test]
    fn empty_description_is_error() {
        let result = Lexer::new("feat: ").tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn unclosed_scope_is_error() {
        let result = Lexer::new("feat(auth: fix thing").tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn unknown_type_is_not_a_lex_error() {
        // The lexer does not validate type values — domain does
        let tokens = lex("unknown-type: do something");
        assert_eq!(tokens[0], Token::Type("unknown-type".into()));
    }
}
