use std::{
    fs::File,
    io::{BufReader, Read},
};

use anyhow::{ensure, Result};
use flate2::read::ZlibDecoder;

use crate::OBJ_DIR;

pub fn cat_file(pretty: bool, object_hash: String) -> Result<()> {
    ensure!(pretty, "");

    let folder = &object_hash[..2];
    let file = &object_hash[2..];
    let compressed =
        BufReader::new(File::open(format!("{}/{}/{}", OBJ_DIR, folder, file)).unwrap());
        
    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    let mut iter = decompressed.split('\0');
    iter.next().unwrap();
    let content = iter.next().unwrap();

    print!("{}", content);

    Ok(())
}
