use std::env;
use crate::symbol::SYMBOL;

pub struct PathExtImpl;

impl PathExtImpl {
    #[cfg(target_family = "unix")]
    pub fn get_path_separator() -> String {
        SYMBOL.slash.to_string()
    }

    #[cfg(target_family = "windows")]
    pub fn get_path_separator() -> String {
        SYMBOL.reverse_slash.to_string()
    }

    pub fn absolute_path_to_working_directory() -> Result<String, String> {
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
        Ok(working_directory.to_string())
    }

}