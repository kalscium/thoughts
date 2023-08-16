use std::path::Path;
use std::fs;

pub fn wipe(path: impl AsRef<Path>) {
    println!("Wiping all thoughts...");
    let _ = fs::remove_file(path.as_ref().with_extension("ldb"));
    println!("Wiped all thoughts!");
    println!("Run `thoughts init` to re-init thoughts!");
}