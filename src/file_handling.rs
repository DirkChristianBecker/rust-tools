use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Errors that might occur while handling files.
#[derive(Debug, PartialEq)]
pub enum FileErrors {
    FileNotFound,
    CorruptFile
}

/// Read a file as a byte vector. This function should help reduce boilerplate code.
pub fn get_file_as_byte_vec(filename: &Path) -> Result<Vec<u8>, FileErrors> {
    let f = File::open(filename);
    let mut file : File;
    match f {
        Ok(r) => { file = r; }
        Err(_) => { return Err(FileErrors::FileNotFound); }
    }

    let mut buffer = Vec::new();

    let r = file.read_to_end(&mut buffer);
    match r {
        Ok(_) => {},
        Err(_) => { return Err(FileErrors::CorruptFile); },
    }

    Ok(buffer)
}