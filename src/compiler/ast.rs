/// Commit message AST — structurally valid, not semantically validated.
///
/// The parser produces this. It holds raw strings.
/// CommitMessage::try_from(ast) enforces domain invariants:
///   - commit_type string must be a known CommitType variant
///   - description must be ≤ 72 chars
///   - scope must be alphanumeric + hyphens/underscores
///
/// Keeping CommitAst clean of domain types means compiler/ never
/// imports from domain/ — the dependency flows one way only.
/// Root node of the commit AST.
#[derive(Debug, Clone, PartialEq)]
pub struct CommitAst {
    pub header: HeaderNode,
    pub body: Option<BodyNode>,
    pub footers: Vec<FooterNode>,
}

/// Header node — the first line of a conventional commit.
///
/// commit_type is a raw string. Domain validates whether it's
/// a known variant. The parser's job is just to extract it.
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderNode {
    pub commit_type: String, // raw — "feat", "fix", "unknown-type", etc.
    pub scope: Option<String>,
    pub breaking: bool, // was '!' present in the header?
    pub description: String,
}

/// Body node — the optional multi-line section after a blank line.
#[derive(Debug, Clone, PartialEq)]
pub struct BodyNode {
    pub content: String,
}

/// Footer node — a key/value pair from the footer section.
///
/// Examples: { key: "BREAKING CHANGE", value: "old API removed" }
///           { key: "Refs", value: "#123" }
///           { key: "Co-authored-by", value: "Name <email>" }
#[derive(Debug, Clone, PartialEq)]
pub struct FooterNode {
    pub key: String,
    pub value: String,
}
