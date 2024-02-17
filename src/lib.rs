mod file_handling;

pub mod prelude {
    use crate::file_handling;
    pub use file_handling::get_file_as_byte_vec;
    pub use file_handling::FileErrors;
}

#[cfg(test)]
mod tests {
    use crate::file_handling::FileErrors;
    use super::file_handling::get_file_as_byte_vec;
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
