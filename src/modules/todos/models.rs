#[derive(Debug, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
    #[sqlx(default)]
    pub total: i64
}