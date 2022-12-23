use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use rust_web_server::ext::date_time_ext::DateTimeExt;
use rust_web_server::range::Range;
use rust_web_server::symbol::SYMBOL;

#[cfg(test)]
mod tests;

pub struct FileExt;

impl FileExt {
    pub fn read_file_partially(filepath: &str, range: &Range) -> Result<Vec<u8>, String> {
        let mut file_content = Vec::new();

        let buff_length = (range.end - range.start) + 1;
        let boxed_open = File::open(filepath);
        if boxed_open.is_err() {
            let error_msg = boxed_open.err().unwrap();
            let error = format!("<p>Unable to open file: {}</p> <p>error: {}</p>", filepath, error_msg);
            return Err(error)
        }

        let file = boxed_open.unwrap();
        let mut reader = BufReader::new(file);

        let boxed_seek = reader.seek(SeekFrom::Start(range.start));
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

    pub fn get_static_filepath(request_uri: &str) -> Result<String, String> {
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
        let absolute_path = [working_directory, request_uri].join(SYMBOL.empty_string);
        Ok(absolute_path)
    }

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

    pub fn create_file(path: &str) -> Result<File, String>  {
        let boxed_file = File::create(path);

        if boxed_file.is_err() {
            let message = format!("unable to create file: {}", boxed_file.err().unwrap());
            return Err(message)
        }

        let file = boxed_file.unwrap();
        Ok(file)
    }

    pub fn does_file_exist(path: &str) -> bool {
        let file_exists = Path::new(path).is_file();
        file_exists
    }

    pub fn write_file(path: &str, file_content: &[u8]) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(false)
            .truncate(false)
            .open(path)
            .unwrap();
        let boxed_write = file.write_all(file_content);
        if boxed_write.is_err() {
            let message = format!("unable to write to file: {}", boxed_write.err().unwrap());
            return Err(message)
        }
        Ok(())
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

