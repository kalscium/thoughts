use clap::{Parser, Subcommand};
use crate::database::Database;
use crate::get_dir;
use crate::prompt;
use crate::export;
use crate::unwrap;
use crate::wipe;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about="To start today's random thought session")]
    Today,
    #[clap(about="To initialise a new database of thoughts")]
    Init,
    #[clap(about="To export your thoughts as a markdown document")]
    Export {
        #[clap(short, long, help="If you want to export it as markdown instead of ron")]
        markdown: bool,
        #[clap(index=1, help="The location of the file you want to export as")]
        file: String,
    },
    #[clap(about="Wipes all thoughts permanantly")]
    Wipe,
    #[clap(about="Compacts the thoughts database to make it more compact and storage efficient")]
    Compact,
    #[clap(about="Backs up the thoughts database to the specified location")]
    Backup {
        #[clap(index=1, help="The location to backup to as")]
        file: String,
    },
    #[clap(about="Imports a backup of the thoughts database")]
    Import {
        #[clap(short='l', long, help="If the backup is of a legacy version (`lazy-db` datbase)")]
        is_lazydb: bool,
        #[clap(index=1, help="The location of backup")]
        file: String,
    }
}

impl Commands {
    pub fn execute(&self) {
        use Commands as C;
        match self {
            C::Today => prompt::session(get_dir()),
            C::Init => prompt::init(get_dir()),
            C::Export { markdown, file } => export::export(get_dir(), *markdown, file),
            C::Wipe => wipe::wipe(get_dir()),
            C::Compact => {
                let mut database = unwrap!(Database::load(get_dir()));
                unwrap!(database.stackdb.rebase(1024)); // compaction of only 1KiB
            },
            C::Backup { file } => export::backup(get_dir(), file),
            C::Import { is_lazydb, file } => prompt::import(get_dir(), *is_lazydb, file),
        }
    }
}
