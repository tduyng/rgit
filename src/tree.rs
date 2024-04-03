use crate::blob::Blob;
use crate::object::{GitObject, ObjectId};
use anyhow::{Context, Result};
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::collections::BTreeSet;
use std::fs::{create_dir_all, read_dir, rename, File};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Tree {
    pub id: ObjectId,
    pub entries: BTreeSet<TreeEntry>,
}

#[derive(Debug)]
pub struct TreeEntry {
    pub id: ObjectId,
    pub name: String,
    pub mode: String,
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
    pub fn from_object(id: ObjectId, obj: impl Read) -> Result<Self> {
        let obj = obj
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .context(format!("Reading tree object {id}"))?;
        let mut object = obj.as_slice();
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
                id: hex::encode(id).try_into()?,
            });
            object = &rest[21..];
        }

        Ok(Self { id, entries })
    }

    pub fn write(dir: &Path) -> Result<ObjectId> {
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
            let id = if is_dir {
                Tree::write(&path)?
            } else {
                Blob::write(&path, true)?
            };

            entries.insert(TreeEntry {
                name: name.to_string(),
                mode: mode.to_string(),
                id,
            });
        }
        let content = Self::handle_entries(&entries);
        let tmp_dir = tempfile::tempdir()?;
        let tmp_file = tmp_dir.path().join("tempfile");
        let encoder =
            flate2::write::ZlibEncoder::new(File::create(&tmp_file)?, Compression::default());

        let header = format!("tree {}\0", content.len());
        let mut hasher = WriteHasher::new(encoder);
        hasher.write_all(header.as_bytes())?;
        hasher.write_all(&content)?;
        let id = hasher.finalize();
        create_dir_all(id.dir())?;
        rename(tmp_file, id.path())?;

        Ok(id)
    }

    pub fn to_string(&self, name_only: bool) -> Result<String> {
        let mut writer = Vec::new();
        if name_only {
            for entry in &self.entries {
                writeln!(writer, "{}", entry.name)?;
            }
        } else {
            for entry in &self.entries {
                let object = GitObject::from_oid(entry.id.clone())?;
                let ty = match object {
                    GitObject::Blob(_) => "blob",
                    GitObject::Tree(_) => "tree",
                };
                writeln!(writer, "{} {} {}\t{}", entry.mode, ty, entry.id, entry.name).unwrap();
            }
        }

        Ok(String::from_utf8(writer)?)
    }

    fn handle_entries(entries: &BTreeSet<TreeEntry>) -> Vec<u8> {
        let mut content = vec![];
        // <mode> <name>\0<20_byte_sha>
        for entry in entries {
            content.extend(entry.mode.as_bytes());
            content.extend([b' ']);
            content.extend(entry.name.as_bytes());
            content.extend([b'\0']);
            content.extend(entry.id.as_bytes());
        }
        content
    }
}

pub struct WriteHasher<W> {
    writer: W,
    hasher: Sha1,
}

impl<W> WriteHasher<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            hasher: Sha1::new(),
        }
    }

    pub fn finalize(self) -> ObjectId {
        let digest = self.hasher.finalize();
        let oid = hex::encode(digest);
        oid.try_into().expect("Oid is not a hex string")
    }
}

impl<W> Write for WriteHasher<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.hasher.update(&buf[..len]);
        Ok(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
