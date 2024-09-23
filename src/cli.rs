use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(about="To initialise a new database of thoughts")]
    Init,
    #[clap(about="Wipes all thoughts permanantly")]    
    Wipe {
        #[clap(long, required=true)]
        i_know_what_i_am_doing: bool,
    },
}
