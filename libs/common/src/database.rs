use std::sync::Arc;

pub mod postgres {

    use sqlx::{PgPool, Pool, Postgres};

    use super::*;
    use crate::constants::POSTGRES_DATABASE_URL;

    pub type PostgresPool = Pool<Postgres>;
    pub type PgPoolRef = Arc<PostgresPool>;

    pub async fn get_db_connection() -> Arc<Pool<Postgres>> {
        let pool = PgPool::connect(&POSTGRES_DATABASE_URL)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to connect to database: {}", err);
            });

        Arc::new(pool)
    }

    // pub trait PostgreSave<T> {
    //     async fn save(&self, pool: PgPoolRef) -> Result<T, sqlx::Error>;
    // }
}

pub mod mongodb {}
