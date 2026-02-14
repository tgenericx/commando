mod adapters;
mod app;
mod cli;
mod domain;
mod input;
mod ports;

fn main() -> std::process::ExitCode {
    cli::run()
}
