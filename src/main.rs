// this will be a to do app

use std::env;

use todolib::todo;

// todo <command> <rest>

// todo add <todo-name> <description>
// todo edit <todo-name> <description>
// todo tick <todo-name>
// todo remove <todo-name>
// todo list

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: todo <command> <rest>");
        return;
    }

    let command = &args[1];
    let rest = &args[2..];

    match command.as_str() {
        "add" => {
            let name = &rest[0];
            let description = &rest[1];
            match todo::add(name, description) {
                Ok(todo) => println!("Added todo: {:?}", todo),
                Err(e) => println!("Error: {}", e),
            }
        }
        "edit" => {
            let name = &rest[0];
            let description = &rest[1];
            match todo::edit(name, description) {
                Ok(todo) => println!("Edited todo: {:?}", todo),
                Err(e) => println!("Error: {}", e),
            }
        }
        "tick" => {
            let name = &rest[0];
            match todo::tick(name) {
                Ok(todo) => println!("Ticked todo: {:?}", todo),
                Err(e) => println!("Error: {}", e),
            }
        }
        "remove" => {
            let name = &rest[0];
            match todo::remove(name) {
                Ok(_) => println!("Removed todo: {}", name),
                Err(e) => println!("Error: {}", e),
            }
        }
        "list" => match todo::list() {
            Ok(todos) => {
                for todo in todos {
                    println!("{:?}", todo);
                }
            }
            Err(e) => println!("Error: {}", e),
        },
        _ => {
            println!("Usage: todo <command> <rest>");
        }
    }
}
