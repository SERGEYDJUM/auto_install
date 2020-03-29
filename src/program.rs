pub use crate::installed_check::app_installed;
use std::{process::Command, path::Path, env};
use url::Url;


#[derive(Debug)]
pub struct Program {
    pub name: String,
    pub url: Url,
    pub path: String,
    pub filename: String,
    pub silent_key: String,
    pub is_installed: bool
}


impl Program {
    pub fn change_path(&mut self, path: &String) {
        self.path = path.to_owned();
    }


    pub fn install(&mut self) {
        if !self.is_installed {
            Command::new("powershell")
            .arg(format!("{} {}", self.path, self.silent_key ).as_str())
            .output()
            .expect("Failed to install!");
            self.is_installed = true;
        }
    }


    pub fn download(&mut self, install_dir: &str) {
        let path = String::from(env::current_dir().expect("Current dir is invalid!").to_str().expect("Conversion error!"));
        let path = format!("{}\\{}\\{}", path, install_dir, self.filename);
        if !Path::new(&path).exists() { //If the file is already downloaded
            Command::new("powershell")
                    .arg(format!("
                        Import-Module BitsTransfer
                        Start-BitsTransfer -Source {} -Destination {} \
                        -Description \"Downloading {}\" 
                        Write-Host
                        ", String::from(self.url.as_str()), path, self.filename))
                    .output()
                    .expect("BITS startup error!");
        }
        self.change_path(&path);
    }

    
    /// Check if the program is installed on computer and change the "is_installed" field
    pub fn check_installed(&mut self) {
        self.is_installed = app_installed(&self.name)
    }
}