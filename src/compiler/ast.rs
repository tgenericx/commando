use crate::commit_types::CommitType;

/// Root node of commit AST
#[derive(Debug, Clone, PartialEq)]
pub struct CommitAst {
    pub header: HeaderNode,
    pub body: Option<BodyNode>,
    pub footers: Vec<FooterNode>,
}

/// Header node
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderNode {
    pub type_name: CommitType,
    pub scope: Option<String>,
    pub breaking: bool,
    pub description: String,
}

/// Body node
#[derive(Debug, Clone, PartialEq)]
pub struct BodyNode {
    pub content: String,
}

/// Footer node
#[derive(Debug, Clone, PartialEq)]
pub struct FooterNode {
    pub key: String,
    pub value: String,
}
