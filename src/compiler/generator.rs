use crate::compiler::ast::{CommitAst, FooterNode};

const MAX_LINE_WIDTH: usize = 72;

pub struct CommitFormatter;

impl CommitFormatter {
    pub fn format(ast: &CommitAst) -> String {
        let mut output = String::new();

        // ----- HEADER -----
        output.push_str(&Self::format_header(ast));

        // ----- BODY -----
        if let Some(body) = &ast.body {
            let wrapped_body = Self::wrap_text(body.content.trim());
            if !wrapped_body.is_empty() {
                output.push_str("\n\n");
                output.push_str(&wrapped_body);
            }
        }

        // ----- FOOTERS -----
        if !ast.footers.is_empty() {
            output.push_str("\n\n");

            let ordered = Self::order_footers(&ast.footers);

            let formatted = ordered
                .iter()
                .map(Self::format_footer)
                .collect::<Vec<_>>()
                .join("\n");

            output.push_str(&formatted);
        }

        output
    }

    fn format_header(ast: &CommitAst) -> String {
        let mut header = String::new();

        header.push_str(ast.header.type_name.as_str());

        if let Some(scope) = &ast.header.scope {
            header.push('(');
            header.push_str(scope);
            header.push(')');
        }

        if ast.header.breaking {
            header.push('!');
        }

        header.push_str(": ");
        header.push_str(ast.header.description.trim());

        header
    }

    fn format_footer(footer: &FooterNode) -> String {
        format!("{}: {}", footer.key.trim(), footer.value.trim())
    }

    fn order_footers(footers: &[FooterNode]) -> Vec<FooterNode> {
        let mut breaking = Vec::new();
        let mut others = Vec::new();

        for footer in footers {
            if footer.key == "BREAKING CHANGE" || footer.key == "BREAKING-CHANGE" {
                breaking.push(footer.clone());
            } else {
                others.push(footer.clone());
            }
        }

        breaking.into_iter().chain(others.into_iter()).collect()
    }

    fn wrap_text(text: &str) -> String {
        let mut result = String::new();
        let mut line_length = 0;

        for word in text.split_whitespace() {
            let word_len = word.len();

            if line_length + word_len + 1 > MAX_LINE_WIDTH {
                result.push('\n');
                result.push_str(word);
                line_length = word_len;
            } else {
                if line_length != 0 {
                    result.push(' ');
                    line_length += 1;
                }
                result.push_str(word);
                line_length += word_len;
            }
        }

        result
    }
}
