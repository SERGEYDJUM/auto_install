use std::{process::Command, env, fs};
use crate::program::program::Program;
use url::Url;

pub mod program;

fn main() {
    
}

fn check_installed(programs: &mut Vec<Program>) {
    let apps_list = Command::new("powershell")
        .args(&["Get-ItemProperty HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Select-Object DisplayName"])
        .output()
        .expect("Failed to check apps!");
    let apps_list = unsafe {String::from_utf8_unchecked(apps_list.stdout)};
    //TODO: Make this code safe
    let apps_list = apps_list.to_lowercase();
    for program in programs {
        if apps_list.contains(&program.name) {
            program.is_installed = true;
        }
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


#[cfg(test)]
#[test]
fn install_test() {
    let path = env::current_dir().expect("Invalid path!");
    let path = path.to_str().expect("Invalid path symbols!");
    let path = format!("{}\\list.cfg", path);
    let progs: Vec<Program> = file_to_vector(&path);
    for mut prog in progs {
        prog.download();
        prog.install();
    }
}

#[test]
fn download_test() {
    let path = env::current_dir().expect("Invalid path!");
    let path = path.to_str().expect("Invalid path symbols!");
    let path = format!("{}\\list.cfg", path);
    let progs: Vec<Program> = file_to_vector(&path);
    for mut prog in progs {
        prog.download();
    }
}