use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::process::Command;
use crate::date_time_ext::DateTimeExt;
use crate::symbol::SYMBOL;

#[cfg(test)]
mod tests;
mod date_time_ext;
mod symbol;

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
        let mut file_content = Vec::new();

        let buff_length = (end - start) + 1;
        let boxed_open = File::open(filepath);
        if boxed_open.is_err() {
            let error_msg = boxed_open.err().unwrap();
            let error = format!("<p>Unable to open file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        }

        let file = boxed_open.unwrap();
        let mut reader = BufReader::new(file);

        let boxed_seek = reader.seek(SeekFrom::Start(start));
        if boxed_seek.is_ok() {
            let boxed_read = reader.take(buff_length).read_to_end(&mut file_content);
            if boxed_read.is_err() {
                let error_msg = boxed_read.err().unwrap().to_string();
                let error = format!("<p>Unable to read file: {}</p> <p>error: {}</p>", filepath, error_msg);
                return Err(error)
            }
        } else {
            let error_msg = boxed_seek.err().unwrap().to_string();
            let error = format!("<p>Unable to seek file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        }

        Ok(file_content)
    }

    /// Returns file content
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn file_content() {
    ///     let path = "test/index.html";
    ///     let file_raw_bytes = FileExt::read_file(path).unwrap();
    ///     let content = String::from_utf8(file_raw_bytes).unwrap();
    ///
    ///     let expected_content = "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <meta charset=\"UTF-8\">\n    <title>Title</title>\n</head>\n<body>\n\n</body>\n</html>";
    ///
    ///     assert_eq!(expected_content, content);
    /// }
    /// ```
    pub fn read_file(filepath: &str) -> Result<Vec<u8>, String> {

        let mut file_content = Vec::new();
        let boxed_open = File::open(filepath);
        if boxed_open.is_err() {
            let error_msg = boxed_open.err().unwrap();
            let error = format!("<p>Unable to open file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        } else {
            let mut file = boxed_open.unwrap();
            let boxed_read= file.read_to_end(&mut file_content);
            if boxed_read.is_err() {
                let error_msg = boxed_read.err().unwrap();
                let error = format!("<p>Unable to read file: {}</p> <p>error: {}</p>", filepath, error_msg);
                return Err(error)
            }
        }
        Ok(file_content)
    }

    /// Returns file modification timestamp as nanoseconds in Unix epoch
    /// # Examples
    ///
    /// ```
    ///     use file_ext::FileExt;
    ///     #[test]
    ///     fn modification_timestamp() {
    ///         let expected_timestamp : u128 = 1672145538390040940;
    ///         let modified_timestamp = FileExt::file_modified_utc("test/index.html").unwrap();
    ///         assert_eq!(expected_timestamp, modified_timestamp);
    ///     }
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
    ///     let expected = "/".to_string();
    ///     let actual = FileExt::get_path_separator();
    /// }
    /// ```
    pub fn get_path_separator() -> String {
        SYMBOL.slash.to_string()
    }

    #[cfg(target_family = "windows")]
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn unix_path_delimiter() {
    ///     let expected = "\\".to_string();
    ///     let actual = FileExt::get_path_separator();
    /// }
    /// ```
    pub fn get_path_separator() -> String {
        "\\".to_string()
    }


    /// Will return absolute file path
    pub fn get_static_filepath(path: &str) -> Result<String, String> {
        let boxed_dir = env::current_dir();
        if boxed_dir.is_err() {
            let error = boxed_dir.err().unwrap();
            eprintln!("{}", error);
            return Err(error.to_string());
        }
        let dir = boxed_dir.unwrap();


        let boxed_working_directory = dir.as_path().to_str();
        if boxed_working_directory.is_none() {
            let error = "working directory is not set";
            eprintln!("{}", error);
            return Err(error.to_string());
        }

        let working_directory = boxed_working_directory.unwrap();
        let absolute_path = [working_directory, path].join(SYMBOL.empty_string);
        Ok(absolute_path)
    }

    /// Will try to read from file. If file does not exist, will create and write to it given byte array
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn read_or_write() {
    ///     let content = "data".as_bytes();
    ///     let path = "/tmp/test.txt";
    ///
    ///     let doesnt_exist = !FileExt::does_file_exist(path);
    ///     assert!(doesnt_exist);
    ///
    ///     FileExt::read_or_create_and_write(path, content).unwrap();
    ///
    ///     let does_exist = FileExt::does_file_exist(path);
    ///     assert!(does_exist);
    ///
    ///     let new_content = "updated data".as_bytes();
    ///     let content_from_file = FileExt::read_or_create_and_write(path, new_content).unwrap();
    ///
    ///     assert_eq!(content, content_from_file);
    ///
    ///     FileExt::delete_file(path).unwrap();
    ///     let doesnt_exist = !FileExt::does_file_exist(path);
    ///     assert!(doesnt_exist);
    /// }
    /// ```
    pub fn read_or_create_and_write(path: &str, content: &[u8]) -> Result<Vec<u8>, String> {
        let does_passphrase_exist = Self::does_file_exist(path);
        return if does_passphrase_exist {
            let boxed_read = Self::read_file(path);
            if boxed_read.is_err() {
                return Err(boxed_read.err().unwrap());
            }
            let passphrase = boxed_read.unwrap();
            Ok(passphrase)
        } else {
            let boxed_create = Self::create_file(path);
            if boxed_create.is_err() {
                let message = boxed_create.err().unwrap();
                return Err(message)
            }

            let boxed_write = Self::write_file(path, content);
            if boxed_write.is_err() {
                let message = boxed_write.err().unwrap();
                return Err(message)
            }
            Ok(Vec::from(content))
        }
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
        let boxed_file = File::create(path);

        if boxed_file.is_err() {
            let message = format!("unable to create file: {}", boxed_file.err().unwrap());
            return Err(message)
        }

        boxed_file.unwrap();
        Ok(())
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
        let file_exists = Path::new(path).is_file();
        file_exists
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
        let file_exists = Path::new(path).is_file();
        file_exists
    }

    /// Returns boolean indicating directory existence on the path
    /// # Examples
    ///
    /// ```
    /// use file_ext::FileExt;
    /// #[test]
    /// fn symlink_exists() {
    ///     let path = "test/index_rewrite";
    ///     let exists = FileExt::does_symlink_exist(path);
    ///     assert!(exists);
    /// }
    /// ```
    pub fn does_symlink_exist(path: &str) -> bool {
        Path::new(path).is_symlink()
    }

    /// Will write given byte array to a file on the path
    pub fn write_file(path: &str, file_content: &[u8]) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(false)
            .truncate(false)
            .open(path)
            .unwrap();

        file.seek(SeekFrom::End(0)).unwrap();

        let boxed_write = file.write_all(file_content);
        if boxed_write.is_err() {
            let message = format!("unable to write to file: {}", boxed_write.err().unwrap());
            return Err(message)
        }
        Ok(())
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
        let boxed_remove = fs::remove_file(path);
        if boxed_remove.is_err() {
            let msg = boxed_remove.err().unwrap().to_string();
            return Err(msg)
        }

        Ok(())
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
        //check if there is already a file where symlink is going to be created
        let path_to_symlink_included = [symlink_path, symlink_name].join("");
        let does_file_exist = FileExt::does_file_exist(&path_to_symlink_included);
        if does_file_exist {
            let message = format!("There is a file on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }
        let does_directory_exist = FileExt::does_directory_exist(&path_to_symlink_included);
        if does_directory_exist {
            let message = format!("There is a directory on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }

        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExt::does_file_exist(symlink_points_to);
        let does_directory_exist = FileExt::does_directory_exist(symlink_points_to);

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
        //check if there is already a file where symlink is going to be created
        let path_to_symlink_included = [symlink_path, symlink_name].join("");
        let does_file_exist = FileExt::does_file_exist(&path_to_symlink_included);
        if does_file_exist {
            let message = format!("There is a file on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }
        let does_directory_exist = FileExt::does_directory_exist(&path_to_symlink_included);
        if does_directory_exist {
            let message = format!("There is a directory on a given path: {}", &path_to_symlink_included);
            return Err(message)
        }

        //check if there is a file or directory for symlink to be created
        let does_file_exist = FileExt::does_file_exist(symlink_points_to);
        let does_directory_exist = FileExt::does_directory_exist(symlink_points_to);

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

    #[cfg(target_family = "windows")]
    pub fn get_temp_folder_path() -> Result<String, String>{
        let boxed_username = FileExt::get_current_user();
        if boxed_username.is_err() {
            let message = boxed_username.err().unwrap().to_string();
            return Err(message)
        }

        let username = boxed_username.unwrap();
        let path = ["C:", "Users", username.as_str(), "AppData", "Local", "Temp"].join(FileExt::get_path_separator().as_str());
        Ok(path)
    }

    #[cfg(target_family = "unix")]
    pub fn get_temp_folder_path() -> Result<String, String>{
        Ok("/tmp".to_string())
    }
}

