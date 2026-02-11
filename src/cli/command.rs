#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Commit,
    Validate(String),
    Help,
    Version,
}

pub fn parse(args: &[String]) -> Command {
    if args.len() < 2 {
        return Command::Commit;
    }

    match args[1].as_str() {
        "--help" | "-h" => Command::Help,
        "--version" | "-v" => Command::Version,
        "--validate" if args.len() > 2 => Command::Validate(args[2].clone()),
        _ => Command::Commit,
    }
}
