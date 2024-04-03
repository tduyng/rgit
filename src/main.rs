use anyhow::Result;
use clap::{Parser, Subcommand};
use git_starter_rust::commands::Commands;
use std::{io, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about = "Git CLI written in Rust", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Init a .git folder")]
    Init,

    #[command(about = "Read a blob object")]
    CatFile {
        #[arg(short, long, default_value = "true", help = "Pretty print?")]
        pretty: bool,
        oid: String,
    },
    #[command(about = "Create a blob object")]
    HashObject {
        #[arg(short, long, help = "Write object hash?")]
        write: bool,
        file: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Command::Init => Commands::init(),
        Command::CatFile { pretty: _, oid } => Commands::cat_file(oid, &mut io::stdout()),
        Command::HashObject { write, file } => {
            Commands::hash_object(file, write, &mut io::stdout())
        }
    }
}
