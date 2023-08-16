use crate::version::Version;

pub struct Config;
impl Config {
    pub const AUTHOR: &str = "GreenChild04";
    pub const VERSION: Version = Version::new(1, 0, 0);
    pub const VERSION_STRING: &str = "1.0.0";
    pub const ABOUT: &str = "A really simple CLI frontend to a database of my random thoughts.";
}