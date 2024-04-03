use crate::git_object::GitObject;
use crate::object_id::ObjectId;
use anyhow::Result;
use std::collections::BTreeSet;
use std::io::Write;

#[derive(Debug)]
pub struct Tree {
    pub id: ObjectId,
    pub entries: BTreeSet<TreeEntry>,
}

#[derive(Debug)]
pub struct TreeEntry {
    pub name: String,
    pub mode: String,
    pub object: GitObject,
}

impl Ord for TreeEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for TreeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TreeEntry {}

impl PartialEq for TreeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Tree {
    pub fn from_object(id: ObjectId, obj: Vec<u8>) -> Result<Self> {
        let mut object = &obj[..];
        let mut entries = BTreeSet::new();

        loop {
            if object.is_empty() {
                break;
            }

            // <mode> is the mode of the file/directory (check the previous section for valid values)
            // <name> is the name of the file/directory
            // \0 is a null byte
            // <20_byte_sha> is the 20-byte SHA-1 hash of the blob/tree (this is not in hexadecimal format)
            //  tree <size>\0
            //  <mode> <name>\0<20_byte_sha>
            //  <mode> <name>\0<20_byte_sha>
            let (mode, rest) = object.split_at(
                object
                    .iter()
                    .position(|x| *x == b' ')
                    .ok_or(anyhow::anyhow!("Corruption"))?,
            );
            let rest = &rest[1..];
            let (name, rest) = rest.split_at(
                rest.iter()
                    .position(|x| *x == b'\0')
                    .ok_or(anyhow::anyhow!("Corruption"))?,
            );
            let id = rest[1..].split_at(20).0;
            entries.insert(TreeEntry {
                mode: String::from_utf8(mode.into())?,
                name: String::from_utf8(name.into())?,
                object: GitObject::from_oid(hex::encode(id).try_into()?)?,
            });
            object = &rest[21..];
        }

        Ok(Self { id, entries })
    }

    pub fn to_string(&self, name_only: bool) -> String {
        let mut writer = Vec::new();
        if name_only {
            for entry in &self.entries {
                writeln!(writer, "{}", entry.name).unwrap();
            }
        } else {
            for entry in &self.entries {
                let (ty, id) = match &entry.object {
                    GitObject::Blob(blob) => ("blob", &blob.id),
                    GitObject::Tree(tree) => ("tree", &tree.id),
                };
                writeln!(writer, "{} {} {}\t{}", entry.mode, ty, id, entry.name).unwrap();
            }
        }
        String::from_utf8(writer).unwrap()
    }
}
