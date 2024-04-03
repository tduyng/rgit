use crate::blob::Blob;
use crate::git_object::GitObject;
use crate::object_id::ObjectId;
use anyhow::{Context, Result};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::collections::BTreeSet;
use std::fs::{create_dir_all, read_dir, write};
use std::io::Write;
use std::path::Path;

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

    pub fn from_directory(dir: &Path) -> Result<Self> {
        anyhow::ensure!(dir.is_dir(), "Not a directory: {:?}", dir);
        let mut entries = BTreeSet::new();

        for entry in read_dir(dir).context(format!("Failed to list {dir:?}"))? {
            let entry = entry?;
            let path = entry.path();
            let name = path
                .file_name()
                .unwrap_or_else(|| panic!("Failed to parse {path:?}"))
                .to_str()
                .expect("Filename is not valid UTF-8");

            if name == ".git" {
                continue; // Ingore .git directory
            }

            let metadata = entry.metadata()?;
            let is_dir = metadata.is_dir();
            let mode = if is_dir { "40000" } else { "100644" };
            let object = if is_dir {
                GitObject::Tree(Tree::from_directory(&path)?)
            } else {
                GitObject::Blob(Blob::from_file(&path)?)
            };

            entries.insert(TreeEntry {
                name: name.to_string(),
                mode: mode.to_string(),
                object,
            });
        }
        let content = Self::handle_entries(&entries, false);
        let header = format!("tree {}\0", content.len());
        let mut hasher = Sha1::new();
        hasher.update(header.as_bytes());
        hasher.update(&content);
        let digest = hasher.finalize();
        let oid = hex::encode(digest);
        let id = oid.try_into()?;

        Ok(Self { id, entries })
    }

    pub fn write(&self) -> Result<()> {
        let content = Self::handle_entries(&self.entries, true);
        let header = format!("tree {}\0", content.len());
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header.as_bytes())?;
        encoder.write_all(&content)?;

        let blob = encoder.finish()?;
        create_dir_all(self.id.dir())?;
        write(self.id.path(), blob)?;

        Ok(())
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

    fn handle_entries(entries: &BTreeSet<TreeEntry>, write: bool) -> Vec<u8> {
        let mut content = vec![];
        // <mode> <name>\0<20_byte_sha>
        for entry in entries {
            content.extend(entry.mode.as_bytes());
            content.extend([b' ']);
            content.extend(entry.name.as_bytes());
            content.extend([b'\0']);
            content.extend(entry.object.id().as_bytes());
            if write {
                entry.object.write().unwrap();
            }
        }
        content
    }
}
