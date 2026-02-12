use crate:: compiler:: compile:: compile;
use crate:: input:: InputError;

mod prompt;
mod builder;
mod validator;

pub use prompt:: Prompt;
pub use builder:: MessageBuilder;

pub struct InteractiveMode {
  prompt: Prompt,
    builder: MessageBuilder,
}

impl InteractiveMode {
    pub fn new () -> Self {
        Self {
      prompt: Prompt:: new (),
        builder: MessageBuilder:: new (),
        }
  }

    pub fn collect(& mut self) -> Result < String, InputError > {
    println!("\n=== Interactive Commit Builder ===\n");

        // Collect all parts
        let ty = self.prompt.commit_type() ?;
    self.builder.with_type(ty);

    let scope = self.prompt.scope() ?;
    self.builder.with_scope(scope);

    let breaking = self.prompt.breaking() ?;
    self.builder.with_breaking(breaking);

    let description = self.prompt.description() ?;
    self.builder.with_description(description);

    let body = self.prompt.body() ?;
    self.builder.with_body(body);

    if breaking {
    let breaking_desc = self.prompt.breaking_description() ?;
    self.builder.with_breaking_description(breaking_desc);
  }

  let footers = self.prompt.footers() ?;
  self.builder.with_footers(footers);

  // Build and compile
  let message = self.builder.build() ?;
  let formatted = compile(& message) ?;

  Ok(formatted)
}
}

impl Default for InteractiveMode {
  fn default() -> Self {
    Self:: new()
  }
}
