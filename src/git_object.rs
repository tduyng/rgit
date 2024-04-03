use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, Read},
};

use crate::{blob::Blob, constants::OBJ_DIR, tree::Tree};
use anyhow::Result;
use flate2::read::ZlibDecoder;

#[derive(Debug)]
pub enum GitObject {
    Blob(Blob),
    Tree(Tree),
}

impl GitObject {
    pub fn from_oid(oid: String) -> Result<Self> {
        let path = format!("{}/{}/{}", OBJ_DIR, &oid[..2], &oid[2..]);
        let file = BufReader::new(File::open(path)?);
        let mut decoder = ZlibDecoder::new(file);
        let mut content = Vec::new();
        decoder.read_to_end(&mut content)?;

        let seperator = content.iter().position(|&x| x == b'\0').unwrap();
        let body = content.split_off(seperator + 1);

        let object = match content.as_slice() {
            h if h.starts_with(b"blob") => GitObject::Blob(Blob::from_object(oid, body)),
            h if h.starts_with(b"tree") => GitObject::Tree(Tree::from_object(oid, body)?),
            _ => return Err(anyhow::anyhow!("Invalid object")),
        };

        Ok(object)
    }
}

impl Display for GitObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitObject::Blob(blob) => write!(f, "{}", blob),
            GitObject::Tree(tree) => write!(f, "{}", tree.to_string(false)),
        }
    }
}
