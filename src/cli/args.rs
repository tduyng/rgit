use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rit")]
#[command(version, about = "Git CLI written in Rust", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Init a .git folder")]
    Init,

    #[command(about = "Read a blob object")]
    CatFile {
        #[arg(short, long, default_value = "true", help = "Pretty print?")]
        pretty: bool,
        object_hash: String,
    },
    #[command(about = "Create a blob object")]
    HashObject {
        #[arg(short, long, help = "Write object hash?")]
        write: bool,
        file: PathBuf,
    },
}
