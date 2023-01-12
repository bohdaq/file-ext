#[cfg(test)]
mod tests;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct FileExtImpl;

impl FileExtImpl {
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

    pub fn create_file(path: &str) -> Result<(), String>  {
        let boxed_file = File::create(path);

        if boxed_file.is_err() {
            let message = format!("unable to create file: {}", boxed_file.err().unwrap());
            return Err(message)
        }

        boxed_file.unwrap();
        Ok(())
    }

    pub fn does_file_exist(path: &str) -> bool {
        let file_exists = Path::new(path).is_file();
        file_exists
    }

    pub fn delete_file(path: &str) -> Result<(), String> {
        let boxed_remove = fs::remove_file(path);
        if boxed_remove.is_err() {
            let msg = boxed_remove.err().unwrap().to_string();
            return Err(msg)
        }

        Ok(())
    }
}