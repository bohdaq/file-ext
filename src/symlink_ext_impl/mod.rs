use std::fs;
use std::path::Path;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;

#[cfg(test)]
mod tests;

pub struct SymlinkExtImpl;

impl SymlinkExtImpl {
    pub fn does_symlink_exist(path: &str) -> bool {
        Path::new(path).is_symlink()
    }

    #[cfg(target_family = "unix")]
    pub fn create_symlink(symlink_path: &str, symlink_name: &str, symlink_points_to: &str) -> Result<(), String> {
        //check if there is already a file where symlink is going to be created
        let path_to_symlink_included = [symlink_path, symlink_name].join("");
        let does_file_exist = FileExtImpl::does_file_exist(&path_to_symlink_included);
        if does_file_exist {
            let message = format!("There is a file on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(&path_to_symlink_included);
        if does_directory_exist {
            let message = format!("There is a directory on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }

        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExtImpl::does_file_exist(symlink_points_to);
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(symlink_points_to);

        if !does_file_exist && !does_directory_exist   {
            let message = format!("There is no file or directory for symlink to be created: {}", symlink_points_to);
            return Err(message)
        }

        let boxed_symlink = std::os::unix::fs::symlink(symlink_points_to, path_to_symlink_included);
        if boxed_symlink.is_err()   {
            let message = boxed_symlink.err().unwrap().to_string();
            return Err(message)
        }

        Ok(())

    }

    #[cfg(target_family = "windows")]
    pub fn create_symlink(symlink_path: &str, symlink_name: &str, symlink_points_to: &str) -> Result<(), String> {
        //check if there is already a file where symlink is going to be created
        let path_to_symlink_included = [symlink_path, symlink_name].join("");
        let does_file_exist = FileExtImpl::does_file_exist(&path_to_symlink_included);
        if does_file_exist {
            let message = format!("There is a file on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(&path_to_symlink_included);
        if does_directory_exist {
            let message = format!("There is a directory on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }

        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExtImpl::does_file_exist(symlink_points_to);
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(symlink_points_to);

        if !does_file_exist && !does_directory_exist   {
            let message = format!("There is no file or directory for symlink to be created: {}", symlink_points_to);
            return Err(message)
        }

        if does_file_exist {
            let boxed_symlink = std::os::windows::fs::symlink_file(symlink_points_to, path_to_symlink_included);
            if boxed_symlink.is_err()   {
                let message = boxed_symlink.err().unwrap().to_string();
                return Err(message)
            }

            return  Ok(());
        }

        if does_directory_exist {
            let boxed_symlink = std::os::windows::fs::symlink_dir(symlink_points_to, path_to_symlink_included);
            if boxed_symlink.is_err()   {
                let message = boxed_symlink.err().unwrap().to_string();
                return Err(message)
            }

            return Ok(());
        }

        let message = "Something went wrong".to_string();
        Err(message)

    }

    pub fn is_symlink(path: &str) -> Result<bool, String> {
        let boxed_symlink_metadata = fs::symlink_metadata(path);
        if boxed_symlink_metadata.is_err() {
            let msg = boxed_symlink_metadata.err().unwrap().to_string();
            return Err(msg)
        }

        let symlink_metadata = boxed_symlink_metadata.unwrap();
        Ok(symlink_metadata.file_type().is_symlink())
    }

    pub fn symlink_points_to(path: &str) -> Result<String, String> {
        let boxed_path_buff = fs::read_link(path);
        if boxed_path_buff.is_err() {
            let msg = boxed_path_buff.err().unwrap().to_string();
            return Err(msg)
        }
        let path_buff = boxed_path_buff.unwrap();
        let boxed_points_to = path_buff.as_path().to_str();
        if boxed_points_to.is_none() {
            let msg = "unable to read link as path".to_string();
            return Err(msg)
        }
        let points_to = boxed_points_to.unwrap();
        Ok(points_to.to_string())
    }
}