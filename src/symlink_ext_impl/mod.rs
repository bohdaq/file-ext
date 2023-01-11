use std::path::Path;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;

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
}