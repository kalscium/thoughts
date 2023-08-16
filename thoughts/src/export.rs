use lazy_db::*;
use crate::victor::Victor;
use std::path::Path;
use std::fs::File;

pub fn export(path: impl AsRef<Path>, output: impl AsRef<Path>) {
    println!("Exporting thoughts as `{}`...", output.as_ref().to_string_lossy());
    let path = path.as_ref();
    let database = match LazyDB::load_db(path.with_extension("ldb")) {
        Ok(x) => x,
        Err(e) => match e {
            LDBError::FileNotFound(_) => crate::error("thought database not found! run `thoughts init` to initialise a new one."),
            _ => Err(e).unwrap(),
        },
    };
    let victor = Victor::load(database.as_container().unwrap()).unwrap();

    let mut file = FileWrapper::new_writer(File::create(output)
        .unwrap_or_else(|_| crate::error("couldn't export to file due to io error")));
    let _ = file.write("# Thoughts :D\n---\n".as_bytes());
    for thought in victor {
        let thought = format!("- {thought}");
        let _ = file.write(thought.as_bytes());
    }

    println!("Successfully exported thoughts!")
}