use std::fs;
use clap::Parser;
use log::info;
use thoughts::{cli::Cli, database::Database, get_dir, log::Logger, session};

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
        },
        C::Wipe { .. } => {
            info!("wiping all thoughts...");
            let _ = fs::remove_dir_all(get_dir());
            info!("run `thoughts init` to re-init thoughts!");
        },
        C::Today => session::session(),
        C::Push { thought } => session::push_thought(thought, &mut Database::load(get_dir()).expect("database corrupt or non-existent")),
        C::Export { markdown, path } => thoughts::port::export(markdown, &path),
        C::Import { path } => thoughts::port::import(&path),
        C::Compact => {
            info!("compacting thoughts database...");
            let mut database = Database::load(get_dir()).expect("failed to load thoughts database");
            database.stackdb.rebase(1024 * 16).expect("failed to rebase thoughts database"); // compaction of only 16KiB
            info!("successfully compacted thoughts database!");
        },
    }
}
