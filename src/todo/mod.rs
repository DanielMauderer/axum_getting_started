use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

const TODO_FILE: &str = "todo.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    name: String,
    description: String,
    status: Status,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum Status {
    Done,
    ToDo,
}

pub fn add(name: &str, description: &str) -> Result<Todo, String> {
    let mut todos = load_todos()?;
    if todos.iter().any(|t| t.name == name) {
        return Err(format!("Todo with name {} already exists", name));
    }
    let todo = Todo {
        name: name.to_string(),
        description: description.to_string(),
        status: Status::ToDo,
    };
    todos.push(todo.clone());
    save_todos(&mut todos)?;
    Ok(todo)
}

pub fn edit(name: &str, description: &str) -> Result<Todo, String> {
    let mut todos = load_todos()?;
    let todo = todos.iter_mut().find(|t| t.name == name);
    if todo.is_none() {
        return Err(format!("Todo with name {} does not exist", name));
    }

    let todo = match todo {
        Some(t) => t,
        None => return Err(format!("Todo with name {} does not exist", name)),
    };

    todo.description = description.to_string();
    let n_todo = todo.clone();
    save_todos(&todos)?;
    Ok(n_todo)
}

pub fn tick(name: &str) -> Result<Todo, String> {
    let mut todos = load_todos()?;
    let todo = todos.iter_mut().find(|t| t.name == name);
    if todo.is_none() {
        return Err(format!("Todo with name {} does not exist", name));
    }
    let todo = match todo {
        Some(t) => t,
        None => return Err(format!("Todo with name {} does not exist", name)),
    };
    todo.status = Status::Done;
    let n_todo = todo.clone();
    save_todos(&todos)?;
    Ok(n_todo)
}

pub fn remove(name: &str) -> Result<(), String> {
    let mut todos = load_todos()?;
    match todos.iter().position(|t| t.name == name) {
        Some(index) => {
            todos.remove(index);
            save_todos(&todos)?;
            Ok(())
        }
        None => Err(format!("Todo with name {} does not exist", name)),
    }
}

pub fn list() -> Result<Vec<Todo>, String> {
    let todos = load_todos()?;
    Ok(todos)
}

fn load_todos() -> Result<Vec<Todo>, String> {
    let file = File::open(TODO_FILE);
    if file.is_err() {
        return Ok(Vec::new());
    }
    let reader = BufReader::new(file.map_err(|e| e.to_string())?);
    let todos: Vec<Todo> = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(todos)
}

fn save_todos(todos: &Vec<Todo>) -> Result<(), String> {
    let file = File::create(TODO_FILE).map_err(|e| e.to_string())?;
    serde_json::to_writer(file, &todos).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn test_list() {
        clear_test_file();
        let todos = list().unwrap();
        assert_eq!(todos.len(), 0);
    }

    #[test]
    #[serial]
    fn test_add() {
        clear_test_file();
        let name = "test";
        let todo = add(name, "test").unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "test");
        assert_eq!(todo.status, Status::ToDo);

        let todos = list().unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].name, name);
        assert_eq!(todos[0].description, "test");
        assert_eq!(todos[0].status, Status::ToDo);
    }

    #[test]
    #[serial]
    fn test_add_duplicate() {
        clear_test_file();
        let name = "test";
        let todo = add(name, "test").unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "test");
        assert_eq!(todo.status, Status::ToDo);

        let todo = add(name, "test").unwrap_err();
        assert_eq!(todo, format!("Todo with name {} already exists", name));
    }

    #[test]
    #[serial]
    fn test_edit() {
        clear_test_file();
        let name = "test";
        let todo = add(name, "test").unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "test");
        assert_eq!(todo.status, Status::ToDo);

        let todo = edit(name, "new test").unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "new test");
        assert_eq!(todo.status, Status::ToDo);

        let todos = list().unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].name, name);
        assert_eq!(todos[0].description, "new test");
        assert_eq!(todos[0].status, Status::ToDo);
    }

    #[test]
    #[serial]
    fn test_edit_not_found() {
        clear_test_file();
        let name = "test";
        let todo = edit(name, "new test").unwrap_err();
        assert_eq!(todo, format!("Todo with name {} does not exist", name));
    }

    #[test]
    #[serial]
    fn test_tick() {
        clear_test_file();
        let name = "test";
        let todo = add(name, "test").unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "test");
        assert_eq!(todo.status, Status::ToDo);

        let todo = tick(name).unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "test");
        assert_eq!(todo.status, Status::Done);

        let todos = list().unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].name, name);
        assert_eq!(todos[0].description, "test");
        assert_eq!(todos[0].status, Status::Done);
    }

    #[test]
    #[serial]
    fn test_tick_not_found() {
        clear_test_file();
        let name = "test";
        let todo = tick(name).unwrap_err();
        assert_eq!(todo, format!("Todo with name {} does not exist", name));
    }

    #[test]
    #[serial]
    fn test_remove() {
        clear_test_file();
        let name = "test";
        let todo = add(name, "test").unwrap();
        assert_eq!(todo.name, name);
        assert_eq!(todo.description, "test");
        assert_eq!(todo.status, Status::ToDo);

        let _ = remove(name).unwrap();

        let todos = list().unwrap();
        assert_eq!(todos.len(), 0);
    }

    #[test]
    #[serial]
    fn test_remove_not_found() {
        clear_test_file();
        let name = "test";
        let todo = remove(name).unwrap_err();
        assert_eq!(todo, format!("Todo with name {} does not exist", name));
    }

    fn clear_test_file() {
        let _ = std::fs::remove_file(TODO_FILE);
    }
}
