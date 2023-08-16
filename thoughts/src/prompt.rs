use lazy_db::*;
use promptly::prompt;
use std::path::Path;
use crate::victor::Victor;

pub fn session(path: impl AsRef<Path>) {
    println!("Starting Session...");
    let path = path.as_ref();
    let database = LazyDB::load_db(path).unwrap();
    let mut victor = Victor::load(database.as_container().unwrap()).unwrap();
    
    println!("Welcome to a space for random thoughts :D!");
    loop {
        let input: String = prompt("--> ").unwrap();
        if input.contains("(exit)") { break }
        let _ = victor.push(&input);
    }
}

pub fn init(path: impl AsRef<Path>) {
    println!("Initialising a new database...");
    let path = path.as_ref();
    LazyDB::init_db(path).unwrap();
    println!("Initialised a new thought database!")
}