use std::{fs, path::Path};
use lazy_db::LazyDB;
use crate::{database::Database, info, unwrap, victor::Victor};

pub fn session(path: impl AsRef<Path>) {
    info("starting session...");
    let path = path.as_ref();
    if !path.is_dir() {
        crate::error::<()>("thought database not found! run `thoughts init` to initialise a new one.");
    }

    let mut database = unwrap!(Database::load(path));
    info("welcome to a space for random thoughts :D!");
    info("enter `(exit)`, CTRL+C or CTRL+D to exit the thought session");

    let mut rl = unwrap!(rustyline::DefaultEditor::new());
    loop {
        let line = rl.readline("\x1b[35m=>>\x1b[0m ");

        match line {
            Ok(line) => {
                if line.contains("(exit)") { break };
                if line.is_empty() { continue };
                unwrap!(database.push(&line));
            },
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(_) => continue,
        }
    }
}

pub fn init(path: impl AsRef<Path>) {
    info("initialising a new thought database...");
    unwrap!(Database::new(path));
}

pub fn import(path: impl AsRef<Path>, is_lazydb: bool, backup: impl AsRef<Path>) {
    let path = path.as_ref();
    crate::wipe::wipe(path);
    unwrap!(fs::create_dir_all(path));

    info("importing the backup...");

    if is_lazydb {
        info("detected that backup is a legacy `lazy-db`");
        let mut database = unwrap!(Database::new(path));
        
        let lazy_db = unwrap!(LazyDB::load_db(backup));
        let victor = unwrap!(Victor::load(unwrap!(lazy_db.as_container())));

        for thought in victor {
            unwrap!(database.push(thought.as_str()));
        } return;
    }

    unwrap!(fs::create_dir_all(path));
    unwrap!(fs::copy(backup, path.join("0")));
}
