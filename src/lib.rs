#[macro_use] extern crate prettytable;

use std::{path::Path, fs};

use log::error;
use slothserver::server::Server;

pub mod server;


fn path_assembler(pwd: &str, path : Option<&mut String>) -> Result<String, String> {
    match path {
        Some(s) => {
            if !s.ends_with(".json") {
                s.push_str(".json");
            }
            Ok(format!("{pwd}/{s}"))
        },
        None => Err("The path of the file is missing.".to_string())
    }
}

fn load_file(pwd: &str, path: Option<&mut String>) -> Result<String, String> {
    let path = path_assembler(pwd, path)?;

    let content = fs::read_to_string(Path::new(&path)).expect("Impossible to read the file.");
    Ok(content)
}

fn load_server(pwd: &str, path: Option<&mut String>) -> Result<Server, String>{

    let c = load_file(pwd, path)?;
    match serde_json::from_str(&c) {
        Ok(s) => Ok(s),
        Err(e) => {
            error!(" Error while parsing to a Server struct : {e} " );
            Err("File couldn't be parsed. ".to_string())
        }
    }
}

fn save_file(pwd: &str, path: Option<&mut String>, c: String) -> Result<(), String> {
    let path = path_assembler(pwd, path)?;
    match std::fs::write(path, c) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(" std::fs::write error : {e} ");
            Err("An error occured when writing the file.".to_string())
        },
    } 
}

fn save_server(pwd: &str, path: Option<&mut String>, server: Server) -> Result<(), String> {
    let server = match serde_json::to_string(&server) {
        Ok(s) => s,
        Err(e) => {
            error!(" Failed to serialize Server : {e} ");
            return Err("Failed to saved the Server.".to_string());
        },
    };
    save_file(pwd, path, server)
}

fn does_file_exist(pwd: &str, path: Option<&mut String>) -> Result<(), String> {
    let path = path_assembler(pwd, path)?;
    let p = Path::new(&path);
    if p.exists() {
        Err( format!("{path} already exists" ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod file_handling_tests {

    use super::*;

    #[test]
    pub fn load_file_test() {
        let pwd = std::env::current_dir().expect("Unable to obtain the pwd.").display().to_string();
        assert!(path_assembler(&pwd, Some(&mut "test.json".to_string())).is_ok());
        assert!(path_assembler(&pwd, Some(&mut "test".to_string())).is_ok());
    }

    #[test]
    pub fn save_file_test() {
        let pwd = std::env::current_dir().expect("Unable to obtain the pwd.").display().to_string();
        assert!(save_file(&pwd, Some(&mut "new".to_string()), r#"{"hello": "word"}"#.to_string()).is_ok())
    }
}