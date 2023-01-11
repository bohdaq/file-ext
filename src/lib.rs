use std::{fs};
use std::fs::{File};
use std::process::Command;
use crate::date_time_ext::DateTimeExt;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;
use crate::path_ext_impl::PathExtImpl;
use crate::symbol::SYMBOL;
use crate::symlink_ext_impl::SymlinkExtImpl;

#[cfg(test)]
mod tests;
mod date_time_ext;
mod symbol;
mod file_ext_impl;
mod path_ext_impl;
mod directory_ext_impl;
mod symlink_ext_impl;

pub struct FileExt;

impl FileExt {

    /// Returns portion of a file of specified range. Range described as starting from byte M up to byte N.
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn partial_read() {
    ///     let path = "test/index.html";
    ///     let file_raw_bytes = FileExt::read_file_partially(path, 4, 10).unwrap();
    ///     let content = String::from_utf8(file_raw_bytes).unwrap();
    ///
    ///     let expected_content = "CTYPE h";
    ///
    ///     assert_eq!(expected_content, content);
    /// }
    /// ```
    pub fn read_file_partially(filepath: &str, start: u64, end: u64) -> Result<Vec<u8>, String> {
        FileExtImpl::read_file_partially(filepath, start, end)
    }

    /// Returns file content
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  fn file_content() {
    ///      let path = "test/index.html";
    ///      let file_raw_bytes = FileExt::read_file(path).unwrap();
    ///      let content = String::from_utf8(file_raw_bytes).unwrap();
    ///  
    ///      let content_escaped_newline_carriage_return = str::replace(content.as_str(), "\r\n", "\n");
    ///
    ///      let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";
    ///
    ///      assert_eq!(expected_content, content_escaped_newline_carriage_return);
    ///  }
    /// ```
    pub fn read_file(filepath: &str) -> Result<Vec<u8>, String> {
        FileExtImpl::read_file(filepath)
    }

    /// Returns file modification timestamp as nanoseconds in Unix epoch
    /// # Examples
    ///
    /// ```
    ///  use std::{thread, time};
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  fn modification_timestamp() {
    ///
    ///      let content = "data".as_bytes();
    ///      let path = "modification_timestamp-test.content";
    ///
    ///      FileExt::create_file(path).unwrap();
    ///      FileExt::write_file(path, content).unwrap();
    ///
    ///      let does_exist = FileExt::does_file_exist(path);
    ///      assert!(does_exist);
    ///
    ///      let modified_timestamp = FileExt::file_modified_utc(path).unwrap();
    ///
    ///      let one_second = time::Duration::from_secs(1);
    ///      thread::sleep(one_second);
    ///
    ///      FileExt::write_file(path, "\nnewline and some data".as_bytes()).unwrap();
    ///
    ///      let after_update_modified_timestamp = FileExt::file_modified_utc(path).unwrap();
    ///      assert!(after_update_modified_timestamp > modified_timestamp);
    ///
    ///
    ///      FileExt::delete_file(path).unwrap();
    ///      let doesnt_exist = !FileExt::does_file_exist(path);
    ///      assert!(doesnt_exist);
    ///  }
    /// ```
    pub fn file_modified_utc(filepath: &str) -> Result<u128, String> {
        let boxed_open = File::open(filepath);
        if boxed_open.is_err() {
            let error_msg = boxed_open.err().unwrap();
            let error = format!("<p>Unable to open file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        }

        let file : File = boxed_open.unwrap();
        let boxed_metadata = file.metadata();
        if boxed_metadata.is_err() {
            let error_msg = boxed_metadata.err().unwrap();
            let error = format!("<p>Unable to open file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        }
        let metadata = boxed_metadata.unwrap();
        let boxed_last_modified_time = metadata.modified();
        if boxed_last_modified_time.is_err() {
            let error_msg = boxed_last_modified_time.err().unwrap();
            let error = format!("<p>Unable to open file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        }
        let modified_time = boxed_last_modified_time.unwrap();
        let nanos = DateTimeExt::_system_time_to_unix_nanos(modified_time);
        Ok(nanos)
    }

    #[cfg(target_family = "unix")]
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn unix_path_delimiter() {
    ///     let expected = SYMBOL.slash.to_string();
    ///     let actual = FileExt::get_path_separator();
    /// }
    /// ```
    pub fn get_path_separator() -> String {
        PathExtImpl::get_path_separator()
    }

    #[cfg(target_family = "windows")]
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn unix_path_delimiter() {
    ///     let expected = SYMBOL.reverse_slash.to_string();
    ///     let actual = FileExt::get_path_separator();
    /// }
    /// ```
    pub fn get_path_separator() -> String { PathExtImpl::get_path_separator() }


    /// Will return absolute file path to the working directory
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn absolute_path_to_working_directory() {
    ///     let boxed_path = FileExt::get_static_filepath(FileExt::get_path_separator().as_str());
    ///     assert!(boxed_path.is_ok());
    ///     let path = boxed_path.unwrap();
    /// }
    /// ```
    pub fn get_static_filepath(path: &str) -> Result<String, String> {
        let boxed_working_directory = PathExtImpl::absolute_path_to_working_directory();
        if boxed_working_directory.is_err() {
            let message = boxed_working_directory.err().unwrap();
            return Err(message)
        }

        let working_directory = boxed_working_directory.unwrap();
        let absolute_path = [working_directory, path.to_string()].join(SYMBOL.empty_string);
        Ok(absolute_path)
    }

    /// Will try to read from file. If file does not exist, will create and write to it given byte array
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  fn read_or_create_and_write() {
    ///      let content = "data".as_bytes();
    ///      let tmp_folder = FileExt::get_temp_folder_path().unwrap();
    ///
    ///      let path = [tmp_folder, "test.txt".to_string()].join(FileExt::get_path_separator().as_str());
    ///
    ///      let doesnt_exist = !FileExt::does_file_exist(path.as_str());
    ///      assert!(doesnt_exist);
    ///
    ///      FileExt::read_or_create_and_write(path.as_str(), content).unwrap();
    ///
    ///      let does_exist = FileExt::does_file_exist(path.as_str());
    ///      assert!(does_exist);
    ///
    ///      let new_content = "updated data".as_bytes();
    ///      FileExt::read_or_create_and_write(path.as_str(), new_content).unwrap();
    ///
    ///      let file_content = FileExt::read_file(path.as_str()).unwrap();
    ///      assert_eq!(content, file_content);
    ///
    ///      FileExt::delete_file(path.as_str()).unwrap();
    ///      let doesnt_exist = !FileExt::does_file_exist(path.as_str());
    ///      assert!(doesnt_exist);
    ///  }
    /// ```
    pub fn read_or_create_and_write(path: &str, content: &[u8]) -> Result<Vec<u8>, String> {
        FileExtImpl::read_or_create_and_write(path, content)
    }

    /// Will create a file on the path
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn file_creation_deletion() {
    ///     let path = "test/file-creation.txt";
    ///
    ///     let exists = FileExt::does_file_exist(path);
    ///     assert!(!exists);
    ///
    ///     FileExt::create_file(path).unwrap();
    ///
    ///     let content = FileExt::read_file(path).unwrap();
    ///     assert_eq!(content.len(), 0);
    ///
    ///     FileExt::delete_file(path).unwrap();
    ///
    ///     let exists = FileExt::does_file_exist(path);
    ///     assert!(!exists);
    /// }
    /// ```
    pub fn create_file(path: &str) -> Result<(), String>  {
        FileExtImpl::create_file(path)
    }

    /// Returns boolean indicating file existence on the path
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn file_exists() {
    ///     let path = "test/index_rewrite";
    ///     let exists = FileExt::does_file_exist(path);
    ///     assert!(exists);
    /// }
    /// ```
    pub fn does_file_exist(path: &str) -> bool {
        FileExtImpl::does_file_exist(path)
    }

    /// Returns boolean indicating directory existence on the path
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn directory_exists() {
    ///     let path = "test";
    ///     let exists = FileExt::does_directory_exist(path);
    ///     assert!(exists);
    /// }
    /// ```
    pub fn does_directory_exist(path: &str) -> bool {
        DirectoryExtImpl::does_directory_exist(path)
    }

    /// Returns boolean indicating symlink existence on the path
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn symlink_exists() {
    ///   let symlink_path = ["test", "index-link2"].join(FileExt::get_path_separator().as_str());
    ///
    ///   if FileExt::does_symlink_exist(symlink_path.as_str()) {
    ///     FileExt::delete_file(symlink_path.as_str()).unwrap();
    ///   }
    ///
    ///   let path = [SYMBOL.empty_string, "test", SYMBOL.empty_string].join(FileExt::get_path_separator().as_str());
    ///   let path_prefix = FileExt::get_static_filepath(path.as_str()).unwrap();
    ///   let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");
    ///
    ///   let boxed_symlink = FileExt::create_symlink(
    ///     path_prefix.as_str(),
    ///     "index-link2",
    ///     points_to.as_str()
    ///   );
    ///
    ///
    ///   assert!(boxed_symlink.is_ok());
    ///
    ///   let symlink_created = FileExt::does_symlink_exist(symlink_path.as_str());
    ///   assert!(symlink_created);
    ///
    ///   let actual_points_to = FileExt::symlink_points_to(symlink_path.as_str()).unwrap();
    ///   assert_eq!(points_to, actual_points_to);
    ///
    ///   FileExt::delete_file(symlink_path.as_str()).unwrap();
    /// }
    /// ```
    pub fn does_symlink_exist(path: &str) -> bool {
        SymlinkExtImpl::does_symlink_exist(path)
    }

    /// Will write given byte array to a file on the path
    /// # Examples
    /// ```
    ///  use file_ext::FileExt;
    /// #[test]
    ///  fn write() {
    ///      let filename = "write-test.content";
    ///      FileExt::create_file(filename).unwrap();
    ///
    ///      let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";
    ///      FileExt::write_file(filename, expected_content.as_bytes()).unwrap();
    ///
    ///      let actual = FileExt::read_file(filename).unwrap();
    ///      assert_eq!(actual, expected_content.as_bytes());
    ///
    ///  }
    /// ```
    pub fn write_file(path: &str, file_content: &[u8]) -> Result<(), String> {
        FileExtImpl::write_file(path, file_content)
    }

    /// Will delete file on a given path
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn file_creation_deletion() {
    ///     let path = "test/file-creation.txt";
    ///
    ///     let exists = FileExt::does_file_exist(path);
    ///     assert!(!exists);
    ///
    ///     FileExt::create_file(path).unwrap();
    ///
    ///     let content = FileExt::read_file(path).unwrap();
    ///     assert_eq!(content.len(), 0);
    ///
    ///     FileExt::delete_file(path).unwrap();
    ///
    ///     let exists = FileExt::does_file_exist(path);
    ///     assert!(!exists);
    /// }
    /// ```
    pub fn delete_file(path: &str) -> Result<(), String> {
        FileExtImpl::delete_file(path)
    }

    /// Will create symlink on path `symlink_path` with the specified name `symlink_name`.
    /// Symlink will point to specific file or directory `symlink_points_to`. Paths are absolute.
    /// # Examples
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    ///fn symlink_creation() {
    ///    let symlink_path = "test/index-link";
    ///
    ///    if FileExt::does_symlink_exist(symlink_path) {
    ///        FileExt::delete_file(symlink_path).unwrap();
    ///    }
    ///
    ///    let path_prefix = FileExt::get_static_filepath("/test/").unwrap();
    ///    let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");
    ///
    ///    let boxed_symlink = FileExt::create_symlink(
    ///        path_prefix.as_str(),
    ///        "index-link",
    ///        points_to.as_str());
    ///
    ///     assert!(boxed_symlink.is_ok());
    ///
    ///     let symlink_created = FileExt::does_symlink_exist(symlink_path);
    ///     assert!(symlink_created);
    ///
    ///     let actual_points_to = FileExt::symlink_points_to(symlink_path).unwrap();
    ///     assert_eq!(points_to, actual_points_to);
    ///
    ///     FileExt::delete_file(symlink_path).unwrap();
    ///}
    ///```
    #[cfg(target_family = "unix")]
    pub fn create_symlink(symlink_path: &str, symlink_name: &str, symlink_points_to: &str) -> Result<(), String> {
        SymlinkExtImpl::create_symlink(symlink_path, symlink_name, symlink_points_to)
    }


    /// Will create symlink on path `symlink_path` with the specified name `symlink_name`.
    /// Symlink will point to specific file or directory `symlink_points_to`. Paths are absolute.
    /// # Examples
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    ///fn symlink_creation() {
    ///    let symlink_path = "test/index-link";
    ///
    ///    if FileExt::does_symlink_exist(symlink_path) {
    ///        FileExt::delete_file(symlink_path).unwrap();
    ///    }
    ///
    ///    let path_prefix = FileExt::get_static_filepath("/test/").unwrap();
    ///    let points_to = [path_prefix.to_string(), "index.html".to_string()].join("");
    ///
    ///    let boxed_symlink = FileExt::create_symlink(
    ///        path_prefix.as_str(),
    ///        "index-link",
    ///        points_to.as_str());
    ///
    ///        assert!(boxed_symlink.is_ok());
    ///
    ///        let symlink_created = FileExt::does_symlink_exist(symlink_path);
    ///        assert!(symlink_created);
    ///
    ///        let actual_points_to = FileExt::symlink_points_to(symlink_path).unwrap();
    ///        assert_eq!(points_to, actual_points_to);
    ///
    ///        FileExt::delete_file(symlink_path).unwrap();
    ///}
    ///```
    #[cfg(target_family = "windows")]
    pub fn create_symlink(symlink_path: &str, symlink_name: &str, symlink_points_to: &str) -> Result<(), String> {
        SymlinkExtImpl::create_symlink(symlink_path, symlink_name, symlink_points_to)
    }

    /// Checks if the file is symlink
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn is_link() {
    ///     let path: String = ["test", "index_rewrite"].join(&FileExt::get_path_separator());
    ///     let is_symlink = FileExt::is_symlink(path.as_str()).unwrap();
    ///     assert!(is_symlink);
    /// }
    /// ```
    pub fn is_symlink(path: &str) -> Result<bool, String> {
        let boxed_symlink_metadata = fs::symlink_metadata(path);
        if boxed_symlink_metadata.is_err() {
            let msg = boxed_symlink_metadata.err().unwrap().to_string();
            return Err(msg)
        }

        let symlink_metadata = boxed_symlink_metadata.unwrap();
        Ok(symlink_metadata.file_type().is_symlink())
    }

    /// Returns path to a file, symlink points to
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn link_points_to() {
    ///     let path: String = ["test", "index_rewrite"].join(&FileExt::get_path_separator());
    ///     let points_to = FileExt::symlink_points_to(path.as_str()).unwrap();
    ///     assert_eq!("index.html", points_to);
    /// }
    /// ```
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

    /// Returns name of the user running the process
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  #[cfg(target_family = "unix")]
    ///  fn current_user() {
    ///      let boxed_user = FileExt::get_current_user();
    ///      assert!(boxed_user.is_ok());
    ///
    ///      let user = boxed_user.unwrap();
    ///  }
    /// ```
    #[cfg(target_family = "unix")]
    pub fn get_current_user() -> Result<String, String> {
        let boxed_whoami = Command::new("whoami")
            .output();

        if boxed_whoami.is_err() {
            let message = boxed_whoami.err().unwrap().to_string();
            return Err(message);
        }

        let boxed_current_user = String::from_utf8(boxed_whoami.unwrap().stdout);
        if boxed_current_user.is_err() {
            let message = boxed_current_user.err().unwrap().to_string();
            return Err(message);
        }

        let current_user = boxed_current_user.unwrap();
        let user = str::replace(current_user.as_str(), "\n", "");

        Ok(user)
    }

    /// Returns name of the user running the process
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  #[cfg(target_family = "windows")]
    ///  fn current_user() {
    ///      let boxed_user = FileExt::get_current_user();
    ///      assert!(boxed_user.is_ok());
    ///
    ///      let user = boxed_user.unwrap();
    ///  }
    /// ```
    #[cfg(target_family = "windows")]
    pub fn get_current_user() -> Result<String, String> {
        let boxed_whoami = Command::new("whoami")
            .output();

        if boxed_whoami.is_err() {
            let message = boxed_whoami.err().unwrap().to_string();
            return Err(message);
        }

        let boxed_current_user = String::from_utf8(boxed_whoami.unwrap().stdout);
        if boxed_current_user.is_err() {
            let message = boxed_current_user.err().unwrap().to_string();
            return Err(message);
        }

        let current_user = boxed_current_user.unwrap();

        let boxed_domain_user = current_user.split_once("\\");
        if boxed_domain_user.is_none() {
            let message = format!("unable to extract user: {}", current_user);
            return Err(message);
        }

        let (_domain, user) = boxed_domain_user.unwrap();

        let user = str::replace(user, "\r\n", "");

        Ok(user.to_string())
    }

    /// Returns domain of the user running the process
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  #[cfg(target_family = "windows")]
    ///  fn current_user() {
    ///      let boxed_user_domain = FileExt::get_current_user_domain();
    ///      assert!(boxed_user_domain.is_ok());
    ///
    ///      let domain = boxed_user_domain.unwrap();
    ///  }
    /// ```
    #[cfg(target_family = "windows")]
    pub fn get_current_user_domain() -> Result<String, String> {
        let boxed_whoami = Command::new("whoami")
            .output();

        if boxed_whoami.is_err() {
            let message = boxed_whoami.err().unwrap().to_string();
            return Err(message);
        }

        let boxed_current_user = String::from_utf8(boxed_whoami.unwrap().stdout);
        if boxed_current_user.is_err() {
            let message = boxed_current_user.err().unwrap().to_string();
            return Err(message);
        }

        let current_user = boxed_current_user.unwrap();

        let boxed_domain_user = current_user.split_once("\\");
        if boxed_domain_user.is_none() {
            let message = format!("unable to extract user: {}", current_user);
            return Err(message);
        }

        let (domain, _user) = boxed_domain_user.unwrap();

        Ok(domain.to_string())
    }

    /// Returns path to the temporary folder
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  #[cfg(target_family = "windows")]
    ///  fn temp_folder() {
    ///      let temp_folder_path = FileExt::get_temp_folder_path().unwrap();
    ///      assert!(temp_folder_path.starts_with("C:\\Users\\"));
    ///      assert!(temp_folder_path.ends_with("\\AppData\\Local\\Temp"));
    ///  }
    /// ```
    #[cfg(target_family = "windows")]
    pub fn get_temp_folder_path() -> Result<String, String>{
        PathExtImpl::get_temp_folder_path()
    }

    /// Returns path to the temporary folder
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  #[cfg(target_family = "unix")]
    ///  fn temp_folder() {
    ///      let temp_folder_path = FileExt::get_temp_folder_path().unwrap();
    ///      assert_eq!(temp_folder_path, "/tmp")
    ///  }
    /// ```
    #[cfg(target_family = "unix")]
    pub fn get_temp_folder_path() -> Result<String, String>{
        PathExtImpl::get_temp_folder_path()
    }
}

