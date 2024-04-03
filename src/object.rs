use crate::blob::Blob;
use crate::constants::OBJ_DIR;
use crate::tree::Tree;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use flate2::read::ZlibDecoder;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectId(String);

impl ObjectId {
    pub fn new(oid: &str) -> Self {
        oid.try_into().expect("Object id is not valid")
    }

    pub fn path(&self) -> PathBuf {
        format!("{}/{}/{}", OBJ_DIR, &self.0[..2], &self.0[2..]).into()
    }

    pub fn dir(&self) -> PathBuf {
        format!("{}/{}/", OBJ_DIR, &self.0[..2]).into()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        hex::decode(&self.0).expect("oid is not a hex string")
    }
}

impl TryFrom<&str> for ObjectId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 40 {
            return Err(anyhow::anyhow!("Invalid object id"));
        }

        Ok(ObjectId(value.to_string()))
    }
}

impl TryFrom<String> for ObjectId {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 40 {
            return Err(anyhow::anyhow!("Invalid object id"));
        }

        Ok(ObjectId(value))
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum GitObject<R> {
    Blob(Blob<R>),
    Tree(Tree),
}

impl GitObject<()> {
    pub fn from_oid(oid: ObjectId) -> Result<GitObject<impl Read>> {
        let path = oid.path();
        let file = File::open(&path)?;
        let mut decoder = BufReader::new(ZlibDecoder::new(file));
        let mut header = Vec::new();
        decoder
            .read_until(0, &mut header)
            .context(format!("uncompressing {path:?}"))?;
        header.truncate(header.len() - 1);
        let header = String::from_utf8(header).context(format!("parsing header of {oid}"))?;
        let (ty, len) = header
            .split_once(' ')
            .context(format!("header of {oid} has invalid format"))?;
        let reader = decoder.take(len.parse().context(format!("parsing length of {oid}"))?);
        let object = match ty {
            "blob" => GitObject::Blob(Blob::from_object(oid, reader)),
            "tree" => GitObject::Tree(Tree::from_object(oid, reader)?),
            _ => anyhow::bail!(format!("Unknown object type {ty}")),
        };
        Ok(object)
    }
}
