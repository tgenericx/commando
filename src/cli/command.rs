#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Interactive,
    Commit,
    Validate(String),
    Help,
    Version,
}

pub fn parse(args: &[String]) -> Command {
    if args.len() < 2 {
        return Command::Interactive;
    }

    match args[1].as_str() {
        "--help" | "-h" => Command::Help,
        "--version" | "-v" => Command::Version,
        "--validate" if args.len() > 2 => Command::Validate(args[2].clone()),
        "--interactive" | "-i" => Command::Interactive,
        _ => Command::Commit,
    }
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
    fn explicit_interactive_flag() {
        let args = vec!["commando".to_string(), "--interactive".to_string()];
        assert_eq!(parse(&args), Command::Interactive);

        let args = vec!["commando".to_string(), "-i".to_string()];
        assert_eq!(parse(&args), Command::Interactive);
    }
}
