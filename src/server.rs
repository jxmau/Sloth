use std::fs::remove_file;

use log::error;
use prettytable::{Table, row, format};
use serde_json::from_str;
use slothserver::{server::Server, route::Route};
use crate::{load_file, save_file, does_file_exist, load_server, save_server, path_assembler};

pub fn parse_server(pwd: String, vec: &mut Vec<String>) -> Result<(), String> {
    match vec.get(2) {
        None => Err("No command specified for the mock server module or no path to the server file to enter the Route module. Use the help command to get help.".to_string()),
        Some(c) => {
            match &c as &str {
                // sloth sever run <file>
                "run" | "r" => serve(&pwd, vec.get_mut(3)),
                // sloth server new <path> <port?> | no need for json.
                "new" | "n" => new(&pwd, vec),
                // sloth server add <path> <name> <relative_path> <method> <status?>
                "add" | "a" => add_route(&pwd, vec),
                // sloth server list <path>
                "list" | "l" => list(&pwd, vec.get_mut(3)),
                // sloth server delete <path>
                "delete" | "d" => delete_server(&pwd, vec.get_mut(3)),
                // sloth serber delete_route <path> <od>
                "delete_route" | "dr" => delete_route(&pwd, vec),
                // sloth server reorganize <path>
                "reorganize" | "rg" => reorganize(&pwd, vec),
                _ => Err("Module Route hasn't been implemented yet.".to_string())
            }
        }
    }

}

fn serve(pwd: &str, path: Option<&mut String>) -> Result<(), String> {
    let content = load_file(pwd, path)?;
    slothserver::serve(&content)
}

fn new(pwd: &str, vec: &mut Vec<String>) -> Result<(), String> {
    // let path = path_assembler(pwd, path)?;
    // FIXME:This unwrap is secured, but should be be better handled.
    let port : u16 = from_str::<u16>(vec.get(4).unwrap_or(&&mut "8080".to_string())).unwrap();
    let server = Server::new(port);
    let server = match serde_json::to_string(&server) {
        Ok(s) => s,
        Err(e) => {
            error!(" Failed to serialize Server : {e} ");
            return Err("Failed to saved the Server.".to_string());
        },
    };
    does_file_exist(pwd, vec.get_mut(3))?;
    save_file(pwd, vec.get_mut(3), server)
}

// sloth server add <path> <name> <relative_path> <method> <status?>
fn add_route(pwd: &str, vec: &mut Vec<String>) -> Result<(), String> {
    let mut server = load_server(pwd, vec.get_mut(3))?;
    
    match (vec.get(4), vec.get(5), vec.get(6)) {
        (Some(name), Some(path), Some(method)) => {
            // TODO: Many unwraps to handle.
            let mut route = Route::new(name, path, method, Some(server.get_routes().len().try_into().unwrap()));
            route.response.status = from_str::<u16>(vec.get(7).unwrap_or(&"200".to_string())).unwrap();
            server.add_route(route);
            save_server(pwd, vec.get_mut(3), server)?;
            return Ok(())
        },
        (_, _, _)=> {
            error!(" Command line received : {vec:?} ");
            return Err("Invalid arguments given.".to_string())
        }
    }
}
    

fn list(pwd: &str, path: Option<&mut String>)  -> Result<(), String> {

    let server = load_server(pwd, path)?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER);
    table.add_row(row!["Order", "Name", "Path", "Method", "Status", "Headers", "Cookies"]);
    for r in server.get_routes() {
        table.add_row(row![r.order.unwrap_or(0), r.name, r.path, r.method, r.response.status, r.response.headers.len(), r.response.cookies.len() ]);
    }
    table.printstd();
    Ok(())
}

fn delete_server(pwd: &str, path: Option<&mut String>) -> Result<(), String> {
    let path = path_assembler(pwd, path)?;
    match remove_file(path) {
        Ok(_) =>   Ok(()),
        Err(e) => {
            error!(" Error when trying to delte the mock Server file : {e} ");
            Err("Failed to delete Mock Server.".to_string())
        },
    }
  
}

fn delete_route(pwd: &str, vec: &mut Vec<String>) -> Result<(), String> {
    let mut server = load_server(pwd, vec.get_mut(3))?;
    let od : u16 = match vec.get(4) {
        Some(s) => match from_str(&s) {
            Ok(s) => s,
            Err(e) => {
                error!(" Couldn't parse the od into u16 : {e}");
                return Err("Invalid Route od.".to_string())
            }
        },
        None => {
            error!("Failed to find the od to delete");
            return Err("Od of route couldn't be found.".to_string())
        }
    }; 

    server.delete_route(od - 1);
    save_server(&pwd, vec.get_mut(3), server)?;
    Ok(())
}

fn reorganize(pwd: &str, vec: &mut Vec<String>) -> Result<(), String> {
    let mut server = load_server(&pwd, vec.get_mut(3))?;
    server.reorganize_routes();
    save_server(&pwd, vec.get_mut(3), server)?;
    Ok(())
}