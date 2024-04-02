use anyhow::Result;
use clap::Parser;
use git_starter_rust::cat_file;
use git_starter_rust::{init, Args, Commands};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Init => init(),
        Commands::CatFile {
            pretty,
            object_hash,
        } => cat_file(pretty, object_hash),
    }
}
