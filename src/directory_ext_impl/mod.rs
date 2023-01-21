use std::fs;
use std::path::Path;
use std::process::Command;
use crate::filter_string::FilterString;
use crate::path_ext_impl::PathExtImpl;
use crate::symbol::SYMBOL;

#[cfg(test)]
mod tests;

pub struct DirectoryExtImpl;

impl DirectoryExtImpl {
    pub fn does_directory_exist(path: &str) -> bool {
        let file_exists = Path::new(path).is_dir();
        file_exists
    }

    pub fn create_directory(path: &str) -> Result<(), String> {
        let boxed_check = FilterString::is_valid_input_string(path);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

        DirectoryExtImpl::recursively_create_directories("", path)
    }

    pub fn delete_directory(path: &str) -> Result<(), String> {
        let boxed_check = FilterString::is_valid_input_string(path);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

        if !DirectoryExtImpl::does_directory_exist(path) {
            let message = format!("There is no directory at the given path: {}", path);
            return Err(message)
        }

        DirectoryExtImpl::remove_directory_recursively_bypass_warnings(path)
    }

    #[cfg(target_family = "windows")]
    fn remove_directory_recursively_bypass_warnings(path: &str) -> Result<(), String> {

        let boxed_rm_rf = Command::new("cmd")
            .args(["/c", "rd" ,"/s", "/q", path])
            .output();

        if boxed_rm_rf.is_err() {
            let message = boxed_rm_rf.err().unwrap().to_string();
            return Err(message)
        }

        let output = boxed_rm_rf.unwrap();

        let success = output.status.success();
        if !success {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            let log = [stdout, stderr].join(SYMBOL.new_line_carriage_return);

            return Err(log);
        }

        Ok(())
    }

    #[cfg(target_family = "unix")]
    fn remove_directory_recursively_bypass_warnings(path: &str) -> Result<(), String> {

        let boxed_rm_rf = Command::new("rm")
            .args(["-Rf", path])
            .output();

        if boxed_rm_rf.is_err() {
            let message = boxed_rm_rf.err().unwrap().to_string();
            return Err(message)
        }

        let output = boxed_rm_rf.unwrap();

        let success = output.status.success();
        if !success {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            let log = [stdout, stderr].join(SYMBOL.new_line_carriage_return);

            return Err(log);
        }

        Ok(())
    }

    fn recursively_create_directories(processed_path: &str, remaining_path: &str) -> Result<(), String> {
        let boxed_split = remaining_path.split_once(PathExtImpl::get_path_separator().as_str());
        if boxed_split.is_none() {
            let mut folder_path = remaining_path.to_string();
            if processed_path.chars().count() != 0 {
                folder_path = [processed_path, remaining_path].join(PathExtImpl::get_path_separator().as_str());
            }


            let boxed_create_folder = fs::create_dir(folder_path.as_str());
            if boxed_create_folder.is_err() {
                let message = boxed_create_folder.err().unwrap().to_string();
                return Err(message)
            }

            return Ok(());
        }
        let (folder, remaining_path) = boxed_split.unwrap();

        let mut  folder_path = folder.to_string();
        if processed_path.chars().count() != 0 {
            folder_path = [processed_path, folder].join(PathExtImpl::get_path_separator().as_str());
        }


        let boxed_create_folder = fs::create_dir(folder_path.as_str());
        if boxed_create_folder.is_err() {
            let message = boxed_create_folder.err().unwrap().to_string();
            return Err(message)
        }
        let mut _processed_path = folder.to_string();
        if processed_path.chars().count() != 0 {
            _processed_path = [processed_path, folder].join(PathExtImpl::get_path_separator().as_str());
        }
        DirectoryExtImpl::recursively_create_directories(_processed_path.as_str(), remaining_path)
    }
}

