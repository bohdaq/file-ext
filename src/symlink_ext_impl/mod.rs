use std::fs;
use std::path::Path;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;
use crate::FileExt;
use crate::path_ext_impl::PathExtImpl;
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
        // FileExt::write_file("out.log", "\nsymlink_path:".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_path.as_bytes()).unwrap();
        //
        // FileExt::write_file("out.log", "\nsymlink_name:".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_name.as_bytes()).unwrap();
        //
        // FileExt::write_file("out.log", "\nsymlink_points_to:".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_points_to.as_bytes()).unwrap();

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

        // FileExt::write_file("out.log", "\n".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_path.as_bytes()).unwrap();
        // FileExt::write_file("out.log", "\n".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_points_to.as_bytes()).unwrap();


        let mut absolute_path_symlink_points_to = SymlinkExtImpl::resolve_symlink_path(symlink_path,symlink_points_to).unwrap();

        // FileExt::write_file("out.log", "\n!!".as_bytes()).unwrap();
        // FileExt::write_file("out.log", absolute_path_symlink_points_to.as_bytes()).unwrap();
        //
        // FileExt::write_file("out.log", "\n!!".as_bytes()).unwrap();
        // FileExt::write_file("out.log", absolute_path_symlink_points_to.as_bytes()).unwrap();

        if !symlink_points_to.starts_with(SYMBOL.slash) {
            let working_directory = FileExt::get_static_filepath("").unwrap();
            absolute_path_symlink_points_to = [working_directory, absolute_path_symlink_points_to.to_string()].join(PathExtImpl::get_path_separator().as_str());
        }


        // FileExt::write_file("out.log", "\nabsolute_path_symlink_points_to:".as_bytes()).unwrap();
        // FileExt::write_file("out.log", absolute_path_symlink_points_to.as_bytes()).unwrap();

        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExtImpl::does_file_exist(absolute_path_symlink_points_to.as_str());
        let does_directory_exist = DirectoryExtImpl::does_directory_exist(absolute_path_symlink_points_to.as_str());

        if !does_file_exist && !does_directory_exist   {
            let message = format!("There is no file or directory for symlink to be created: {}", absolute_path_symlink_points_to.as_str());
            return Err(message)
        }

        let boxed_symlink = std::os::unix::fs::symlink(absolute_path_symlink_points_to.as_str(), path_to_symlink_included);
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

    pub fn resolve_symlink_path(symlink_directory: &str, symlink_points_to: &str) -> Result<String, String> {
        // FileExt::write_file("out.log", "\n\nresolve_symlink_path symlink_directory:".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_directory.as_bytes()).unwrap();

        // FileExt::write_file("out.log", "\nresolve_symlink_path symlink_points_to:".as_bytes()).unwrap();
        // FileExt::write_file("out.log", symlink_points_to.as_bytes()).unwrap();

        if symlink_points_to.starts_with(PathExtImpl::get_path_separator().as_str()) {
            return Ok(symlink_points_to.to_string())
        }

        let boxed_split = symlink_points_to.split_once(PathExtImpl::get_path_separator().as_str());
        if boxed_split.is_none() {
            let path = [symlink_directory, symlink_points_to].join(PathExtImpl::get_path_separator().as_str());
            // FileExt::write_file("out.log", "\n1".as_bytes()).unwrap();
            return Ok(path)
        }

        let (part, symlink_after_split) = boxed_split.unwrap();
        if part == ".." {
            // FileExt::write_file("out.log", "\n2".as_bytes()).unwrap();

            if symlink_directory.chars().count() == 0 {
                let message = "not valid path for the symlink";
                return Err(message.to_string())
            }

            let reversed_base_dir = symlink_directory.chars().rev().collect::<String>();
            let boxed_one_level_up_split = reversed_base_dir.split_once(PathExtImpl::get_path_separator().as_str());
            if boxed_one_level_up_split.is_some() {
                // FileExt::write_file("out.log", "\n3".as_bytes()).unwrap();

                let (_cut_folder, remaining_base_dir) = boxed_one_level_up_split.unwrap();
                let _symlink_directory = remaining_base_dir.chars().rev().collect::<String>();
                return  SymlinkExtImpl::resolve_symlink_path(_symlink_directory.as_str(), symlink_after_split);
            } else {
                // FileExt::write_file("out.log", "\n4".as_bytes()).unwrap();

                return  SymlinkExtImpl::resolve_symlink_path("", symlink_after_split);
            }


        } else {
            // FileExt::write_file("out.log", "\n5".as_bytes()).unwrap();

            let mut _symlink_directory = part.to_string();
            if symlink_directory.chars().count() != 0 {
                _symlink_directory = [symlink_directory, part].join(PathExtImpl::get_path_separator().as_str());
            }

            return SymlinkExtImpl::resolve_symlink_path(_symlink_directory.as_str(), symlink_after_split);
        }

        Ok(symlink_points_to.to_string())
    }
}