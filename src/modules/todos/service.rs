use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use super::models;
use crate::{configs::db::PgDbPool, schema, ApplicationState};

pub struct TodoService {
    db_pool: PgDbPool,
}
impl TodoService {
    pub fn new(state: Arc<ApplicationState>) -> Self {
        return Self {
            db_pool: state.db_pool.clone(),
        };
    }

    pub fn find(&self, id: i32) -> Result<models::Todo, diesel::result::Error> {
        let result: Result<models::Todo, diesel::result::Error> = schema::todos::dsl::todos
            .find(id)
            .select(models::Todo::as_select())
            .first(&mut self.new_pooled_conn());

        result
    }

    pub fn create(
        &self,
        title: &str,
        content: &str,
    ) -> Result<models::Todo, diesel::result::Error> {
        let new_todo: models::NewTodo<'_> = models::NewTodo { title, content };
        let result: Result<models::Todo, diesel::result::Error> =
            diesel::insert_into(schema::todos::table)
                .values(&new_todo)
                .returning(models::Todo::as_returning())
                .get_result(&mut self.new_pooled_conn());
        result
    }

    pub fn list(&self) -> Result<Vec<models::Todo>, diesel::result::Error> {
        let result: Result<Vec<models::Todo>, diesel::result::Error> = schema::todos::dsl::todos
            .select(models::Todo::as_select())
            .load(&mut self.new_pooled_conn());

        result
    }

    pub fn update(
        &self,
        id: i32,
        title: &str,
        content: &str,
    ) -> Result<models::Todo, diesel::result::Error> {
        let result = diesel::update(schema::todos::dsl::todos.find(id))
            .set((schema::todos::dsl::content.eq(content), schema::todos::dsl::title.eq(title)))
            .returning(models::Todo::as_returning())
            .get_result(&mut self.new_pooled_conn());
        result
    }

    fn new_pooled_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.db_pool.new_connection()
    }
}
