use dotboard_rs_derive::Object;
use std::path::PathBuf;

pub trait Object {
    fn children(&self) -> Vec<&dyn Object>;
    fn mv(&mut self, other: &mut dyn Object);
    fn object_type(&self) -> &str;
    fn read(&mut self);
    fn write(&self);
}

#[derive(Object)]
pub struct Todo {
    title: String,
    description: String,
    attachments: Data,
    labels: Vec<String>,
    deadlines: Vec<String>,
    assignees: Vec<String>,
}

#[derive(Object)]
pub struct Data {
    files: Vec<PathBuf>,
}

#[derive(Object)]
pub struct Board {
    title: String,
    description: String,
    lists: Vec<List>,
}

#[derive(Object)]
pub struct List {
    title: String,
    description: String,
    todos: Vec<Todo>,
}
