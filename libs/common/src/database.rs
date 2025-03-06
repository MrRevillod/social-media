use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;

pub type PostgresPool = Pool<Postgres>;
pub type PgPoolRef = Arc<PostgresPool>;

pub struct PostgresClient;

impl PostgresClient {
    pub async fn new(url: &String) -> PgPoolRef {
        let pool = PgPool::connect(url).await.unwrap_or_else(|err| {
            panic!("Failed to connect to database: {}", err);
        });

        Arc::new(pool)
    }
}
