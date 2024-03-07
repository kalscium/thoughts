use std::path::Path;
use std::fs;
use crate::info;

pub fn wipe(path: impl AsRef<Path>) {
    info("wiping all thoughts...");
    let _ = fs::remove_dir_all(path.as_ref());
    info("run `thoughts init` to re-init thoughts!");
}
