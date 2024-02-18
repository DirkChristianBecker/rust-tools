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

#[cfg(test)]
mod tests {
    use super::FileErrors;
    use super::get_file_as_byte_vec;
    use std::env;

    #[test]
    fn test_get_file_as_bytes_no_error() {
        if let Ok(mut path) = env::current_dir() {
            path.push("src");
            path.push("tests");
            path.push("bytes.txt");

            let r = get_file_as_byte_vec(path.as_path());
            match r {
                Ok(file) => {
                    assert_eq!(file, "test".as_bytes())
                }

                Err(e) => {
                    println!("File under test: {:?}", path);
                    println!("Error: {:?}", e);
                    panic!();
                }            
            }
        }
    }

    #[test]
    fn test_get_file_as_bytes_file_not_found() {
        if let Ok(mut path) = env::current_dir() {
            path.push("src");
            path.push("tests");
            path.push("bytes2.txt");

            let r = get_file_as_byte_vec(path.as_path());
            match r {
                Ok(_) => {
                    panic!("File should not exist");
                }

                Err(e) => {
                    assert_eq!(e, FileErrors::FileNotFound);
                }            
            }
        }
    }
}