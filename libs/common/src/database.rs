use std::sync::Arc;

pub mod postgre {

    use super::*;
    use prisma::PrismaClient;

    pub mod prisma {
        pub use crate::prisma::*;
    }

    pub type DbConnectionRef = Arc<PrismaClient>;

    pub async fn init_prisma_client() -> DbConnectionRef {
        let client = PrismaClient::_builder()
            .build()
            .await
            .unwrap_or_else(|err| panic!("Failed to connect to database: {}", err));

        Arc::new(client)
    }
}

pub mod mongodb {}
