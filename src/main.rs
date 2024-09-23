use std::fs;

use chrono::Utc;
use clap::Parser;
use log::{error, info};
use thoughts::{cli::Cli, database::Database, get_dir, log::Logger, thought::Thought};

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
        C::Today => session(),
        C::Push { thought } => push_thought(thought, &mut Database::load(get_dir()).expect("database corrupt or non-existent")),
    }
}

/// Starts a random thought session
fn session() {
    info!("starting thought session...");
    if !get_dir().is_dir() {
        error!("thought database not found!");
        info!("run `thoughts init` to initialise a new one");
        std::process::exit(1);
    }

    // get the database
    let mut database = Database::load(get_dir()).expect("database corrupt");

    info!("welcome to a space for random thoughts :D!");
    info!("enter `(exit)`, CTRL+C or CTRL+D to exit the thought session");

    // start the actual session
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    loop {
        let line = rl.readline("\x1b[35m=>>\x1b[0m ");

        match line {
            Ok(line) => {
                if line.contains("(exit)") { break };
                if line.is_empty() { continue };

                push_thought(line, &mut database);
            }
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(_) => continue,
        }
    }
}

/// Pushes a thought to the thought database
fn push_thought(thought: String, database: &mut Database) {
    // construct the thought and serialize it
    let thought = Thought(thought, Some(Utc::now()));
    let bytes = bincode::serialize(&thought).unwrap();

    // push it
    database.push(&bytes).expect("failed to push thought into thought database");
}
