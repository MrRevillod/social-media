use std::sync::Arc;

pub mod postgre {

    use sqlx::{PgPool, Pool, Postgres};

    use super::*;
    use crate::{constants::POSTGRES_DATABASE_URL, prisma::PrismaClient};

    pub type DbConnectionRef = Arc<PrismaClient>;

    pub async fn init_prisma_client() -> DbConnectionRef {
        let client = PrismaClient::_builder()
            .build()
            .await
            .unwrap_or_else(|err| panic!("Failed to connect to database: {}", err));

        Arc::new(client)
    }

    pub async fn get_db_connection() -> Arc<Pool<Postgres>> {
        let pool = PgPool::connect(&POSTGRES_DATABASE_URL)
            .await
            .unwrap_or_else(|err| {
                panic!("Failed to connect to database: {}", err);
            });

        Arc::new(pool)
    }
}

pub mod mongodb {}
