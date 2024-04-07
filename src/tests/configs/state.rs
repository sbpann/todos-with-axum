#[cfg(test)]

mod tests {
    use crate::configs::state::*;

    #[tokio::test]
    #[should_panic]
    async fn build_state_database_err_url_unset() {
        build_state().await;
    }

    #[tokio::test]
    async fn build_state_ok() {
        let build_state_wrapper = async {
            let state = build_state().await;
            let connection_option = state.db_pool.connect_options();
            let database = connection_option.get_database();
            assert!(!database.is_none());
            assert_eq!(database.unwrap(), "my_database");
        };
        temp_env::async_with_vars(
            [(
                "DATABASE_URL",
                Some("postgres://my_user:my_password@localhost/my_database"),
            )],
            build_state_wrapper,
        )
        .await;
    }
}
