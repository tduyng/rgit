use crate::constants::OBJ_DIR;
use anyhow::{ensure, Result};
use flate2::read::ZlibDecoder;
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn cat_file(pretty: bool, object_hash: String) -> Result<()> {
    ensure!(pretty, "");

    let folder = &object_hash[..2];
    let file = &object_hash[2..];
    let compressed =
        BufReader::new(File::open(format!("{}/{}/{}", OBJ_DIR, folder, file)).unwrap());

    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    let mut iter = decompressed.split('\0'); // split by `\0` byte
    iter.next().unwrap(); // ignore `blob`
    let content = iter.next().unwrap();

    print!("{}", content);

    Ok(())
}
