use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use super::models;
use crate::{schema, ApplicationState};

pub struct TodoService {
    app_state: ApplicationState,
}
impl TodoService {
    pub fn new(app_state: ApplicationState) -> Self {
        return Self { app_state };
    }

    pub fn find(self, id: i32) -> Result<models::Todo, diesel::result::Error> {
        let result: Result<models::Todo, diesel::result::Error> = schema::todos::dsl::todos
            .find(id)
            .select(models::Todo::as_select())
            .first(&mut self.app_state.db_pool.get_connection());

        result
    }

    pub fn create(self, title: &str, content: &str) -> Result<models::Todo, diesel::result::Error> {
        let new_todo: models::NewTodo<'_> = models::NewTodo { title, content };
        let result: Result<models::Todo, diesel::result::Error> = diesel::insert_into(schema::todos::table)
            .values(&new_todo)
            .returning(models::Todo::as_returning())
            .get_result(&mut self.app_state.db_pool.get_connection());
        result
    }

    pub fn list(self) -> Result<Vec<models::Todo>, diesel::result::Error> {
        let result: Result<Vec<models::Todo>, diesel::result::Error> = schema::todos::dsl::todos
            .select(models::Todo::as_select())
            .load(&mut self.app_state.db_pool.get_connection());

        result
    }
}
