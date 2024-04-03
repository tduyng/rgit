use crate::constants::OBJ_DIR;
use anyhow::Ok;
use std::{fmt::Display, path::PathBuf};

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
