use std::str;
use std::process::Command;

pub fn app_installed_on_machine(app_name: &String) -> bool {
    
    let output = Command::new("powershell")
        .arg("reg query HKEY_LOCAL_MACHINE\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall /v DisplayName /s")
        .output()
        .expect("Failed to execute command")
        .stdout;
    // output contains all applications installed on machine
    let output = str::from_utf8(&output)
        .expect("Error converting output to &str");
    output.to_ascii_lowercase()
        .contains(&app_name.to_ascii_lowercase())
}


pub fn app_installed_locally(app_name: &String) -> bool {
    let output = Command::new("powershell")
        .arg(format!("reg query HKEY_CURRENT_USER\\Software\\ /s /f {} /k", app_name))
        .output()
        .expect("Failed to execute command")
        .stdout;
    // output contains all applications installed locally
    let output = str::from_utf8(&output)
        .expect("Error converting output to &str");
    output.to_ascii_lowercase()
        .contains(&app_name.to_ascii_lowercase())
}


pub fn app_installed(app_name: &String) -> bool {
    app_installed_locally(app_name) || app_installed_on_machine(app_name)
}