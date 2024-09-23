use std::{fs::{self, File}, io::Write};

use log::info;

use crate::{database::Database, get_dir, thought::Thought};

/// Exports a thought database as either markdown or RON
pub fn export(markdown: bool, path: &str) {
    if markdown {
        export_markdown(path);
    } else {
        export_ron(path);
    }
}

fn export_markdown(path: &str) {
    info!("exporting thoughts as markdown (`{path}`)...");

    // get both the database and output file
    let database = Database::load(get_dir()).expect("database either corrupt or non-existent");
    let mut file = File::create(path).unwrap();

    // write the title
    file.write_all("# Thoughts :D\n---\n".as_bytes()).unwrap();

    // write the entries
    for bytes in database {
        // deserialize the thought
        let thought = bincode::deserialize(&bytes).expect("thought database is corrupt");

        // extract the thought and time
        let Thought(thought, _) = thought;

        // write the thought to the file
        file.write_all(thought.as_bytes()).unwrap();
    }

    // flush the file
    file.flush().unwrap();
    info!("successfully export thoughts as markdown!");
}

fn export_ron(path: &str) {
    info!("exporting thoughts as RON (`{path}`)...");

    // get database
    let database = Database::load(get_dir()).expect("database either corrupt or non-existent");

    // collect and generate ron
    let ron = database.into_iter()
        .map(|bytes| {
            // deserialize thought
            bincode::deserialize::<Thought>(&bytes).expect("thought database is corrupt")
        })
        .collect::<Vec<_>>();
    let ron = ron::to_string(&ron).unwrap();

    // write the generated ron to the file
    fs::write(path, ron).unwrap();
    info!("successfully export thoughts as RON!");
}
