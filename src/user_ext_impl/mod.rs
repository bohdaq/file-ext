use std::process::Command;

#[cfg(test)]
mod tests;

pub struct UserExtImpl {}

impl UserExtImpl {
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
}