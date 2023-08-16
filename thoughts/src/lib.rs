pub mod cli;
pub mod config;
pub mod version;
pub mod victor;
pub mod prompt;

use std::path::PathBuf;

#[inline]
pub fn get_path() -> PathBuf {
    #[allow(deprecated)]
    let home = std::env::home_dir().unwrap();
    home.join(".thoughts/database")
}