pub mod program {
    use std::process::Command;
    use std::env;
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

    pub fn install(&self) {
        if !self.is_installed {
            Command::new("cmd")
            .args(&["/C", format!("{} {}", self.path, self.silent_key ).as_str()])
            .output()
            .expect("Failed to install!");
        }
    }

    pub fn download(&mut self) {
        let path = String::from(env::current_dir().expect("Current dir is invalid!").to_str().expect("Conversion error!"));
        let path = format!("{}\\installers\\{}", path, self.filename);
        Command::new("powershell")
                    .args(&[format!("
                        $url = \"{}\"
                        $output = \"{}\"
                        $start_time = Get-Date
                        Import-Module BitsTransfer
                        Start-BitsTransfer -Source $url -Destination $output
                        Write-Output \"Time taken: $((Get-Date).Subtract($start_time).Seconds) second(s)\"
                    ", String::from(self.url.as_str()), path)])
                    .output()
                    .expect("Can't execute command to download file.");
        self.change_path(&path);
    }
}
}