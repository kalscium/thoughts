use std::io::{BufWriter, Write};
use std::path::Path;
use std::fs::{self, File};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::database::Database;
use crate::{info, unwrap};

pub fn export(path: impl AsRef<Path>, markdown: bool, output: impl AsRef<Path>) {
    if markdown {
        export_markdown(path, output);
    } else {
        export_ron(path, output);
    }
}

#[derive(Serialize)]
pub struct Thought(String, Option<DateTime<Utc>>);

fn export_ron(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    info(&format!("exporting thoughts as `{}`...", output.as_ref().to_string_lossy()));
    let database = unwrap!(Database::load(path));

    // collect and generate ron
    let ron = database.into_iter()
        .map(|thought| Thought(thought, None))
        .collect::<Vec<_>>();
    let ron = unwrap!(ron::to_string(&ron));

    // write the outputed ron to the file
    unwrap!(fs::write(output, ron.as_bytes()));
}

fn export_markdown(path: impl AsRef<Path>, output: impl AsRef<Path>) {
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
