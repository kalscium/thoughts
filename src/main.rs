use clap::Parser;
use log::info;
use thoughts::{cli::Cli, database::Database, get_dir, log::Logger};

pub static LOGGER: Logger = Logger;

fn main() {
    // setup logging & panic hook
    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Info));
    color_eyre::install().unwrap();

    // parse the cli and match the command
    let cli = Cli::parse();
    use thoughts::cli::Command as C;
    match cli.command {
        C::Init => {
            info!("initialising a new thought database...");
            Database::new(get_dir()).unwrap();
        }
    }
}
