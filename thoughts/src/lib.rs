use std::path::PathBuf;

pub mod cli;
pub mod victor;
pub mod database;
pub mod prompt;
pub mod export;
pub mod wipe;

#[inline]
pub fn error<T>(msg: &str) -> T {
    eprintln!("\x1b[31;1merror\x1b[0m {msg}");
    std::process::exit(0);
}

#[inline]
pub fn info(msg: &str) {
    println!("\x1b[36;1minfo\x1b[0m  {msg}");
}

#[macro_export]
macro_rules! unwrap {
    ($msg:expr) => {
        match $msg {
            Ok(x) => x,
            Err(e) => $crate::error(&format!("{e:?}")),
        }
    }
}

#[inline]
pub fn get_dir() -> PathBuf {
    home::home_dir()
        .unwrap_or(PathBuf::new())
        .join(".thoughts-cli")
}
