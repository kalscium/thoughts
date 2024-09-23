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
}
