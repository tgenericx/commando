#[derive(Debug, Default)]
pub struct MessageBuilder {
    commit_type: Option<String>,
    scope: Option<String>,
    breaking: bool,
    description: Option<String>,
    body: Option<String>,
    breaking_description: Option<String>,
    footers: Option<String>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn commit_type(&self) -> Option<&String> {
        self.commit_type.as_ref()
    }

    pub fn scope(&self) -> Option<&String> {
        self.scope.as_ref()
    }

    pub fn breaking(&self) -> Option<bool> {
        Some(self.breaking)
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }

    pub fn breaking_description(&self) -> Option<&String> {
        self.breaking_description.as_ref()
    }

    pub fn footers(&self) -> Option<&String> {
        self.footers.as_ref()
    }

    pub fn with_type(&mut self, ty: String) -> &mut Self {
        self.commit_type = Some(ty);
        self
    }

    pub fn with_scope(&mut self, scope: Option<String>) -> &mut Self {
        self.scope = scope;
        self
    }

    pub fn with_breaking(&mut self, breaking: bool) -> &mut Self {
        self.breaking = breaking;
        self
    }

    pub fn with_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn with_body(&mut self, body: Option<String>) -> &mut Self {
        self.body = body;
        self
    }

    pub fn with_breaking_description(&mut self, description: String) -> &mut Self {
        self.breaking_description = Some(description);
        self
    }

    pub fn with_footers(&mut self, footers: Option<String>) -> &mut Self {
        self.footers = footers;
        self
    }

    pub fn build(&self) -> Result<String, crate::input::InputError> {
        let mut message = String::new();

        // Type (required)
        let ty = self
            .commit_type
            .as_ref()
            .ok_or_else(|| crate::input::InputError::Empty)?;
        message.push_str(ty);

        // Scope (optional)
        if let Some(scope) = &self.scope {
            message.push('(');
            message.push_str(scope);
            message.push(')');
        }

        // Breaking indicator
        if self.breaking {
            message.push('!');
        }

        // Description (required)
        let desc = self
            .description
            .as_ref()
            .ok_or_else(|| crate::input::InputError::Empty)?;
        message.push_str(": ");
        message.push_str(desc);

        // Body (optional)
        if let Some(body) = &self.body {
            message.push_str("\n\n");
            message.push_str(body);
        }

        // Breaking change footer
        if self.breaking {
            let breaking_desc = self
                .breaking_description
                .as_ref()
                .ok_or_else(|| crate::input::InputError::Empty)?;
            message.push_str("\n\nBREAKING CHANGE: ");
            message.push_str(breaking_desc);
        }

        // Additional footers
        if let Some(footers) = &self.footers {
            message.push_str("\n\n");
            message.push_str(footers);
        }

        Ok(message)
    }
}
