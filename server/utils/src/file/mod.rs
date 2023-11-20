use std::io::prelude::*;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

enum FileMode {
    Append,
    Overwrite,
}

fn file_put_contents(filename: &str, content: &[u8], mode: FileMode) -> Result<(), std::io::Error> {
    let mut file = match mode {
        FileMode::Append => OpenOptions::new().write(true).append(true).create(true).open(filename)?,
        FileMode::Overwrite => File::create(filename)?,
    };
    file.write_all(content)?;
    Ok(())
}

pub fn file_get_contents(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}