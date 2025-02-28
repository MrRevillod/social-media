use crate::constants::POSTGRES_DATABASE_URL;

use sqlx::{PgPool, Pool, Postgres};
use std::sync::Arc;

pub type PostgresPool = Pool<Postgres>;
pub type PgPoolRef = Arc<PostgresPool>;

pub struct PostgresClient;

impl PostgresClient {
    pub async fn new() -> PgPoolRef {
        let pool = PgPool::connect(&POSTGRES_DATABASE_URL)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to connect to database: {}", err);
            });

        Arc::new(pool)
    }
}
