// this will be a to do app

use std::env;

// todo <command> <rest>
// todo <add> <todo-name> <description>
// todo <edit> <todo-name> <description>
// todo <tick> <todo-name>
// todo <remove> <todo-name>
// todo <list>

fn main() {
    let args: Vec<String> = env::args().collect();
}
