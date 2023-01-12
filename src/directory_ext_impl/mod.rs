use std::fs;
use std::path::Path;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;

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

    pub(crate) fn recursive_call(processed_path: &str, remaining_path: &str, log_filename: &str) -> Result<(), String> {
        let name = log_filename;
        FileExt::write_file(name, format!("\n\nprocessed path: {}", processed_path).as_bytes()).unwrap();
        FileExt::write_file(name, format!("\nremaining path: {}", remaining_path).as_bytes()).unwrap();

        let boxed_split = remaining_path.split_once(PathExtImpl::get_path_separator().as_str());
        if boxed_split.is_none() {
            let mut folder_path = remaining_path.to_string();
            if processed_path.chars().count() != 0 {
                folder_path = [processed_path, remaining_path].join(PathExtImpl::get_path_separator().as_str());
            }

            FileExt::write_file(name, format!("\nfolder path: {}", folder_path).as_bytes()).unwrap();
            FileExt::write_file(name, format!("\nremaining path: {}", remaining_path).as_bytes()).unwrap();

            let boxed_create_folder = FileExt::create_directory(folder_path.as_str());
            if boxed_create_folder.is_err() {
                let message = boxed_create_folder.err().unwrap();
                return Err(message)
            }

            return Ok(());
        }
        let (folder, remaining_path) = boxed_split.unwrap();

        let mut  folder_path = folder.to_string();
        if processed_path.chars().count() != 0 {
            folder_path = [processed_path, folder].join(PathExtImpl::get_path_separator().as_str());
        }

        FileExt::write_file(name, format!("\nfolder path: {}", folder_path).as_bytes()).unwrap();
        FileExt::write_file(name, format!("\nremaining path: {}", remaining_path).as_bytes()).unwrap();

        let boxed_create_folder = FileExt::create_directory(folder_path.as_str());
        if boxed_create_folder.is_err() {
            let message = boxed_create_folder.err().unwrap();
            return Err(message)
        }
        let mut _processed_path = folder.to_string();
        if processed_path.chars().count() != 0 {
            _processed_path = [processed_path, folder].join(PathExtImpl::get_path_separator().as_str());
        }
        DirectoryExtImpl::recursive_call(_processed_path.as_str(), remaining_path, name)
    }
}

