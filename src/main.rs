use relative_path::RelativePath;
use std::path::Path;
use std::env;
use std::fs;
//use std::io;
//use reqwest; 

fn main() {

}

#[derive(Debug)]

struct Program {
    name: String,
    url: String,
    path: String, //Path to exe
    silent_key: String
}

impl Program {
    fn change_path(&mut self, path: &String) {
        self.path = path.to_owned();
    }
}

fn file_to_vector(path: &Path) -> Vec<Program> {
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
                let url = tokens[1];
                let mut key: String = String::new();
                if tokens.len() > 2 {
                    key += tokens[2];
                    for i in 3..tokens.len() { //For multi-args silent keys
                        key += " ";
                        key += tokens[i];
                    }
                }

                programs_list.push(
                    Program {
                        name: name.to_owned(), 
                        url: url.to_owned(),
                        silent_key: key,
                        path: String::new()
                    }
                )
            }
            //TODO: Installation from local
        }
    }

    programs_list
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

#[cfg(test)]
#[test]
fn read_test() {
    let dir = env::current_dir().unwrap();
    let dir = dir.to_str().unwrap();
    let path = RelativePath::new("/src/list.cfg").to_path(Path::new(&dir)); 
    let mut progs: Vec<Program> = file_to_vector(&path);
    let path: String = String::from(path.to_str().unwrap());
    progs[0].change_path(&path);
    for prog in progs {
        println!("{:?}", prog);
    }
}