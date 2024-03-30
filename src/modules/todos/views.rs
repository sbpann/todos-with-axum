use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct Todos {
    pub todos: Vec<Todo>
}