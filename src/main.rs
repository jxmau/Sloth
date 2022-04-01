use log::trace;
use sloth::server::parse_server;

fn main() {

    env_logger::init();
    let mut args : Vec<String> = std::env::args().collect();

    trace!(" {args:?} ");

    let pwd = std::env::current_dir().expect("Unable to obtain the pwd.").display().to_string();
    let cmd = args.get(1).expect("Command was expected. Run help to get the list of commands.");
    let result: Result<(), String> = match &cmd as &str {
        "server"|"s" => parse_server(pwd,&mut args),
        _ => Err("Invalid Command".into()),
    };

    match result {
        Ok(_) => {},
        Err(e) => println!("Error : {e} ")
    }
}

