#[derive(sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub content: String,
    #[sqlx(default)]
    pub total: Option<i64>
}