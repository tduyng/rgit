use anyhow::{Context, Ok, Result};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{
    fmt::Debug,
    fs::{create_dir_all, rename, File},
    io::{empty, BufReader, Read, Write},
    path::Path,
};
use tempfile;

use crate::object::ObjectId;

pub struct Blob<R> {
    pub id: ObjectId,
    pub blob: R,
}

impl<R> Blob<R>
where
    R: Read,
{
    pub fn from_object(oid: ObjectId, blob: R) -> Self {
        Self { id: oid, blob }
    }

    pub fn copy(mut self, writer: &mut impl Write) -> Result<()> {
        std::io::copy(&mut self.blob, writer)?;
        Ok(())
    }
}

impl Blob<()> {
    pub fn write(path: &Path, write: bool) -> Result<ObjectId> {
        let file = File::open(path).context(format!("Opening file {path:?}"))?;
        let len = file.metadata()?.len();
        let header = format!("blob {}\0", len);
        let body = BufReader::new(file);
        let mut reader = ReadHasher::new(header.as_bytes().chain(body));

        if write {
            let tmp_dir = tempfile::tempdir()?;
            let tmp_path = tmp_dir.path().join("tempfile");
            let file = File::create(&tmp_path)
                .context(format!("Creating temporary file {:?}", tmp_path))?;
            let mut encoder = ZlibEncoder::new(file, Compression::default());
            std::io::copy(&mut reader, &mut encoder)?;
            encoder.finish().context(format!("Encoding {path:?}"))?;
            let id = reader.finalize();
            create_dir_all(id.dir()).context(format!("Creating directory {:?}", id.dir()))?;
            rename(&tmp_path, id.path()).context(format!(
                "Renaming temporary file from {:?} to {:?}",
                tmp_path,
                id.path()
            ))?;

            Ok(id)
        } else {
            std::io::copy(&mut reader, &mut empty())?;
            Ok(reader.finalize())
        }
    }
}

impl<R> Debug for Blob<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Blob")
            .field("id", &self.id)
            .field("blob", &"...")
            .finish()
    }
}

#[derive(Debug)]
struct ReadHasher<R> {
    reader: R,
    hasher: Sha1,
}

impl<R> Read for ReadHasher<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let len = self.reader.read(buf)?;
        self.hasher.update(&buf[..len]);
        Result::Ok(len)
    }
}

impl<R> ReadHasher<R>
where
    R: Read,
{
    fn new(reader: R) -> Self {
        Self {
            reader,
            hasher: Sha1::new(),
        }
    }

    fn finalize(self) -> ObjectId {
        let digest = self.hasher.finalize();
        let oid = hex::encode(digest);
        oid.try_into().unwrap()
    }
}
