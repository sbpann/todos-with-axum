
use serde::Serialize;

#[derive(Serialize)]
pub struct Pagination<T> {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
    pub items: Vec<T>
}