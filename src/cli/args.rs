use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        object_hash: String,
    },
    HashObject {
        #[arg(short, long)]
        write: bool,
        file: PathBuf,
    },
}
