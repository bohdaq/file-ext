#[cfg(test)]
mod tests;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;
use crate::FileExt;
use crate::filter_string::FilterString;

pub struct FileExtImpl;

impl FileExtImpl {
    pub fn read_file(filepath: &str) -> Result<Vec<u8>, String> {
        let boxed_check = FilterString::is_valid_input_string(filepath);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

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
        let boxed_check = FilterString::is_valid_input_string(filepath);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

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
        let boxed_check = FilterString::is_valid_input_string(path);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

        let does_file_exist = Self::does_file_exist(path);
        return if does_file_exist {
            let boxed_read = Self::read_file(path);
            if boxed_read.is_err() {
                return Err(boxed_read.err().unwrap());
            }
            let file_content = boxed_read.unwrap();
            Ok(file_content)
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
        let boxed_check = FilterString::is_valid_input_string(path);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .create(false)
            .truncate(false)
            .open(path)
            .unwrap();

        let boxed_seek = file.seek(SeekFrom::End(0));
        if boxed_seek.is_err() {
            let message = boxed_seek.err().unwrap().to_string();
            return Err(message);
        }

        boxed_seek.unwrap();

        let boxed_write = file.write_all(file_content);
        if boxed_write.is_err() {
            let message = format!("unable to write to file: {}", boxed_write.err().unwrap());
            return Err(message)
        }
        Ok(())
    }

    pub fn create_file(path: &str) -> Result<(), String>  {
        let boxed_check = FilterString::is_valid_input_string(path);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }

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
        let boxed_check = FilterString::is_valid_input_string(path);
        if boxed_check.is_err() {
            let message = boxed_check.err().unwrap();
            return Err(message)
        }
        
        let boxed_remove = fs::remove_file(path);
        if boxed_remove.is_err() {
            let msg = boxed_remove.err().unwrap().to_string();
            return Err(msg)
        }

        Ok(())
    }

    pub fn copy_part_of_file(from: Vec<&str>, to: Vec<&str>, start: u64, end: u64) -> Result<(), String> {
        let from_path = FileExt::build_path(&from);
        let file_exists = FileExt::does_file_exist(from_path.as_str());
        if !file_exists {
            let message = format!("file at given path {} does not exist", from_path.as_str());
            return Err(message);
        }


        let boxed_content_to_copy = FileExt::read_file_partially(from_path.as_str(), start, end);
        if boxed_content_to_copy.is_err() {
            let message = boxed_content_to_copy.err().unwrap();
            return Err(message);
        }
        let content_to_copy = boxed_content_to_copy.unwrap();


        let to_path = FileExt::build_path(&to);
        if !FileExt::does_file_exist(to_path.as_str()) {
            let boxed_create = FileExt::create_file(to_path.as_str());
            if boxed_create.is_err() {
                let message = boxed_create.err().unwrap();
                return Err(message);
            }
        }


        let boxed_write =
            FileExt::write_file(to_path.as_str(), content_to_copy.as_slice());
        if boxed_write.is_err() {
            let message = boxed_write.err().unwrap();
            return Err(message);
        }
        Ok(boxed_write.unwrap())
    }

    pub fn copy_file(from: Vec<&str>, to: Vec<&str>)-> Result<(), String> {
        let boxed_length = FileExtImpl::file_length(from.clone());
        if boxed_length.is_err() {
            let message = boxed_length.err().unwrap();
            return Err(message);
        }

        let file_length = boxed_length.unwrap();
        let _100kb = 102400;
        let step = _100kb;
        let mut start = 0;
        let mut end = step;
        if step >= file_length {
            end = file_length - 1;
        }

        let mut continue_copying = true;
        while continue_copying {
            let boxed_copy = FileExtImpl::copy_part_of_file(
                from.clone(),
                to.clone(),
                start,
                end
            );

            if boxed_copy.is_err() {
                let message = boxed_copy.err().unwrap();
                return Err(message);
            }

            boxed_copy.unwrap();

            if end == file_length - 1 {
                continue_copying = false;
            } else {
                start = end + 1;
                end = end + step;
                if start + step >= file_length {
                    end = file_length - 1;
                }
            }

        }

        Ok(())
    }

    pub fn copy_file_with_callbacks
            <F: FnMut(u64, u64, u64), C: FnMut(u64, u64, u64) -> bool>
                (
                    from: Vec<&str>,
                    to: Vec<&str>,
                    block_size: Option<u64>,
                    mut progress_callback: F,
                    mut cancel_callback: C,
                )
        -> Result<(), String> {
        let boxed_length = FileExtImpl::file_length(from.clone());
        if boxed_length.is_err() {
            let message = boxed_length.err().unwrap();
            return Err(message);
        }

        let file_length = boxed_length.unwrap();
        let _100kb = 102400;
        let mut step = _100kb;
        if block_size.is_some() {
            step = block_size.unwrap();
        }
        let mut start = 0;
        let mut end = start + step;
        if step >= file_length {
            end = file_length - 1;
        }

        let mut continue_copying = true;
        while continue_copying {
            progress_callback(start, end, file_length);
            let boxed_copy = FileExtImpl::copy_part_of_file(
                from.clone(),
                to.clone(),
                start,
                end
            );

            if boxed_copy.is_err() {
                let message = boxed_copy.err().unwrap();
                return Err(message);
            }

            boxed_copy.unwrap();

            let copying_cancelled_by_user = cancel_callback(start, end, file_length);
            let reached_end_of_file = end == file_length - 1;

            if reached_end_of_file || copying_cancelled_by_user {
                continue_copying = false;
            } else {
                start = end + 1;
                end = end + step;
                if start + step >= file_length {
                    end = file_length - 1;
                }
            }

        }

        Ok(())
    }

    pub fn file_length(path: Vec<&str>) -> Result<u64, String> {
        let filepath = FileExt::build_path(path.as_slice());
        let boxed_length = fs::metadata(filepath);
        if boxed_length.is_err() {
            let message = boxed_length.err().unwrap().to_string();
            return Err(message)
        }
        let length = boxed_length.unwrap().len();
        Ok(length)
    }
}

