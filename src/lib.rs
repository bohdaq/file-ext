use std::fs::{File};
use crate::date_time_ext::DateTimeExt;
use crate::directory_ext_impl::DirectoryExtImpl;
use crate::file_ext_impl::FileExtImpl;
use crate::path_ext_impl::PathExtImpl;
use crate::symbol::SYMBOL;
use crate::symlink_ext_impl::SymlinkExtImpl;
use crate::user_ext_impl::UserExtImpl;

#[cfg(test)]
mod tests;
mod date_time_ext;
mod symbol;
mod file_ext_impl;
mod path_ext_impl;
mod directory_ext_impl;
mod symlink_ext_impl;
mod user_ext_impl;
mod filter_string;

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
    ///     let boxed_path = FileExt::working_directory();
    ///     assert!(boxed_path.is_ok());
    ///     let path = boxed_path.unwrap();
    /// }
    /// ```
    pub fn working_directory() -> Result<String, String> {
        PathExtImpl::working_directory()
    }


    /// Will return absolute file path to the working directory. Same as working_directory function. Kept here to preserve backward compatability.
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn absolute_path_to_working_directory() {
    ///     let boxed_path = FileExt::absolute_path_to_working_directory();
    ///     assert!(boxed_path.is_ok());
    ///     let path = boxed_path.unwrap();
    /// }
    /// ```
    pub fn absolute_path_to_working_directory() -> Result<String, String> {
        PathExtImpl::working_directory()
    }


    /// Will return absolute working directory path appended to the given string
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn absolute_path_to_working_directory() {
    ///     let boxed_path = FileExt::get_static_filepath("");
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


    /// Will create a new directory on specified path
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  fn new_directory_create_delete() {
    ///      let path = "new_directory";
    ///
    ///      let boxed_create = FileExt::create_directory(path);
    ///      assert!(boxed_create.is_ok());
    ///
    ///      assert!(FileExt::does_directory_exist(path));
    ///
    ///      let boxed_delete = FileExt::delete_directory(path);
    ///      assert!(boxed_delete.is_ok());
    ///  }
    /// ```
    pub fn create_directory(path: &str) -> Result<(), String> {
        DirectoryExtImpl::create_directory(path)
    }

    /// Will delete directory and all of the content on specified path (won't follow symlinks)
    /// # Examples
    ///
    /// ```
    ///  use file_ext::FileExt;
    ///  #[test]
    ///  fn new_directory_create_delete() {
    ///      let path = "new_directory";
    ///
    ///      let boxed_create = FileExt::create_directory(path);
    ///      assert!(boxed_create.is_ok());
    ///
    ///      assert!(FileExt::does_directory_exist(path));
    ///
    ///      let boxed_delete = FileExt::delete_directory(path);
    ///      assert!(boxed_delete.is_ok());
    ///  }
    /// ```
    pub fn delete_directory(path: &str) -> Result<(), String> {
        DirectoryExtImpl::delete_directory(path)
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

    /// Returns absolute path, symlink points to.
    /// Takes 2 parameters: path to the directory, where symlink is located and where symlink points to
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn resolve_symlink_path() {
    ///    let base_dir = "/home/someuser/folder/subfolder/subsubfolder";
    ///    let symlink_points_to = "../../subfolder2/subsubfolder2";
    ///
    ///    let expected_path = "/home/someuser/folder/subfolder2/subsubfolder2";
    ///    let actual_path = FileExt::resolve_symlink_path(base_dir, symlink_points_to).unwrap();
    ///
    ///    assert_eq!(expected_path, actual_path);
    /// }
    /// ```
    pub fn resolve_symlink_path(symlink_directory: &str, symlink_points_to: &str) -> Result<String, String> {
        SymlinkExtImpl::resolve_symlink_path(symlink_directory, symlink_points_to)
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
        SymlinkExtImpl::is_symlink(path)
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
        SymlinkExtImpl::symlink_points_to(path)
    }

    /// Builds a path from a given node list
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    ///
    /// #[test]
    /// #[cfg(target_family = "unix")]
    /// fn build_path() {
    ///     let root = FileExt::root();
    ///     let folder_up = FileExt::folder_up();
    ///
    ///     let node_list =
    ///         [
    ///             root.as_str(),
    ///             "home",
    ///             "someuser",
    ///             "folder",
    ///             "subfolder",
    ///             "subsubfolder",
    ///         ];
    ///
    ///     let another_node_list =
    ///         [
    ///             folder_up.as_str(),
    ///             folder_up.as_str(),
    ///             "subfolder2",
    ///             "subsubfolder2",
    ///         ];
    ///     let path = PathExtImpl::build_path(&node_list);
    ///     let another_path = PathExtImpl::build_path(&another_node_list);
    ///
    ///     assert_eq!("/home/someuser/folder/subfolder/subsubfolder", path);
    ///     assert_eq!("../../subfolder2/subsubfolder2", another_path);
    /// }
    ///
    ///
    /// #[test]
    /// #[cfg(target_family = "windows")]
    /// fn build_path() {
    ///     let root = FileExt::root();
    ///     let folder_up = FileExt::folder_up();
    ///
    ///     let node_list =
    ///         [
    ///             root.as_str(),
    ///             "Users",
    ///             "someuser",
    ///             "folder",
    ///             "subfolder",
    ///             "subsubfolder",
    ///         ];
    ///
    ///     let path = PathExtImpl::build_path(&node_list);
    ///
    ///     assert_eq!("C:\\Users\\someuser\\folder\\subfolder\\subsubfolder", path);
    /// }
    /// ```
    pub fn build_path(list: &[&str]) -> String {
        PathExtImpl::build_path(list)
    }

    /// Root node of the system. It is meant to be used in `build_path` function.
    /// On Linux and macOS `build_path` function will evaluate it to `/`,
    /// on Windows it will be `C:`
    pub fn root() -> String {
        PathExtImpl::root()
    }


    /// Folder up, or `..`. It is meant to be used in `build_path` function.
    pub fn folder_up() -> String {
        PathExtImpl::folder_up()
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
        UserExtImpl::get_current_user()
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
        UserExtImpl::get_current_user()
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
        UserExtImpl::get_current_user_domain()
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

    /// Returns file length in bytes
    ///```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn length() {
    ///    let expected_length: u64 = 398;
    ///    let pwd = FileExt::working_directory().unwrap();
    ///    let length = FileExt::file_length(vec![pwd.as_str(), "LICENSE"]).unwrap();
    ///
    ///    assert_eq!(expected_length, length);
    /// }
    /// ```
    pub fn file_length(path: Vec<&str>) -> Result<u64, String> {
        FileExtImpl::file_length(path)
    }

    /// Copies file block by block. Block size is 100kb
    ///```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn copy_file() {pub fn copy_file(from: Vec<&str>, to: Vec<&str>)-> Result<(), String> {
    ///    let pwd = FileExt::working_directory().unwrap();
    ///    FileExt::copy_file(vec![pwd.as_str(), "LICENSE"], vec![pwd.as_str(), "LICENSE_copy2"]).unwrap();}
    ///
    ///    let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy2"].as_slice());
    ///    FileExt::delete_file(path.as_str()).unwrap();
    /// }
    /// ```
    pub fn copy_file(from: Vec<&str>, to: Vec<&str>)-> Result<(), String> {
        FileExtImpl::copy_file(from, to)
    }

    /// Copies file block by block. If block size is None it is set to 100kb.
    /// Calls the progress callback at the beginning of the block copy.
    /// Calls the cancel callback at the end of the block copy.
    ///```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn copy_file_with_callback_and_block_size() {
    ///     let block_size : u64 = 1000000;
    ///     let pwd = FileExt::working_directory().unwrap();
    ///     let mut label = "".to_string();
    ///     let progress_callback = |start, end, total| { label = format!("copying block {}-{} of {} bytes", start, end, total).to_string(); };
    ///     let cancel_callback = |_start, _end, _total| { false };
    ///     FileExt::copy_file_with_callbacks(
    ///         vec![pwd.as_str(), "LICENSE"],
    ///         vec![pwd.as_str(), "LICENSE_copy3"],
    ///         Some(block_size),
    ///         progress_callback,
    ///         cancel_callback
    ///     ).unwrap();
    ///
    ///     let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy3"].as_slice());
    ///     FileExt::delete_file(path.as_str()).unwrap();
    /// }
    ///```
    pub fn copy_file_with_callbacks<F: FnMut(u64, u64, u64), C: FnMut(u64, u64, u64) -> bool>
                (
                    from: Vec<&str>,
                    to: Vec<&str>,
                    block_size: Option<u64>,
                    progress_callback: F,
                    cancel_callback: C
                )
                    -> Result<(), String> {
        FileExtImpl::copy_file_with_callbacks(from, to, block_size, progress_callback, cancel_callback)
    }

    /// Copies file block by block starting from specific byte. If block size is None it is set to 100kb.
    /// Calls the progress callback at the beginning of the block copy.
    /// Calls the cancel callback at the end of the block copy.
    ///```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn copy_file_with_callback_and_block_size_starting_from_byte() {
    ///     let block_size : u64 = 1000000;
    ///     let pwd = FileExt::working_directory().unwrap();
    ///     let mut label = "".to_string();
    ///     let progress_callback = |start, end, total| { label = format!("copying block {}-{} of {} bytes", start, end, total).to_string(); };
    ///     let cancel_callback = |_start, _end, _total| { false };
    ///     let starting_byte = 4;
    ///     FileExt::copy_file_with_callbacks_starting_from_byte(
    ///         vec![pwd.as_str(), "LICENSE"],
    ///         vec![pwd.as_str(), "LICENSE_copy4"],
    ///         starting_byte,
    ///         Some(block_size),
    ///         progress_callback,
    ///         cancel_callback
    ///     ).unwrap();
    ///
    ///     let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy4"].as_slice());
    ///     FileExt::delete_file(path.as_str()).unwrap();
    /// }
    ///```
    pub fn copy_file_with_callbacks_starting_from_byte
    <F: FnMut(u64, u64, u64), C: FnMut(u64, u64, u64) -> bool>
    (
        from: Vec<&str>,
        to: Vec<&str>,
        starting_byte: u64,
        block_size: Option<u64>,
        progress_callback: F,
        cancel_callback: C,
    )
        -> Result<(), String> {
        FileExtImpl::copy_file_with_callbacks_starting_from_byte(from, to, starting_byte, block_size, progress_callback, cancel_callback)
    }

    /// Copies file block by block starting from specific byte up to ending byte.
    /// If block size is None it is set to 100kb.
    /// Calls the progress callback at the beginning of the block copy.
    /// Calls the cancel callback at the end of the block copy.
    ///```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn copy_file_with_callback_and_block_size_starting_from_byte_to_ending_byte() {
    ///     let block_size : u64 = 1000000;
    ///     let pwd = FileExt::working_directory().unwrap();
    ///     let mut label = "".to_string();
    ///     let progress_callback = |start, end, total| { label = format!("copying block {}-{} of {} bytes", start, end, total).to_string(); };
    ///     let cancel_callback = |_start, _end, _total| { false };
    ///     let starting_byte = 4;
    ///     let ending_byte = 10;
    ///     FileExt::copy_file_with_callbacks_starting_from_byte_and_ending_at_byte(
    ///         vec![pwd.as_str(), "LICENSE"],
    ///         vec![pwd.as_str(), "LICENSE_copy5"],
    ///         starting_byte,
    ///         ending_byte,
    ///         Some(block_size),
    ///         progress_callback,
    ///         cancel_callback
    ///     ).unwrap();
    ///
    ///     let path = FileExt::build_path(vec![pwd.as_str(), "LICENSE_copy5"].as_slice());
    ///     FileExt::delete_file(path.as_str()).unwrap();
    /// }
    ///```
    pub fn copy_file_with_callbacks_starting_from_byte_and_ending_at_byte
    <F: FnMut(u64, u64, u64), C: FnMut(u64, u64, u64) -> bool>
    (
        from: Vec<&str>,
        to: Vec<&str>,
        starting_byte: u64,
        ending_byte: u64,
        block_size: Option<u64>,
        progress_callback: F,
        cancel_callback: C,
    )
        -> Result<(), String>
    {
        FileExtImpl::copy_file_with_callbacks_starting_from_byte_and_ending_at_byte(from, to, starting_byte, ending_byte, block_size, progress_callback, cancel_callback)
    }
}

