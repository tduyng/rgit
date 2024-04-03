use crate::constants::{GIT_DIR, HEAD, OBJ_DIR, REF_DIR};
use anyhow::Result;
use std::fs::{create_dir_all, write};

pub fn init() -> Result<()> {
    create_dir_all(GIT_DIR).unwrap();
    create_dir_all(OBJ_DIR).unwrap();
    create_dir_all(REF_DIR).unwrap();
    write(HEAD, "ref: refs/heads/main\n").unwrap();

    println!("Initialized git directory");
    Ok(())
}
