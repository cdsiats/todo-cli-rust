use std::{fs::{File, OpenOptions}, io::{Read, Write}};

use crate::todo::Todo;

const FILE_PATH: &str = "todos.json";

pub fn read_todos() -> Vec<Todo> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)
        .expect("Failed to open file.");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if contents.trim().is_empty() {
        vec![]
    } else {
        serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
    }
}

pub fn write_todos(todos: &[Todo]) {
    let json = serde_json::to_string(&todos).unwrap();
    let mut file = File::create(FILE_PATH).expect("Failed to create file.");
    file.write_all(json.as_bytes()).expect("Failed to write to file.");
}