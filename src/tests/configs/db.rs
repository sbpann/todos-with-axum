#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn new_pg_pool() {
        use sqlx::Connection;
        use crate::configs::db::*;

        let database_url = "postgres://my_user:my_password@localhost/my_database";

        let pg_pool = new_pg_pool(&database_url).await;
        match pg_pool.acquire().await {
            Err(error) => panic!("{}", error),
            Ok(mut conn) => match conn.ping().await {
                Err(error) => panic!("{}", error),
                Ok(_) => {}
            },
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn new_pg_pool_panic() {
        crate::configs::db::new_pg_pool("not-url").await;
    }
}
