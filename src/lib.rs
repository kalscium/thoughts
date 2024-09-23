pub mod log;
pub mod database;
pub mod thought;
pub mod cli;
pub mod session;

pub fn get_dir() -> std::path::PathBuf {
    home::home_dir()
        .expect("could not find home directory")
        .join(".thoughts-cli")
}
