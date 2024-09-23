use std::{fs::{self, File}, io::Write};
use chrono::{DateTime, Datelike, Local, Utc};
use log::{info, warn};

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

    // write the title & initialise the 'last time' variable
    file.write_all("# Thoughts :D\n---\n".as_bytes()).unwrap();
    let mut last: Option<DateTime<Utc>> = None;

    // write the entries
    for bytes in database {
        // deserialize the thought
        let thought = bincode::deserialize(&bytes).expect("thought database is corrupt");

        // extract the thought and time
        let Thought(thought, time) = thought;

        // if there is a timestamp then check it
        // diff day, print date
        // if it's been longer than an 30 mins, print time
        if let Some(time) = time {
            // get the last time or otherwise use a generic time
            let last = last.unwrap_or(DateTime::from_timestamp_nanos(0));
            
            // check the day or month or year
            if last.day() != time.day() || last.month() != time.month() || last.year() != time.year() {
                let time: DateTime<Local> = DateTime::from(time);

                // format the date
                let format = &format!(
                    "%A, %-d{} of %B %Y, `%I:%M %p`",
                    // get the suffix (may replace later with better alternative)
                    match time.day() {
                        t if t % 10 == 1 && t % 100 != 11 => "st",
                        t if t % 10 == 2 && t % 100 != 12 => "nd",
                        t if t % 10 == 3 && t % 100 != 13 => "rd",
                        _ => "th",
                    }
                );
                let date = format!("## {}\n", time.format(format));

                // write the formatted date
                file.write_all(date.as_bytes()).unwrap();
            } else if (time.time() - last.time()).num_minutes() > 30 { // check if it's within 30 minutes
                // format the time and write it
                let time: DateTime<Local> = DateTime::from(time);
                let time = time.format("`%I:%M %p`");
                let time = format!("#### {time}\n");

                // write the formatted time
                file.write_all(time.as_bytes()).unwrap();
            }
        }

        // update last
        last = time;

        // write the thought to the file
        let thought = format!("- {thought}\n");
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

/// Imports RON thoughts and combines it with the current existing database
pub fn import(path: &str) {
    info!("importing RON thoughts and combining with current database...");

    // get the current thoughts from the database if it exists, otherwise create an empty vector
    let mut thoughts = if get_dir().exists() {
        info!("thoughts database found");
        Database::load(get_dir()).unwrap()
            .map(|bytes| {
                // deserialize the thought
                bincode::deserialize(&bytes)
                    .expect("thoughts database is corrupt")
            })
            .collect::<Vec<Thought>>()
    } else {
        warn!("thoughts database not found, initialising a new one");
        Vec::new()
    };

    // get the RON thoughts
    #[allow(clippy::expect_fun_call)]
    let ron_thoughts: Vec<Thought> = ron::from_str(&fs::read_to_string(path).expect(&format!("while reading the contents of `{path}`"))).expect("RON thoughts are corrupt");

    // combine the thoughts and remove duplicates
    for thought in ron_thoughts.into_iter() {
        if !thoughts.contains(&thought) {
            thoughts.push(thought);
        }
    }

    // sort the thoughts by time
    thoughts.sort_by_key(|thought| thought.1.map(|utc| utc.timestamp_millis()).unwrap_or(0));

    // write the resulting thoughts to a new database
    let _ = fs::remove_dir_all(get_dir());
    let mut database = Database::new(get_dir()).expect("while initialising database");
    for thought in thoughts.into_iter() {
        database.push(
            &bincode::serialize(&thought).unwrap()
        ).expect("while writing to thought database (warning: major data loss)");
    } database.commit().expect("while writing to thought database (warning: major data loss)");

    info!("successfully imported RON thoughts!");
}
