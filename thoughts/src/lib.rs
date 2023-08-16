pub mod cli;
pub mod config;
pub mod version;
pub mod victor;
pub mod prompt;
pub mod export;

use std::path::PathBuf;

#[inline]
pub fn get_path() -> PathBuf {
    #[allow(deprecated)]
    let home = std::env::home_dir().unwrap();
    home.join(".thoughts/thoughts")
}

#[inline]
pub fn error<T>(msg: &str) -> T {
    eprintln!("\x1b[35m[\x1b[31merror\x1b[35m]\x1b[0m {msg}");
    std::process::exit(0);
}

#[inline]
pub fn ask(prompt: &str) -> String {
    use std::io::{stdout, stdin, Write};
    let mut input = String::new();
    print!("{prompt}");
    let _ = stdout().flush();
    let _ = stdin().read_line(&mut input);
    input
}