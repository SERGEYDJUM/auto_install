pub mod program {
    use std::process::Command;
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
}
}