use std::io::{BufWriter, Write};
use std::path::Path;
use std::fs::{self, File};
use crate::database::Database;
use crate::{info, unwrap};

pub fn export(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    info(&format!("exporting thoughts as `{}`...", output.as_ref().to_string_lossy()));
    let database = unwrap!(Database::load(path));

    let mut file = BufWriter::new(unwrap!(File::create(output)));
    unwrap!(file.write("# Thoughts :D\n---\n".as_bytes()));

    for thought in database {
        let thought = format!("- {thought}\n");
        unwrap!(file.write(thought.as_bytes()));
    }

    info("successfully exported thoughts!")
}

pub fn backup(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    info(&format!("backing-up thoughts as `{}`...", output.as_ref().to_string_lossy()));
    let path = path.as_ref();
    let mut database = Database::load(path).unwrap();

    unwrap!(database.stackdb.rebase(1024 * 1024)); // backup limit of 1MiB
    unwrap!(fs::copy(path.join("0"), output));

    info("successfully backed-up thoughts!")
}
