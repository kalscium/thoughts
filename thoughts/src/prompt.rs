use lazy_db::*;
use std::path::Path;
use crate::victor::Victor;
use crate::ask;

pub fn session(path: impl AsRef<Path>) {
    println!("Starting Session...");
    let path = path.as_ref();
    let database = match LazyDB::load_db(path.with_extension("ldb")) {
        Ok(x) => x,
        Err(e) => match e {
            LDBError::FileNotFound(_) => crate::error("thought database not found! run `thoughts init` to initialise a new one."),
            _ => Err(e).unwrap(),
        },
    };
    let mut victor = Victor::load(database.as_container().unwrap()).unwrap();
    
    println!("Welcome to a space for random thoughts :D!");
    println!("Enter `(exit)` to exit the thought session.");
    loop {
        let input: String = ask("\x1b[35m-->\x1b[0m ");
        if input.contains("(exit)") { break }
        if input.is_empty() { break }
        let _ = victor.push(&input);
    }
}

pub fn init(path: impl AsRef<Path>) {
    println!("Initialising a new database...");
    let path = path.as_ref();
    let db = LazyDB::init_db(path).unwrap();
    Victor::new(db.as_container().unwrap()).unwrap();
    println!("Initialised a new thought database!");
}