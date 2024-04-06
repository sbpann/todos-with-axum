use std::sync::Arc;

use sqlx::{postgres::PgQueryResult, Postgres};

use super::models;
use crate::ApplicationState;

pub struct TodoService {
    db_pool: sqlx::Pool<Postgres>,
}
impl TodoService {
    pub fn new(state: Arc<ApplicationState>) -> Self {
        return Self {
            db_pool: state.db_pool.clone(),
        };
    }

    pub async fn find(&self, id: i32) -> Result<models::Todo, sqlx::Error> {
        let query_result = sqlx::query_as::<_, models::Todo>("SELECT * FROM todos WHERE id = $1;")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await;

        query_result
    }

    pub async fn create(&self, title: &str, content: &str) -> Result<models::Todo, sqlx::Error> {
        let query_result = sqlx::query_as::<_, models::Todo>(
            "INSERT INTO todos (title, content) VALUES ($1, $2) RETURNING *;",
        )
        .bind(title)
        .bind(content)
        .fetch_one(&self.db_pool)
        .await;

        query_result
    }

    pub async fn list(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<models::Todo>, sqlx::Error> {

        let query_result = 
        sqlx::query_as::<_, models::Todo>(
            "SELECT * , COUNT(*) OVER () AS total FROM todos LIMIT $1 OFFSET $2;")
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.db_pool)
            .await;
        query_result
    }

    pub async fn update(
        &self,
        id: i32,
        title: &str,
        content: &str,
    ) -> Result<models::Todo, sqlx::Error> {
        let query_result = sqlx::query_as::<_, models::Todo>(
            "UPDATE todos SET title = $1, content = $2 WHERE id = $3 RETURNING * ;",
        )
        .bind(title)
        .bind(content)
        .bind(id)
        .fetch_one(&self.db_pool)
        .await;

        query_result
    }

    pub async fn delete(&self, id: i32) -> Result<PgQueryResult, sqlx::Error> {
        let query_result = sqlx::query(
            "DELETE FROM todos WHERE id = $1;",
        )
        .bind(id)
        .execute(&self.db_pool)
        .await;

        query_result
    }

}
