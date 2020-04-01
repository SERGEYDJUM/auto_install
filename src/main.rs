use std::{env, fs, path::Path};
use crate::program::Program;
use url::Url;


pub mod program;
pub mod installed_scan;


fn main() {
    let install_dir = String::from("installers");
    fs::create_dir_all(&install_dir).expect("Dir creating error!");
    let mode = input(&"Scan installed apps before start? (Y/N)");
    if mode == "Y" || mode == "y" {
        mode_with_scan(&install_dir)
    }
    else if mode == "N" || mode == "n" {
        mode_without_scan(&install_dir)
    }
    else {
        println!("Wrong input!");
        println!("Installation aborted!");
    }
}


fn mode_without_scan(install_dir: &str) {
    let path = env::current_dir().expect("Invalid path!");
    let path = path.to_str().expect("Invalid path symbols!");
    let path = format!("{}\\list.ini", path);
    print!("Reading list.ini... ");
    let apps: Vec<Program> = file_to_vector(&path);
    println!("OK");
    println!("Apps will be installed:");
    for app in &apps {
        println!("  {}", app.name)
    }
    let user_input = input(&"Download and install apps? (Y)");
    if user_input == "Y" || user_input == "y" {
        for mut app in apps {
            print!("Downloading {}... ", app.name);
            app.download(&install_dir);
            println!("OK");
            print!("Installing {}... ", app.name);
            app.install();
            println!("OK");
        }
        println!("Installation Completed!");
    }
    else {
        println!("Installation aborted!");
    }


}


fn mode_with_scan(install_dir: &str) { 
    let path = env::current_dir().expect("Invalid path!");
    let path = path.to_str().expect("Invalid path symbols!");
    let path = format!("{}\\list.ini", path);
    print!("Reading list.ini... ");
    let mut apps: Vec<Program> = file_to_vector(&path);
    println!("OK");
    print!("Scaning installed apps... ");
    for app in &mut apps {
        app.check_installed();
    }
    println!("OK");
    let mut staged_apps: Vec<Program> = Vec::new(); 
    println!("Apps found: ");
    println!("  Installed: ");
    for app in apps {
        if app.is_installed {
            println!("      {}", app.name)
        }
        else {
            staged_apps.push(app)
        }
    }

    if staged_apps.is_empty() {
        println!("  Will be installed:");
        println!("      None");
        println!("No programs to install!");
        std::process::exit(0);
    }
    else {
        println!("  Will be installed:");
        for app in &staged_apps {
            println!("      {}", app.name)
        }
    }

    let user_input = input(&"Download and install apps? (Y/N)");
    if user_input == "Y" || user_input == "y" {
        for mut app in staged_apps {
            print!("Downloading {}... ", app.name);
            app.download(&install_dir);
            println!("OK");
            print!("Installing {}... ", app.name);
            app.install();
            println!("OK");
        }
        println!("Installation Completed!");
    }
    else {
        println!("Installation aborted!");
    }
}


fn string_to_pure_rows(text: &String) -> Vec<String> {
    let lines: Vec<&str> = text.split('\n').collect(); 
    let mut rows: Vec<String> = Vec::new(); //Returnable vec
    for row in lines { //Moves lines to rows 
        let row: String = row.to_owned();
        rows.push(row);
    }
    for i in 0..rows.len() { //Deletes syntax symbols
        rows[i] = rows[i]
        .trim()
        .replace("->", "")
        .replace("  ", " ");
    }
    rows
}


fn file_to_vector(path: &String) -> Vec<Program> {
    if !Path::new(&path).exists() {
        panic!("File don't exist!")
    }
    let mut programs_list: Vec<Program> = Vec::new(); //Returnable vec
    let file_content = fs::read_to_string(path).expect("Reading file error!");      
    let file_content = file_content.trim().to_owned();
    if file_content.is_empty() {
        panic!("No programs to install!");
    }
    else {
        let rows = string_to_pure_rows(&file_content);
        for row in rows {
            let tokens: Vec<&str> = row.split(' ').collect(); 
            if tokens.len() > 1 {
                let name = tokens[0];
                let url = Url::parse(tokens[1]).expect("Url parsing error!");
                let mut key: String = String::new();
                if tokens.len() > 2 {
                    key += tokens[2];
                    for i in 3..tokens.len() { //For multi-args silent keys
                        key += " ";
                        key += tokens[i];
                    }
                }
                let filename = String::from(url.clone().as_str());
                let mut filename: Vec<&str> = filename.split("/").collect();
                programs_list.push(
                    Program {
                        name: name.to_lowercase().to_owned(), 
                        url: url,
                        filename: String::from(filename.pop().unwrap()),
                        silent_key: key,
                        path: String::new(),
                        is_installed: false

                    }
                )
            }
            //TODO: Installation from local
        }
    }
    programs_list
}


fn input(message: &str) -> String
{
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read from stdin!");
    String::from(input.trim())
}


#[cfg(test)]
#[test]
fn install_test() {
    let path = env::current_dir().expect("Invalid path!");
    let path = path.to_str().expect("Invalid path symbols!");
    let path = format!("{}\\list.cfg", path);
    let progs: Vec<Program> = file_to_vector(&path);
    fs::create_dir_all("installers").expect("Dir creating error!"); //Recursive
    for mut prog in progs {
        prog.download("installers");
        prog.install();
    }
}


#[test]
fn download_test() {
    let path = env::current_dir().expect("Invalid path!");
    let path = path.to_str().expect("Invalid path symbols!");
    let path = format!("{}\\list.cfg", path);
    let progs: Vec<Program> = file_to_vector(&path);
    fs::create_dir_all("installers").expect("Dir creating error!");
    for mut prog in progs {
        prog.download("installers");
    }
}