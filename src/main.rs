use std::{fs, path::Path};

fn main() {
    let args : Vec<String> = std::env::args().collect();

    println!(" {args:?} ");

    let pwd = std::env::current_dir().expect("Unable to obtain the pwd.").display().to_string();
    let cmd = args.get(1).expect("Command was expected. Run help to get the list of commands.");
    let result: Result<(), String> = match &cmd as &str {
        "serve" => serve(&pwd, args.get(2)),
        _ => Err("Invalid Command".into()),
    };

    match result {
        Ok(_) => {},
        Err(e) => println!(" {e} ")
    }
}


fn load_file(pwd: &str, path: Option<&String>) -> Result<String, String> {
    let path = match path {
        Some(s) => {
            format!("{pwd}/{s}")
        },
        None => return Err("The path of the file is missing.".to_string())
    };

    let content = fs::read_to_string(Path::new(&path)).expect("Impossible to read the file.");
    Ok(content)
}

fn serve(pwd: &str, path: Option<&String>) -> Result<(), String> {
    let content = load_file(pwd, path)?;
    slothserver::serve(&content)?;
    Ok(())
}