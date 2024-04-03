use crate::{object::ObjectId, tree::WriteHasher};
use anyhow::Result;
use flate2::Compression;
use std::{
    fs::{create_dir_all, rename, File},
    io::Write,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Commit {}

impl Commit {
    pub fn write(message: String, parent: Option<ObjectId>, tree: ObjectId) -> Result<ObjectId> {
        let mut body = Vec::new();
        writeln!(body, "tree {}", tree)?;
        if let Some(parent) = parent {
            writeln!(body, "parent {parent}")?;
        }

        let seconds = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        writeln!(body, "author tduyng<tduyng@gmail.com> {seconds} +0000")?;
        writeln!(body, "commiter tduyng<tduyng@gmail.com> {seconds} +0000")?;
        writeln!(body, "\n{message}")?;
        let header = format!("commit {}\0", body.len());
        let tmp_dir = tempfile::tempdir()?;
        let tmp_file = tmp_dir.path().join("tempfile");

        let encoder =
            flate2::write::ZlibEncoder::new(File::create(&tmp_file)?, Compression::default());
        let mut hasher = WriteHasher::new(encoder);
        hasher.write_all(header.as_bytes())?;
        hasher.write_all(&body)?;
        let id = hasher.finalize();
        create_dir_all(id.dir())?;
        rename(tmp_file, id.path())?;

        Ok(id)
    }
}
