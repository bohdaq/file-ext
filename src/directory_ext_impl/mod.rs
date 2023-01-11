use std::fs;
use std::path::Path;

pub struct DirectoryExtImpl;

impl DirectoryExtImpl {
    pub fn does_directory_exist(path: &str) -> bool {
        let file_exists = Path::new(path).is_dir();
        file_exists
    }

    pub fn create_directory(path: &str) -> Result<(), String> {
        let boxed_create_dir = fs::create_dir(path);
        if boxed_create_dir.is_err() {
            let message = boxed_create_dir.err().unwrap().to_string();
            return Err(message)
        }
        Ok(())
    }

    pub fn delete_directory(path: &str) -> Result<(), String> {
        let boxed_create_dir = fs::remove_dir_all(path);
        if boxed_create_dir.is_err() {
            let message = boxed_create_dir.err().unwrap().to_string();
            return Err(message)
        }
        Ok(())
    }
}