use lazy_db::*;
use promptly::prompt;
use std::path::Path;
use crate::victor::Victor;

pub fn session(path: impl AsRef<Path>) {
    let path = path.as_ref();
    let database = LazyDB::load_db(path).unwrap();
    let mut victor = Victor::load(database.as_container().unwrap()).unwrap();
    
    loop {
        let input: String = prompt("--> ").unwrap();
        if input.contains("(exit)") { break }
        let _ = victor.push(&input);
    }
}

pub fn init(path: impl AsRef<Path>) {
    let path = path.as_ref();
    std::fs::create_dir_all(path).unwrap();
    LazyDB::init_db(path).unwrap();
}