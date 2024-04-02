use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    CatFile {
        #[arg(short, long)]
        pretty: bool,
        #[arg(short, long)]
        object_hash: String,
    },
}
