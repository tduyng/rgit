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
    #[command(about = "Read a tree object")]
    LsTree {
        #[arg(short, long, help = "Display name only?")]
        name_only: bool,
        oid: String,
    },
    #[command(about = "Write a tree object")]
    WriteTree,
    #[command(about = "Write a commit")]
    CommitTree {
        #[arg(short, long)]
        message: String,
        #[arg(short, long)]
        parent: String,
        tree: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Command::Init => Commands::init(),
        Command::CatFile { pretty: _, oid } => {
            Commands::cat_file(oid.try_into()?, &mut io::stdout())
        }
        Command::HashObject { write, file } => {
            Commands::hash_object(file, write, &mut io::stdout())
        }
        Command::LsTree { name_only, oid } => {
            Commands::ls_tree(oid.try_into()?, name_only, &mut io::stdout())
        }
        Command::WriteTree => Commands::write_tree(&mut io::stdout()),
        Command::CommitTree {
            message,
            parent,
            tree,
        } => Commands::write_commit(message, Some(parent), tree, &mut io::stdout()),
    }
}
