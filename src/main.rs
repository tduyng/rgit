use anyhow::Result;
use clap::Parser;
use git_starter_rust::cli::{
    args::{Args, Commands},
    cat_file, hash_object, init,
};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Init => init::exec(),
        Commands::CatFile {
            pretty,
            object_hash,
        } => cat_file::exec(pretty, object_hash),
        Commands::HashObject { write, file } => {
            assert!(write, "Only write mode is supported!");
            let oid = hash_object::exec(&file)?;
            print!("{}", oid);
            Ok(())
        }
    }
}
