use anyhow::Result;
use clap::Parser;
use git_starter_rust::cli::{
    args::{Args, Commands},
    cat_file::cat_file,
    init::init,
};

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
