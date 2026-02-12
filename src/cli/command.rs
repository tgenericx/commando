#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Interactive,
    Message(String),
    Editor,
    Validate(String),
    Help,
    Version,
}

pub fn parse(args: &[String]) -> Command {
    let mut iter = args.iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => return Command::Help,
            "--version" | "-v" => return Command::Version,
            "--interactive" | "-i" => return Command::Interactive,
            "--validate" => {
                if let Some(msg) = iter.next() {
                    return Command::Validate(msg.clone());
                }
            }
            "-m" | "--message" => {
                // Check if next arg exists and isn't another flag
                if let Some(next) = iter.next()
                    && !next.starts_with('-')
                {
                    return Command::Message(next.clone());
                }
                // No message provided, open editor
                return Command::Editor;
            }
            _ => {}
        }
    }

    // No flags = interactive
    Command::Interactive
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_interactive() {
        let args = vec!["commando".to_string()];
        assert_eq!(parse(&args), Command::Interactive);
    }

    #[test]
    fn message_flag_with_content() {
        let args = vec![
            "commando".to_string(),
            "-m".to_string(),
            "feat: add feature".to_string(),
        ];
        assert_eq!(
            parse(&args),
            Command::Message("feat: add feature".to_string())
        );
    }

    #[test]
    fn message_flag_without_content_opens_editor() {
        let args = vec!["commando".to_string(), "-m".to_string()];
        assert_eq!(parse(&args), Command::Editor);
    }

    #[test]
    fn interactive_flag() {
        let args = vec!["commando".to_string(), "-i".to_string()];
        assert_eq!(parse(&args), Command::Interactive);
    }
}
