use crate::compiler::ast::{BodyNode, CommitAst, FooterNode, HeaderNode};
use crate::compiler::error::{CompileError, ParseError};
use crate::compiler::token::Token;

/// Parser — converts a token stream into a CommitAst.
///
/// Does NOT validate semantic correctness — CommitMessage::try_from(ast)
/// does that. The parser's only job is syntax: do the tokens form a
/// structurally valid conventional commit?
///
/// Specifically: the parser does NOT call CommitType::from_str().
/// commit_type stays as a raw String in HeaderNode. The domain validates it.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<CommitAst, CompileError> {
        let header = self.parse_header()?;
        self.consume_newlines();

        let body = self.parse_body()?;
        self.consume_newlines();

        let footers = self.parse_footers()?;

        Ok(CommitAst {
            header,
            body,
            footers,
        })
    }

    fn parse_header(&mut self) -> Result<HeaderNode, CompileError> {
        // commit_type: raw string — NOT validated against CommitType enum here
        let commit_type = match self.next() {
            Token::Type(s) => s,
            t => return Err(self.unexpected("Type token", t)),
        };

        let scope = if let Token::Scope(s) = self.peek() {
            self.advance();
            Some(s)
        } else {
            None
        };

        let breaking = if matches!(self.peek(), Token::Breaking) {
            self.advance();
            true
        } else {
            false
        };

        let description = match self.next() {
            Token::Description(s) => s,
            t => return Err(self.unexpected("Description token", t)),
        };

        Ok(HeaderNode {
            commit_type,
            scope,
            breaking,
            description,
        })
    }

    fn parse_body(&mut self) -> Result<Option<BodyNode>, CompileError> {
        match self.peek() {
            Token::Body(text) => {
                self.advance();
                Ok(Some(BodyNode { content: text }))
            }
            _ => Ok(None),
        }
    }

    fn parse_footers(&mut self) -> Result<Vec<FooterNode>, CompileError> {
        let mut footers = Vec::new();

        while let Token::Footer(raw) = self.peek() {
            self.advance();

            let (key, value) = split_footer(&raw)
                .ok_or_else(|| CompileError::Parse(ParseError::InvalidFooter(raw.clone())))?;

            footers.push(FooterNode { key, value });
            self.consume_newlines();
        }

        Ok(footers)
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof)
    }

    fn next(&mut self) -> Token {
        let token = self.peek();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn consume_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    fn unexpected(&self, expected: &str, found: Token) -> CompileError {
        CompileError::Parse(ParseError::UnexpectedToken {
            expected: expected.to_string(),
            found,
        })
    }
}

/// Split "KEY: value" or "KEY #value" into (key, value).
/// Returns None if the line doesn't have a valid separator.
fn split_footer(raw: &str) -> Option<(String, String)> {
    // Try ": " separator first (standard)
    if let Some(pos) = raw.find(": ") {
        let key = raw[..pos].trim().to_string();
        let value = raw[pos + 2..].trim().to_string();
        if !key.is_empty() && !value.is_empty() {
            return Some((key, value));
        }
    }

    // Try " #" separator (e.g. "Refs #123")
    if let Some(pos) = raw.find(" #") {
        let key = raw[..pos].trim().to_string();
        let value = raw[pos + 1..].trim().to_string();
        if !key.is_empty() && !value.is_empty() {
            return Some((key, value));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Lexer;

    fn parse(input: &str) -> CommitAst {
        let tokens = Lexer::new(input).tokenize().expect("lex failed");
        Parser::new(tokens).parse().expect("parse failed")
    }

    #[test]
    fn parses_minimal_commit() {
        let ast = parse("feat: add login");
        assert_eq!(ast.header.commit_type, "feat");
        assert_eq!(ast.header.scope, None);
        assert!(!ast.header.breaking);
        assert_eq!(ast.header.description, "add login");
        assert!(ast.body.is_none());
        assert!(ast.footers.is_empty());
    }

    #[test]
    fn parses_commit_with_scope() {
        let ast = parse("fix(auth): correct token expiry");
        assert_eq!(ast.header.commit_type, "fix");
        assert_eq!(ast.header.scope, Some("auth".into()));
        assert_eq!(ast.header.description, "correct token expiry");
    }

    #[test]
    fn parses_breaking_marker() {
        let ast = parse("feat(api)!: remove v1 endpoints");
        assert!(ast.header.breaking);
        assert_eq!(ast.header.scope, Some("api".into()));
    }

    #[test]
    fn parses_body() {
        let input = "feat: add search\n\nFull-text search using inverted index.";
        let ast = parse(input);
        assert_eq!(
            ast.body.unwrap().content,
            "Full-text search using inverted index."
        );
    }

    #[test]
    fn parses_footer() {
        let input = "fix: patch null pointer\n\nRefs: #42";
        let ast = parse(input);
        assert_eq!(ast.footers.len(), 1);
        assert_eq!(ast.footers[0].key, "Refs");
        assert_eq!(ast.footers[0].value, "#42");
    }

    #[test]
    fn parses_breaking_change_footer() {
        let input = "feat!: redesign API\n\nBREAKING CHANGE: all v1 endpoints removed";
        let ast = parse(input);
        let bc = ast.footers.iter().find(|f| f.key == "BREAKING CHANGE");
        assert!(bc.is_some());
        assert_eq!(bc.unwrap().value, "all v1 endpoints removed");
    }

    #[test]
    fn unknown_type_parses_successfully() {
        // Parser does not validate type — domain does
        let ast = parse("unknown: do something");
        assert_eq!(ast.header.commit_type, "unknown");
    }

    #[test]
    fn parses_full_commit() {
        let input = "feat(auth)!: migrate to OAuth\n\n\
                     Migrated from session-based auth.\n\n\
                     BREAKING CHANGE: sessions invalidated\n\
                     Refs: #88";
        let ast = parse(input);
        assert_eq!(ast.header.commit_type, "feat");
        assert_eq!(ast.header.scope, Some("auth".into()));
        assert!(ast.header.breaking);
        assert!(ast.body.is_some());
        assert_eq!(ast.footers.len(), 2);
    }
}
