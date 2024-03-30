use diesel::prelude::*;
use crate::schema::todos;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub content: &'a str,
}