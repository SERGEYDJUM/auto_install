pub mod program {
    use std::process::Command;
    use std::env;
    use std::path::Path;
    use url::Url;

    #[derive(Debug)]
    pub struct Program {
        pub name: String,
        pub url: Url,
        pub path: String, //Path to exe
        pub silent_key: String,
        pub is_installed: bool
    }

impl Program {
    pub fn change_path(&mut self, path: &String) {
        self.path = path.to_owned();
    }

    pub fn install(&self) {
        if !self.is_installed {
            Command::new("cmd")
            .args(&["/C", format!("{} {}", self.path, self.silent_key ).as_str()])
            .output()
            .expect("Failed to install!");
        }
    }

    pub fn download(&self) {
        let path = Path::new("./installers");
        env::set_current_dir(path)
            .expect("Unable to set path.");
        Command::new("powershell")
                    .args(&["duma", self.url.as_str()])
                    .output()
                    .expect("Can't execute command to download file.");
    }
}
}