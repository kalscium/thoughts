use thoughts::cli::Cli;
use clap::Parser;

fn main() {
    let cli: Cli = Cli::parse();
    cli.command.execute();
}
