use std::fs;
use std::path::Path;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;
#[cfg(target_family = "unix")]
use crate::symbol::SYMBOL;

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
        let path_to_symlink_included = [symlink_path, symlink_name].join(PathExtImpl::get_path_separator().as_str());
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

        let boxed_resolved_path = SymlinkExtImpl::resolve_symlink_path(symlink_path,symlink_points_to);
        if boxed_resolved_path.is_err() {
            let message = boxed_resolved_path.err().unwrap();
            return Err(message)
        }

        let mut resolved_path_symlink_point_to = boxed_resolved_path.unwrap();

        if !symlink_points_to.starts_with(SYMBOL.slash) {
            let working_directory = FileExt::working_directory().unwrap();
            resolved_path_symlink_point_to = [working_directory, resolved_path_symlink_point_to.to_string()].join(PathExtImpl::get_path_separator().as_str());
        }


        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExtImpl::does_file_exist(resolved_path_symlink_point_to.as_str());
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(resolved_path_symlink_point_to.as_str());

        if !does_file_exist && !does_directory_exist   {
            let message = format!("There is no file or directory for symlink to be created: {}", resolved_path_symlink_point_to.as_str());
            return Err(message)
        }

        let boxed_symlink = std::os::unix::fs::symlink(resolved_path_symlink_point_to.as_str(), path_to_symlink_included);
        if boxed_symlink.is_err()   {
            let message = boxed_symlink.err().unwrap().to_string();
            return Err(message)
        }

        Ok(())

    }

    #[cfg(target_family = "windows")]
    pub fn create_symlink(symlink_path: &str, symlink_name: &str, symlink_points_to: &str) -> Result<(), String> {
        //check if there is already a file where symlink is going to be created
        let path_to_symlink_included = [symlink_path, symlink_name].join(PathExtImpl::get_path_separator().as_str());
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

        let boxed_resolved_path = SymlinkExtImpl::resolve_symlink_path(symlink_path,symlink_points_to);
        if boxed_resolved_path.is_err() {
            let message = boxed_resolved_path.err().unwrap();
            return Err(message)
        }

        let mut resolved_path = boxed_resolved_path.unwrap();

        if resolved_path.chars().count() >= 2 {
            let second_char = resolved_path.chars().take(2).last().unwrap();
            if second_char != ':' {
                let working_directory = FileExt::working_directory().unwrap();
                resolved_path = PathExtImpl::build_path(&[&working_directory, &resolved_path]);
            }
        }

        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExtImpl::does_file_exist(&resolved_path);
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(&resolved_path);

        if !does_file_exist && !does_directory_exist   {
            let message = format!("There is no file or directory for symlink to be created: {}", &resolved_path);
            return Err(message)
        }

        if does_file_exist {
            let boxed_symlink = std::os::windows::fs::symlink_file(&resolved_path, path_to_symlink_included);
            if boxed_symlink.is_err()   {
                let message = boxed_symlink.err().unwrap().to_string();
                return Err(message)
            }

            return  Ok(());
        }

        if does_directory_exist {
            let boxed_symlink = std::os::windows::fs::symlink_dir(&resolved_path, path_to_symlink_included);
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

    pub fn resolve_symlink_path(symlink_directory: &str, symlink_points_to: &str) -> Result<String, String> {

        // windows specific check on a link
        if symlink_points_to.chars().count() >= 2 {
            let second_char = symlink_points_to.chars().take(2).last().unwrap();
            if second_char == ':' {
                return Ok(symlink_points_to.to_string())
            }
        }

        if symlink_points_to.starts_with(PathExtImpl::get_path_separator().as_str()) {
            return Ok(symlink_points_to.to_string())
        }

        let boxed_split = symlink_points_to.split_once(PathExtImpl::get_path_separator().as_str());
        if boxed_split.is_none() {
            let path = [symlink_directory, symlink_points_to].join(PathExtImpl::get_path_separator().as_str());
            return Ok(path)
        }

        let (part, symlink_after_split) = boxed_split.unwrap();
        return if part == ".." {

            if symlink_directory.chars().count() == 0 {
                let message = "not valid path for the symlink";
                return Err(message.to_string())
            }

            let reversed_base_dir = symlink_directory.chars().rev().collect::<String>();
            let boxed_one_level_up_split = reversed_base_dir.split_once(PathExtImpl::get_path_separator().as_str());
            if boxed_one_level_up_split.is_some() {
                let (_cut_folder, remaining_base_dir) = boxed_one_level_up_split.unwrap();
                let _symlink_directory = remaining_base_dir.chars().rev().collect::<String>();
                SymlinkExtImpl::resolve_symlink_path(_symlink_directory.as_str(), symlink_after_split)
            } else {
                SymlinkExtImpl::resolve_symlink_path("", symlink_after_split)
            }
        } else {
            let mut _symlink_directory = part.to_string();
            if symlink_directory.chars().count() != 0 {
                _symlink_directory = [symlink_directory, part].join(PathExtImpl::get_path_separator().as_str());
            }

            SymlinkExtImpl::resolve_symlink_path(_symlink_directory.as_str(), symlink_after_split)
        }

    }
}