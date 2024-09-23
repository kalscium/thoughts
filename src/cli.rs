use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about="Initialises a new database of thoughts")]
    Init,
    #[clap(about="Starts a random thought session for today")]
    Today,
    #[clap(about="Appends a single random thought")]
    Push {
        #[clap(index=1)]
        thought: String,
    },
    #[clap(about="Wipes all thoughts permanantly")]    
    Wipe {
        #[clap(long, required=true)]
        i_know_what_i_am_doing: bool,
    },
    #[clap(about="Exports your thoughts as either RON or markdown")]
    Export {
        #[clap(short, long, help="If you want to export it as markdown instead of ron")]
        markdown: bool,
        #[clap(index=1, help="The path of the file you want to export as")]
        path: String,
    },
    #[clap(about="Imports a RON version of your thoughts and combnies it with your existing thought database")]
    Import {
        #[clap(index=1, help="The path of the RON thoughts you want to import")]
        path: String,
    },
    #[clap(about="Compacts the thoughts database to make it more compact and storage efficient")]
    Compact,
}
