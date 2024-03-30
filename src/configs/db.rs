use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::PgConnection;
use std::env;

#[derive(Clone)]
pub struct PgDbPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PgDbPool {
    fn build_database_url() -> String {
        let error_message = "Cannot inititalize database config";
        let user = env::var("DATABASE_USER")
            .unwrap_or_else(|err| panic!("{}: {}", error_message, err));
        let password = env::var("DATABASE_PASSWORD")
            .unwrap_or_else(|err| panic!("{}: {}", error_message, err));
        let name =
            env::var("DATABASE_NAME").unwrap_or_else(|err| panic!("{}: {}", error_message, err));
        let host =
            env::var("DATABASE_HOST").unwrap_or_else(|err| panic!("{}: {}", error_message, err));
        let port =
            env::var("DATABASE_PORT").unwrap_or_else(|err| panic!("{}: {}", error_message, err));

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            user, password, host, port, name
        );

        database_url
    }
    pub fn new() -> Self {
        let manager = ConnectionManager::<PgConnection>::new(Self::build_database_url());

        Self {
            pool: Pool::builder()
                .test_on_check_out(true)
                .build(manager)
                .unwrap_or_else(|error| panic!("Could not build connection pool: {}", error)),
        }
    }

    pub fn get_connection(self) -> PooledConnection<ConnectionManager<PgConnection>>{
        match self.pool.get() {
            Ok(conn) => conn,
            Err(err) => panic!("Error getting connection from pool: {}", err)
        } 
    }
}
