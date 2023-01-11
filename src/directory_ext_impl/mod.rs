use std::path::Path;

pub struct DirectoryExtImpl;

impl DirectoryExtImpl {
    pub fn does_directory_exist(path: &str) -> bool {
        let file_exists = Path::new(path).is_file();
        file_exists
    }
}