use crate::object_id::ObjectId;
use anyhow::{Ok, Result};
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fmt::Display, fs, io::Write, path::Path};

#[derive(Debug)]
pub struct Blob {
    pub id: ObjectId,
    pub blob: Vec<u8>,
}

impl Blob {
    pub fn from_object(oid: ObjectId, object: Vec<u8>) -> Self {
        Self {
            id: oid,
            blob: object,
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let mut hasher = Sha1::new();
        let content = fs::read(path)?;
        let header = format!("blob {}\0", content.len());
        hasher.update(&header);
        hasher.update(&content);

        let digest: &[u8] = &hasher.finalize()[..];
        let oid = hex::encode(digest);

        Ok(Self {
            id: oid.try_into()?,
            blob: content,
        })
    }

    pub fn write(&self) -> Result<()> {
        let header = format!("blob {}\0", self.blob.len());

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes())?;
        encoder.write_all(&self.blob)?;
        let blob = encoder.finish()?;

        fs::create_dir_all(self.id.dir())?;
        fs::write(self.id.path(), blob)?;

        Ok(())
    }
}

impl Display for Blob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.blob))
    }
}
