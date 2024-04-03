use crate::{
    blob::Blob,
    constants::{GIT_DIR, HEAD, OBJ_DIR, REF_DIR},
    git_object::GitObject,
};
use anyhow::{Ok, Result};
use std::{
    fs::{create_dir_all, write},
    io::Write,
    path::PathBuf,
};

pub struct Commands {}

impl Commands {
    /// Init .git directory
    pub fn init() -> Result<()> {
        create_dir_all(GIT_DIR).unwrap();
        create_dir_all(OBJ_DIR).unwrap();
        create_dir_all(REF_DIR).unwrap();
        write(HEAD, "ref: refs/heads/main\n").unwrap();

        println!("Initialized git directory");
        Ok(())
    }

    /// Read object from .git/objects.
    pub fn cat_file(oid: String, writer: &mut impl Write) -> Result<()> {
        let object = GitObject::from_oid(oid)?;
        writer.write_all(object.to_string().as_bytes())?;
        Ok(())
    }

    /// Write object to .git/objects
    pub fn hash_object(file: PathBuf, write: bool, writer: &mut impl Write) -> Result<()> {
        let blob = Blob::from_file(&file)?;
        if write {
            blob.write()?;
        }
        writer.write_all(blob.id.to_string().as_bytes())?;
        Ok(())
    }
}
