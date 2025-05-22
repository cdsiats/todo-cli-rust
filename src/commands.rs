use crate::{storage::{read_todos, write_todos}, todo::Todo};

pub fn add(task: &str) {
    let mut todos = read_todos();

    let next_id = todos.last().map(|t| t.id + 1).unwrap_or(1);

    let todo = Todo {
        id: next_id,
        task: task.to_string(),
        done: false,
    };

    todos.push(todo);
    write_todos(&todos);

    println!("Added: \"{}\"", task);
}

pub fn list() {
    let todos = read_todos();

    if todos.is_empty() {
        println!("No tasks found.");
        return;
    }

    for todo in &todos {
        println!("{}: {}", todo.id, todo.task);
    }
}

pub fn remove(id: u32) {
    let mut todos = read_todos();
    todos.retain(|todo| todo.id != id);
    write_todos(&todos);
}