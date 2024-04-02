use anyhow::Result;

pub fn cat_file(_pretty: bool, _object_hash: String) -> Result<()> {
    println!("cat-file command executed!");
    Ok(())
}
