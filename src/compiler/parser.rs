use crate::commit_types::CommitType;
use crate::compiler::ast::{BodyNode, CommitAst, FooterNode, HeaderNode};
use crate::compiler::error::{CompileError, ParseError};
use crate::compiler::token::Token;

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
        let type_name = match self.next() {
            Token::Type(s) => CommitType::from_str(&s).map_err(CompileError::SemanticError)?,
            t => return Err(self.unexpected("Type", t)),
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
            t => return Err(self.unexpected("Description", t)),
        };

        Ok(HeaderNode {
            type_name,
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
                .ok_or_else(|| CompileError::ParseError(ParseError::InvalidFooter(raw.clone())))?;

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
        self.pos += 1;
    }

    fn consume_newlines(&mut self) {
        while matches!(self.peek(), Token::Newline) {
            self.advance();
        }
    }

    fn unexpected(&self, expected: &str, found: Token) -> CompileError {
        CompileError::ParseError(ParseError::UnexpectedToken {
            expected: expected.to_string(),
            found,
        })
    }
}

fn split_footer(raw: &str) -> Option<(String, String)> {
    let mut parts = raw.splitn(2, ':');
    let key = parts.next()?.trim();
    let value = parts.next()?.trim();
    if key.is_empty() || value.is_empty() {
        return None;
    }
    Some((key.to_string(), value.to_string()))
}
