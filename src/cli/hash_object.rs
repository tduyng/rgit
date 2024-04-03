use anyhow::Result;
use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::Path};

use crate::constants::OBJ_DIR;

pub fn exec(path: &Path) -> Result<String> {
    let mut hasher = Sha1::new();
    let content = fs::read(path)?;
    let header = format!("blob {}\0", content.len());
    hasher.update(&header);
    hasher.update(&content);
    let digest: &[u8] = &hasher.finalize()[..];

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(header.as_bytes())?;
    encoder.write_all(&content)?;
    let compressed = encoder.finish()?;

    let encode = hex::encode(digest);
    let folder = &encode[..2];
    let file = &encode[2..];
    let path = format!("{}/{}/{}", OBJ_DIR, folder, file);
    fs::create_dir_all(format!("{}/{}", OBJ_DIR, folder))?;
    fs::write(path, compressed)?;

    Ok(encode)
}
