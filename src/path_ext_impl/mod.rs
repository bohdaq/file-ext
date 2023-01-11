use std::env;
use crate::symbol::SYMBOL;
use crate::user_ext_impl::UserExtImpl;

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

    #[cfg(target_family = "unix")]
    pub fn get_temp_folder_path() -> Result<String, String>{
        Ok("/tmp".to_string())
    }

    #[cfg(target_family = "windows")]
    pub fn get_temp_folder_path() -> Result<String, String>{
        let boxed_username = UserExtImpl::get_current_user();
        if boxed_username.is_err() {
            let message = boxed_username.err().unwrap().to_string();
            return Err(message)
        }

        let username = boxed_username.unwrap();
        let path = ["C:", "Users", username.as_str(), "AppData", "Local", "Temp"].join(PathExtImpl::get_path_separator().as_str());
        Ok(path)
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