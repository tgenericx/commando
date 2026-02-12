#[derive(Debug, Default)]
pub struct MessageBuilder {
    commit_type: Option<String>,
    scope: Option<Option<String>>,
    breaking: Option<bool>,
    description: Option<String>,
    body: Option<Option<String>>,
    breaking_description: Option<String>,
    footers: Option<Option<String>>,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            commit_type: None,
            scope: None,
            breaking: None,
            description: None,
            body: None,
            breaking_description: None,
            footers: None,
        }
    }

    pub fn commit_type(&self) -> Option<&String> {
        self.commit_type.as_ref()
    }

    pub fn scope(&self) -> Option<&Option<String>> {
        self.scope.as_ref()
    }

    pub fn breaking(&self) -> Option<bool> {
        self.breaking
    }

    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn body(&self) -> Option<&Option<String>> {
        self.body.as_ref()
    }

    pub fn breaking_description(&self) -> Option<&String> {
        self.breaking_description.as_ref()
    }

    pub fn footers(&self) -> Option<&Option<String>> {
        self.footers.as_ref()
    }

    pub fn with_type(&mut self, ty: String) -> &mut Self {
        self.commit_type = Some(ty);
        self
    }

    pub fn with_scope(&mut self, scope: Option<String>) -> &mut Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_breaking(&mut self, breaking: bool) -> &mut Self {
        self.breaking = Some(breaking);
        self
    }

    pub fn with_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    pub fn with_body(&mut self, body: Option<String>) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn with_breaking_description(&mut self, description: String) -> &mut Self {
        self.breaking_description = Some(description);
        self
    }

    pub fn with_footers(&mut self, footers: Option<String>) -> &mut Self {
        self.footers = Some(footers);
        self
    }

    pub fn build(&self) -> Result<String, crate::input::InputError> {
        let mut message = String::new();

        let ty = self
            .commit_type
            .as_ref()
            .ok_or(crate::input::InputError::Empty)?;
        message.push_str(ty);

        if let Some(Some(scope)) = self.scope.as_ref() {
            message.push('(');
            message.push_str(scope);
            message.push(')');
        }

        if self.breaking == Some(true) {
            message.push('!');
        }

        let desc = self
            .description
            .as_ref()
            .ok_or(crate::input::InputError::Empty)?;
        message.push_str(": ");
        message.push_str(desc);

        if let Some(Some(body)) = self.body.as_ref() {
            message.push_str("\n\n");
            message.push_str(body);
        }

        if self.breaking == Some(true) {
            let breaking_desc = self
                .breaking_description
                .as_ref()
                .ok_or(crate::input::InputError::Empty)?;
            message.push_str("\n\nBREAKING CHANGE: ");
            message.push_str(breaking_desc);
        }

        if let Some(Some(footers)) = self.footers.as_ref() {
            message.push_str("\n\n");
            message.push_str(footers);
        }

        Ok(message)
    }
}
